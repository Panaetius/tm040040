use crate::{
    error::SensorError,
    register::{Bank0, Register},
};

pub(crate) trait Bitfield {
    const BITMASK: u8;
    type Reg: Register;
    const REGISTER: Self::Reg;

    fn bits(self) -> u8;
}

#[derive(Clone, Copy, Debug)]
pub enum Mask {
    Read = 0xA0,
    Write = 0x80,
}

/// i2c adress
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Address {
    #[default]
    Primary = 0x2a,
    Secondary = 0x2c,
}

/// Touchpad power modes
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PowerMode {
    /// Shutdown touchpad. Consumes very low power, does not track touch
    Shutdown = 1,
    /// Enable sleep mode. After 5 seconds of no touch, enter sleep mode. In sleep mode, only check for touch every 300ms. Only uses around 50µA of current.
    Sleep = 2,
    /// Normal operation, switches between active and idle mode depending on touch. In idle state, checks for touch every 10ms.
    #[default]
    Normal = 0,
}

impl Bitfield for PowerMode {
    const BITMASK: u8 = 0b0000_0110;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::SYS_CONFIG1;
    fn bits(self) -> u8 {
        (self as u8) << 1
    }
}
impl TryFrom<u8> for PowerMode {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Shutdown),
            2 => Ok(Self::Sleep),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Feed mode controls if position reporting is turned on or not.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FeedMode {
    /// Report finger tracking
    #[default]
    Enabled = 1,
    /// Disable finger tracking and reporting
    NoFeed = 0,
}
impl Bitfield for FeedMode {
    const BITMASK: u8 = 0b0000_0001;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG1;
    fn bits(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for FeedMode {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoFeed),
            1 => Ok(Self::Enabled),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Position reporting mode
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PositionMode {
    /// Relative mode reports position deltas. Relative mode also allows using internal tap detection, scroll detection and extended features (controlled by other flags).
    #[default]
    Relative = 0,
    /// Absolute position reporting mode. Position is x in range 0 - 2047 and y in 0 - 1535. Distance of finger to touchpad is reported as z level, with 0 being no
    /// finger detected, and values increasing as finger approaches. It is up to the caller to detect touches and taps.
    Absolute = 1,
}
impl Bitfield for PositionMode {
    const BITMASK: u8 = 0b0000_0010;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG1;
    fn bits(self) -> u8 {
        (self as u8) << 1
    }
}
impl TryFrom<u8> for PositionMode {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Relative),
            1 => Ok(Self::Absolute),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Enable or disable hardware filters. Cirque does not reccommend disabling filters.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FilterMode {
    #[default]
    Enable = 0,
    Disable = 1,
}
impl Bitfield for FilterMode {
    const BITMASK: u8 = 0b0000_0100;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG1;
    fn bits(self) -> u8 {
        (self as u8) << 2
    }
}
impl TryFrom<u8> for FilterMode {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Enable),
            1 => Ok(Self::Disable),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Disable specific axis.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum XYEnable {
    #[default]
    Enabled = 0,
    XDisabled = 1,
    YDisabled = 2,
    XYDisabled = 3,
}
impl Bitfield for XYEnable {
    const BITMASK: u8 = 0b0001_1000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG1;
    fn bits(self) -> u8 {
        (self as u8) << 3
    }
}
impl TryFrom<u8> for XYEnable {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Enabled),
            1 => Ok(Self::XDisabled),
            2 => Ok(Self::YDisabled),
            3 => Ok(Self::XYDisabled),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Invert axis reporting (flips sign).
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum XYInverted {
    #[default]
    Normal = 0,
    XInverted = 1,
    YInverted = 2,
    XYInverted = 3,
}
impl Bitfield for XYInverted {
    const BITMASK: u8 = 0b1100_0000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG1;
    fn bits(self) -> u8 {
        (self as u8) << 6
    }
}
impl TryFrom<u8> for XYInverted {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::XInverted),
            2 => Ok(Self::YInverted),
            3 => Ok(Self::XYInverted),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Intelli mouse mode controlls scroll reporting.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum IntelliMouseMode {
    #[default]
    Disabled = 0,
    Enabled = 1,
}
impl Bitfield for IntelliMouseMode {
    const BITMASK: u8 = 0b0000_0001;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG2;
    fn bits(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for IntelliMouseMode {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::Enabled),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Handle what types of taps are detected by hardware.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum TapMode {
    /// Detect all kinds of taps
    #[default]
    Enabled = 0,
    /// Don't detect taps
    AllTapsDisable = 1,
    /// Dont detect secondary button taps. Secondary taps are taps in the upper right corner of the touchpad
    SecondaryTapDisable = 2,
}
impl Bitfield for TapMode {
    const BITMASK: u8 = 0b0000_0010;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG2;
    fn bits(self) -> u8 {
        (self as u8) << 1
    }
}
impl TryFrom<u8> for TapMode {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Enabled),
            1 => Ok(Self::AllTapsDisable),
            2 => Ok(Self::SecondaryTapDisable),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Control scroll mode. Cirque docs don't say what this actually does.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ScrollMode {
    #[default]
    Enabled = 0,
    Disabled = 1,
}
impl Bitfield for ScrollMode {
    const BITMASK: u8 = 0b0000_1000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG2;
    fn bits(self) -> u8 {
        (self as u8) << 3
    }
}
impl TryFrom<u8> for ScrollMode {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Enabled),
            1 => Ok(Self::Disabled),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Control glide extend mode. In glide extend mode, drag actions can be extended by lifting the finger when an edge is reached and repositioning the finger.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum GlideExtendMode {
    #[default]
    Enabled = 0,
    Disabled = 1,
}
impl Bitfield for GlideExtendMode {
    const BITMASK: u8 = 0b0001_0000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG2;
    fn bits(self) -> u8 {
        (self as u8) << 4
    }
}
impl TryFrom<u8> for GlideExtendMode {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Enabled),
            1 => Ok(Self::Disabled),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Swap X and Y axis.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum XYSwapped {
    #[default]
    Normal = 0,
    Swapped = 1,
}
impl Bitfield for XYSwapped {
    const BITMASK: u8 = 0b1000_0000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::FEED_CONFIG1;
    fn bits(self) -> u8 {
        (self as u8) << 7
    }
}
impl TryFrom<u8> for XYSwapped {
    type Error = SensorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Swapped),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}
