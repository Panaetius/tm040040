//! An [embedded-hal] driver for the TM040040 Pinnacle touch pads from Cirque.
//!
//! The Pinnacle touch pad supports X and Y axis movement, tap detection and other features.
//! Note that while the touch pad supports both I²C and SPI, only I²C is supported in this driver.
//! For I²C to be active, the R1 resistor needs to be removed from the touch pad, if there is one.
//! This was only tested with the TM040040 touch pad,but should work with all Pinnacle touch pads.
//! This library only supports the non-AG (Advanced Gestures) version of Pinnacle touch pads.
//!
//! For additional information, please consult the [datasheet] as well as the [Pinnacle ASIC documentation].
//!
//! # Example
//!
//! ```rust,no_run
//! use esp_idf_hal::{
//!     i2c::{I2cConfig, I2cDriver},
//!     peripherals::Peripherals,
//! };
//! use anyhow::Result;
//!
//! use tm040040::{Address, FeedMode, Tm040040, XYInverted};
//! fn main() -> Result<()> {
//!     esp_idf_svc::sys::link_patches();
//!     let peripherals = Peripherals::take().unwrap();
//!
//!     let sda = peripherals.pins.gpio10;
//!     let scl = peripherals.pins.gpio8;
//!     let config = I2cConfig::new().baudrate(400.kHz().into());
//!     let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;
//!     let mut trackpad = Tm040040::new(i2c, Address::Primary).enable().unwrap();
//!     let pad_data = trackpad.relative_data().unwrap();
//!     if let Some(touch_data) = pad_data {
//!         // the above is only `Some` if the pad is currently touched, otherwise it's `None`.
//!         // Do something with the touch data
//!     }
//! }
//! ```
//!
//! [embedded-hal]: https://docs.rs/embedded-hal/latest/embedded_hal/
//! [datasheet]: https://eu.mouser.com/datasheet/2/892/TM040040_SPI-I2C-PINNTrackpad_SPEC1-2-1223705.pdf
//! [Pinnacle ASIC documentation]: https://static1.squarespace.com/static/53233e4be4b044fa7626c453/t/599de7856f4ca3c38aa74632/1503520647200/gt-an-090620_2-4_interfacingtopinnacle_i2c-spi_docver1-6.pdf

#![no_std]

use core::{fmt::Debug, marker::PhantomData};

use config::{Bitfield, Mask};
use embedded_hal::{
    digital::{self, InputPin},
    i2c::I2c,
};

use crate::register::{Bank0, Register};
pub use crate::{
    config::{
        Address, FeedMode, FilterMode, GlideExtendMode, IntelliMouseMode, PositionMode, PowerMode,
        ScrollMode, TapMode, XYEnable, XYInverted, XYSwapped,
    },
    error::Error,
};

mod config;
mod error;
mod register;

mod private {

    pub trait Sealed {}
}

const PINNACLE_X_LOWER: u16 = 128;
const PINNACLE_Y_LOWER: u16 = 64;
const PINNACLE_X_UPPER: u16 = 1920;
const PINNACLE_Y_UPPER: u16 = 1472;

/// Position and button data in relative mode
#[derive(Debug, Clone, Copy)]
pub struct RelativeData {
    /// Whether the primary button is pressed (tap)
    pub primary_pressed: bool,
    /// Whether the secondary button is pressed (tap in upper left corner)
    pub secondary_pressed: bool,
    /// Whether the auxilliary button is pressed (not documented what this is?)
    pub aux_pressed: bool,
    /// The relative delta in the X dimension
    pub x_delta: i16,
    /// The relative delta in the Y dimension
    pub y_delta: i16,
}

/// Position and button data in absolute mode
#[derive(Debug, Clone, Copy)]
pub struct AbsoluteData {
    /// The current button state encoded as bits (lowest 6 bits are used)
    pub button_state: u8,
    /// Absolute position in X dimension, scaled accrding to dead zones
    pub x_pos: u16,

    /// Absolute position in X dimension, scaled accrding to dead zones
    pub y_pos: u16,
    /// Z-level (0 when no finger is close, increases as finger approaches)
    pub z_level: u8,
}

pub trait FeedState: private::Sealed {}
pub struct FeedEnabled;
pub struct NoFeed;
impl FeedState for FeedEnabled {}
impl private::Sealed for FeedEnabled {}
impl FeedState for NoFeed {}
impl private::Sealed for NoFeed {}

pub trait PositionReportingMode: private::Sealed {}
pub struct Relative;
pub struct Absolute;
impl PositionReportingMode for Relative {}
impl private::Sealed for Relative {}
impl PositionReportingMode for Absolute {}
impl private::Sealed for Absolute {}

