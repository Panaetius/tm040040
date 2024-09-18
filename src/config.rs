#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Address {
    Primary = 0x2a,
    Secondary = 0x2c,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    Shutdown = 0x02,
    Sleep = 0x04,
    Normal = 0x00,
}

impl Default for PowerMode {
    fn default() -> Self {
        PowerMode::Normal
    }
}
