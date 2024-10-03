use crate::register::{Bank0, Register};

pub(crate) trait Bitfield {
    const BITMASK: u8;
    type Reg: Register;
    const REGISTER: Self::Reg;

    fn bits(self) -> u8;
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Address {
    #[default]
    Primary = 0x2a,
    Secondary = 0x2c,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PowerMode {
    Shutdown = 1,
    Sleep = 2,
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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FeedMode {
    #[default]
    Enabled = 1,
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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PositionMode {
    #[default]
    Relative = 0,
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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum TapMode {
    #[default]
    Enabled = 0,
    AllTapsDisable = 1,
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