pub struct Tm040040<'a, I2C, PositionMode: PositionReportingMode, Feed: FeedState, E> {
    i2c: I2C,
    address: Address,
    hardware_data_ready: &'a mut dyn InputPin<Error = E>,
    _pos_state: PhantomData<PositionMode>,
    _feed_state: PhantomData<Feed>,
}

impl<I2C, E, PosMode, Feed, PinError> Tm040040<'_, I2C, PosMode, Feed, PinError>
where
    I2C: I2c<Error = E>,
    E: Debug,
    PosMode: PositionReportingMode,
    Feed: FeedState,
    PinError: digital::Error,
{
    /// Return the underlying I2C instance for reuse
    pub fn free(self) -> I2C {
        self.i2c
    }

    /// Get the device/firmware ID of the touchpad
    pub fn device_id(&mut self) -> Result<u8, Error<E, PinError>> {
        self.read_reg(&Bank0::FIRMWARE_ID)
    }

    /// Get the currently configured power mode
    pub fn power_mode(&mut self) -> Result<PowerMode, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::SYS_CONFIG1)? >> 1;
        let mode = PowerMode::try_from(bits)?;

        Ok(mode)
    }

    /// Set the power mode
    pub fn set_power_mode(&mut self, power_mode: PowerMode) -> Result<(), Error<E, PinError>> {
        self.update_reg(power_mode)
    }

    /// Get the current feed mode
    pub fn feed_mode(&mut self) -> Result<FeedMode, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & FeedMode::BITMASK;
        let mode = FeedMode::try_from(bits)?;

        Ok(mode)
    }

    /// Set the feed mode, enabling or disabling position reporting
    fn set_feed_mode(&mut self, fd: FeedMode) -> Result<(), Error<E, PinError>> {
        self.update_reg(fd)
    }

    /// Get the current position reporting mode
    pub fn position_mode(&mut self) -> Result<PositionMode, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & PositionMode::BITMASK;
        let mode = PositionMode::try_from(bits)?;

        Ok(mode)
    }

    /// Set the current position reporting mode (Absolute or Relative coordinates)
    fn set_position_mode(&mut self, pos: PositionMode) -> Result<(), Error<E, PinError>> {
        self.update_reg(pos)
    }

    /// Get the current filter mode
    pub fn filter_mode(&mut self) -> Result<FilterMode, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & FilterMode::BITMASK;
        let mode = FilterMode::try_from(bits)?;

        Ok(mode)
    }

    ///Set the hardware filter mode
    pub fn set_filter_mode(&mut self, filter: FilterMode) -> Result<(), Error<E, PinError>> {
        self.update_reg(filter)
    }

    /// Get enabled axis
    pub fn xy_enable(&mut self) -> Result<XYEnable, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & XYEnable::BITMASK;
        let mode = XYEnable::try_from(bits)?;

        Ok(mode)
    }

    /// Set enabled axis
    pub fn set_xy_enable(&mut self, yx: XYEnable) -> Result<(), Error<E, PinError>> {
        self.update_reg(yx)
    }

    /// Get axis inversion setting
    pub fn xy_inverted(&mut self) -> Result<XYInverted, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & XYInverted::BITMASK;
        let mode = XYInverted::try_from(bits)?;

        Ok(mode)
    }

    /// Invert axis
    pub fn set_xy_inverted(&mut self, yx: XYInverted) -> Result<(), Error<E, PinError>> {
        self.update_reg(yx)
    }

    /// Read the value of a register
    fn read_reg<R: Register>(&mut self, reg: &R) -> Result<u8, Error<E, PinError>> {
        let mut buffer = [0u8];

        self.i2c
            .write_read(
                self.address as u8,
                &[reg.addr() | Mask::Read as u8],
                &mut buffer,
            )
            .map_err(|e| Error::BusError(e))?;

        Ok(buffer[0])
    }

    /// Write a value to a register
    fn write_reg<R: Register>(&mut self, reg: &R, value: u8) -> Result<(), Error<E, PinError>> {
        if reg.read_only() {
            Err(Error::SensorError(error::SensorError::WriteToReadOnly))
        } else {
            self.i2c
                .write(self.address as u8, &[reg.addr() | Mask::Write as u8, value])
                .map_err(|e| Error::BusError(e))
        }
    }

    /// Update specific bits of a register
    fn update_reg<BF: Bitfield>(&mut self, value: BF) -> Result<(), Error<E, PinError>> {
        if BF::REGISTER.read_only() {
            Err(Error::SensorError(error::SensorError::WriteToReadOnly))
        } else {
            let current = self.read_reg(&BF::REGISTER)?;
            let value = (current & !BF::BITMASK) | (value.bits() & BF::BITMASK);
            self.write_reg(&BF::REGISTER, value)
        }
    }

    /// Clears the status flags.
    /// This needs to be called after reading a position, otherwise no new position data is reported
    fn clear_flags(&mut self) -> Result<(), Error<E, PinError>> {
        self.write_reg(&Bank0::STATUS1, 0x00)
    }
}
impl<'a, I2C, E, PinError> Tm040040<'a, I2C, Relative, NoFeed, PinError>
where
    I2C: I2c<Error = E>,
    E: Debug,
    PinError: digital::Error,
{
    //! Create a new trackpad instance.
    pub fn new(
        i2c: I2C,
        address: Address,
        hardware_data_ready: &'a mut impl InputPin<Error = PinError>,
    ) -> Tm040040<'a, I2C, Relative, NoFeed, PinError> {
        Tm040040::<'a, I2C, Relative, NoFeed, PinError> {
            i2c,
            address,
            hardware_data_ready,
            _pos_state: PhantomData,
            _feed_state: PhantomData,
        }
    }
}

