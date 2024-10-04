#![no_std]

use core::fmt::Debug;

use crate::register::Bank0;
use config::{Bitfield, Mask};
use embedded_hal::i2c::I2c;

pub use crate::config::Address;
pub use crate::config::{
    FeedMode, FilterMode, GlideExtendMode, IntelliMouseMode, PositionMode, PowerMode, ScrollMode,
    TapMode, XYEnable, XYInverted, XYSwapped,
};
pub use crate::error::Error;
use crate::register::Register;

mod config;
mod error;
mod register;

#[derive(Debug, Clone, Copy)]
pub struct Tm040040<I2C> {
    i2c: I2C,
    address: Address,
}

#[derive(Debug, Clone, Copy)]
pub struct RelativeData {
    pub primary_pressed: bool,
    pub secondary_pressed: bool,
    pub aux_pressed: bool,
    pub x_delta: i16,
    pub y_delta: i16,
}

impl<I2C, E> Tm040040<I2C>
where
    I2C: I2c<Error = E>,
    E: Debug,
{
    pub fn new(i2c: I2C, address: Address) -> Result<Self, Error<E>> {
        let mut me = Self { i2c, address };

        //TODO: verify device id
        me.set_power_mode(PowerMode::default())?;
        me.clear_flags()?;
        Ok(me)
    }

    pub fn free(self) -> I2C {
        self.i2c
    }

    pub fn device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_reg(&Bank0::FIRMWARE_ID)
    }

    pub fn power_mode(&mut self) -> Result<PowerMode, Error<E>> {
        let bits = self.read_reg(&Bank0::SYS_CONFIG1)? >> 1;
        let mode = PowerMode::try_from(bits)?;
        Ok(mode)
    }

    pub fn set_power_mode(&mut self, power_mode: PowerMode) -> Result<(), Error<E>> {
        self.write_reg(&Bank0::SYS_CONFIG1, power_mode as u8)
    }

    pub fn relative_data(&mut self) -> Result<Option<RelativeData>, Error<E>> {
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
    pub fn set_feed_mode(&mut self, fd: FeedMode) -> Result<(), Error<E>> {
        self.update_reg(fd)
    }
    pub fn position_mode(&mut self) -> Result<PositionMode, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & PositionMode::BITMASK;
        let mode = PositionMode::try_from(bits)?;
        Ok(mode)
    }

    pub fn set_position_mode(&mut self, pos: PositionMode) -> Result<(), Error<E>> {
        self.update_reg(pos)
    }
    pub fn filter_mode(&mut self) -> Result<FilterMode, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & FilterMode::BITMASK;
        let mode = FilterMode::try_from(bits)?;
        Ok(mode)
    }
    pub fn set_filter_mode(&mut self, filter: FilterMode) -> Result<(), Error<E>> {
        self.update_reg(filter)
    }
    pub fn xy_enable(&mut self) -> Result<XYEnable, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & XYEnable::BITMASK;
        let mode = XYEnable::try_from(bits)?;
        Ok(mode)
    }
    pub fn set_xy_enable(&mut self, yx: XYEnable) -> Result<(), Error<E>> {
        self.update_reg(yx)
    }
    pub fn xy_inverted(&mut self) -> Result<XYInverted, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & XYInverted::BITMASK;
        let mode = XYInverted::try_from(bits)?;
        Ok(mode)
    }
    pub fn set_xy_inverted(&mut self, yx: XYInverted) -> Result<(), Error<E>> {
        self.update_reg(yx)
    }
    pub fn xy_swapped(&mut self) -> Result<XYSwapped, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & XYSwapped::BITMASK;
        let mode = XYSwapped::try_from(bits)?;
        Ok(mode)
    }
    pub fn set_xy_swapped(&mut self, yx: XYSwapped) -> Result<(), Error<E>> {
        self.update_reg(yx)
    }
    pub fn intelli_mouse(&mut self) -> Result<IntelliMouseMode, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & IntelliMouseMode::BITMASK;
        let mode = IntelliMouseMode::try_from(bits)?;
        Ok(mode)
    }
    pub fn set_intelli_mouse(&mut self, im: IntelliMouseMode) -> Result<(), Error<E>> {
        self.update_reg(im)
    }
    pub fn tap_mode(&mut self) -> Result<TapMode, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & TapMode::BITMASK;
        let mode = TapMode::try_from(bits)?;
        Ok(mode)
    }
    pub fn set_tap_mode(&mut self, tm: TapMode) -> Result<(), Error<E>> {
        self.update_reg(tm)
    }
    pub fn scroll_mode(&mut self) -> Result<ScrollMode, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & ScrollMode::BITMASK;
        let mode = ScrollMode::try_from(bits)?;
        Ok(mode)
    }
    pub fn set_scroll_mode(&mut self, sm: ScrollMode) -> Result<(), Error<E>> {
        self.update_reg(sm)
    }
    pub fn glide_extend_mode(&mut self) -> Result<GlideExtendMode, Error<E>> {
        let bits = self.read_reg(&Bank0::FEED_CONFIG1)? & GlideExtendMode::BITMASK;
        let mode = GlideExtendMode::try_from(bits)?;
        Ok(mode)
    }
    pub fn set_glide_extend_mode(&mut self, gem: GlideExtendMode) -> Result<(), Error<E>> {
        self.update_reg(gem)
    }

    fn read_reg<R: Register>(&mut self, reg: &R) -> Result<u8, Error<E>> {
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
    fn write_reg<R: Register>(&mut self, reg: &R, value: u8) -> Result<(), Error<E>> {
        if reg.read_only() {
            Err(Error::SensorError(error::SensorError::WriteToReadOnly))
        } else {
            self.i2c
                .write(self.address as u8, &[reg.addr() | Mask::Write as u8, value])
                .map_err(|e| Error::BusError(e))
        }
    }
    fn update_reg<BF: Bitfield>(&mut self, value: BF) -> Result<(), Error<E>> {
        if BF::REGISTER.read_only() {
            Err(Error::SensorError(error::SensorError::WriteToReadOnly))
        } else {
            let current = self.read_reg(&BF::REGISTER)?;
            let value = (current & !BF::BITMASK) | (value.bits() & BF::BITMASK);
            self.write_reg(&BF::REGISTER, value)
        }
    }

    fn clear_flags(&mut self) -> Result<(), Error<E>> {
        self.write_reg(&Bank0::STATUS1, 0x00)
    }
}