impl<'a, I2C, E, PinError> Tm040040<'a, I2C, Relative, FeedEnabled, PinError>
where
    I2C: I2c<Error = E>,
    E: Debug,
    PinError: digital::Error,
{
    /// Read touchpad output as relative data (delta X and Y) plus button presses
    /// `None` if the touchpad isn't being touched.
    pub fn relative_data(&mut self) -> Result<Option<RelativeData>, Error<E, PinError>> {
        let hw_dr = self.hardware_data_ready.is_high()?;
        if !hw_dr {
            return Ok(None);
        }
        let sw_dr = self.read_reg(&Bank0::STATUS1)? & 0b0000_0100;

        if sw_dr == 0 {
            return Ok(None);
        }

        let pb0 = self.read_reg(&Bank0::PACKET_BYTE0)?;
        let pb1 = self.read_reg(&Bank0::PACKET_BYTE1)?;
        let pb2 = self.read_reg(&Bank0::PACKET_BYTE2)?;

        self.clear_flags()?;

        let primary_pressed = (pb0 & 0x1) != 0;
        let secondary_pressed = (pb0 & 0x2) != 0;
        let aux_pressed = (pb0 & 0x4) != 0;
        let x_sign = pb0 & 0b0001_0000;
        let y_sign = pb0 & 0b0010_0000;

        let x_delta = if x_sign == 0 {
            pb1 as i16
        } else {
            (pb1 as i16) - 256
        };

        let y_delta = if y_sign == 0 {
            pb2 as i16
        } else {
            (pb2 as i16) - 256
        };

        Ok(Some(RelativeData {
            primary_pressed,
            secondary_pressed,
            aux_pressed,
            x_delta,
            y_delta,
        }))
    }

    /// Switch to absolute position mode
    pub fn absolute(
        mut self,
    ) -> Result<Tm040040<'a, I2C, Absolute, FeedEnabled, PinError>, Error<E, PinError>> {
        self.set_position_mode(PositionMode::Absolute)?;

        Ok(Tm040040 {
            i2c: self.i2c,
            address: self.address,
            hardware_data_ready: self.hardware_data_ready,
            _pos_state: PhantomData,
            _feed_state: PhantomData,
        })
    }
}

impl<I2C, E, Feed, PinError> Tm040040<'_, I2C, Relative, Feed, PinError>
where
    I2C: I2c<Error = E>,
    E: Debug,
    Feed: FeedState,
    PinError: digital::Error,
{
    /// Get axis swap state
    pub fn xy_swapped(&mut self) -> Result<XYSwapped, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & XYSwapped::BITMASK;
        let mode = XYSwapped::try_from(bits)?;

        Ok(mode)
    }

    /// Swap X/Y axis
    pub fn set_xy_swapped(&mut self, yx: XYSwapped) -> Result<(), Error<E, PinError>> {
        self.update_reg(yx)
    }

    /// Get Intelli mouse config
    pub fn intelli_mouse(&mut self) -> Result<IntelliMouseMode, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & IntelliMouseMode::BITMASK;
        let mode = IntelliMouseMode::try_from(bits)?;

        Ok(mode)
    }

    /// Set Intelli Mouse setting
    /// When enabled, reports back scroll position in relative mode (if supported)
    pub fn set_intelli_mouse(&mut self, im: IntelliMouseMode) -> Result<(), Error<E, PinError>> {
        self.update_reg(im)
    }

    /// Get tap detection mode
    pub fn tap_mode(&mut self) -> Result<TapMode, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & TapMode::BITMASK;
        let mode = TapMode::try_from(bits)?;

        Ok(mode)
    }

    /// Set tap detection mode
    pub fn set_tap_mode(&mut self, tm: TapMode) -> Result<(), Error<E, PinError>> {
        self.update_reg(tm)
    }

    /// Get scroll mode
    pub fn scroll_mode(&mut self) -> Result<ScrollMode, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & ScrollMode::BITMASK;
        let mode = ScrollMode::try_from(bits)?;

        Ok(mode)
    }

    /// Enable/disable scroll data
    pub fn set_scroll_mode(&mut self, sm: ScrollMode) -> Result<(), Error<E, PinError>> {
        self.update_reg(sm)
    }

    /// Get Glide extend config
    pub fn glide_extend_mode(&mut self) -> Result<GlideExtendMode, Error<E, PinError>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & GlideExtendMode::BITMASK;
        let mode = GlideExtendMode::try_from(bits)?;

        Ok(mode)
    }

    /// Set Glide extend config
    /// This allows continuing drag operations when the edge is reached by lifting and repositioning the finger
    pub fn set_glide_extend_mode(
        &mut self,
        gem: GlideExtendMode,
    ) -> Result<(), Error<E, PinError>> {
        self.update_reg(gem)
    }
}

impl<'a, I2C, E, PinError> Tm040040<'a, I2C, Absolute, FeedEnabled, PinError>
where
    I2C: I2c<Error = E>,
    E: Debug,
    PinError: digital::Error,
{
    /// Read touchpad output (X/Y/Z position and button presses) in absolute mode
    /// Output is clipped to min/max usable position on the trackpad
    pub fn absolute_data(&mut self) -> Result<Option<AbsoluteData>, Error<E, PinError>> {
        let hw_dr = self.hardware_data_ready.is_high()?;
        if !hw_dr {
            return Ok(None);
        }
        let button_state = self.read_reg(&Bank0::PACKET_BYTE0)? & 0x3F;
        let x_low = self.read_reg(&Bank0::PACKET_BYTE2)?;
        let y_low = self.read_reg(&Bank0::PACKET_BYTE3)?;
        let x_y_high = self.read_reg(&Bank0::PACKET_BYTE4)?;
        let z_level = self.read_reg(&Bank0::PACKET_BYTE5)? & 0x3F;
        let x_pos = x_low as u16 | (((x_y_high & 0x0F) as u16) << 8);
        let y_pos = y_low as u16 | (((x_y_high & 0xF0) as u16) << 4);

        self.clear_flags()?;

        Ok(Some(AbsoluteData {
            button_state,
            x_pos: x_pos.max(PINNACLE_X_UPPER).min(PINNACLE_X_LOWER),
            y_pos: y_pos.max(PINNACLE_Y_UPPER).min(PINNACLE_Y_LOWER),
            z_level,
        }))
    }

    /// Switch to relative position mode
    pub fn relative(
        mut self,
    ) -> Result<Tm040040<'a, I2C, Relative, FeedEnabled, PinError>, Error<E, PinError>> {
        self.set_position_mode(PositionMode::Relative)?;

        Ok(Tm040040 {
            i2c: self.i2c,
            address: self.address,
            hardware_data_ready: self.hardware_data_ready,
            _pos_state: PhantomData,
            _feed_state: PhantomData,
        })
    }
}

impl<'a, I2C, E, PosMode, PinError> Tm040040<'a, I2C, PosMode, FeedEnabled, PinError>
where
    I2C: I2c<Error = E>,
    E: Debug,
    PosMode: PositionReportingMode,
    PinError: digital::Error,
{
    /// Disable feed, no new data will be collected from sensor
    pub fn disable(
        mut self,
    ) -> Result<Tm040040<'a, I2C, PosMode, NoFeed, PinError>, Error<E, PinError>> {
        self.set_feed_mode(FeedMode::NoFeed)?;

        Ok(Tm040040 {
            i2c: self.i2c,
            address: self.address,
            hardware_data_ready: self.hardware_data_ready,
            _pos_state: PhantomData,
            _feed_state: PhantomData,
        })
    }
}

impl<'a, I2C, E, PosMode, PinError> Tm040040<'a, I2C, PosMode, NoFeed, PinError>
where
    I2C: I2c<Error = E>,
    E: Debug,
    PosMode: PositionReportingMode,
    PinError: digital::Error,
{
    /// enable feed, sensor starts collecting data
    pub fn enable(
        mut self,
    ) -> Result<Tm040040<'a, I2C, PosMode, FeedEnabled, PinError>, Error<E, PinError>> {
        self.set_feed_mode(FeedMode::Enabled)?;
        self.clear_flags()?;

        Ok(Tm040040 {
            i2c: self.i2c,
            address: self.address,
            hardware_data_ready: self.hardware_data_ready,
            _pos_state: PhantomData,
            _feed_state: PhantomData,
        })
    }
}
