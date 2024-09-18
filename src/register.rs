#[allow(non_camel_case_types)]

pub(crate) trait Register {
    fn addr(&self) -> u8;
    fn read_only(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Bank0 {
    FIRMWARE_ID = 0x00,
    FIRMWARE_VERSION = 0x01,
    STATUS1 = 0x02,
    SYS_CONFIG1 = 0x03,
    FEED_CONFIG1 = 0x04,
    FEED_CONFIG2 = 0x05,
    CAL_CONFIG1 = 0x07,
    PS_2_AUX_CONTROL = 0x08,
    SAMPLE_RATE = 0x09,
    Z_IDLE = 0xa,
    Z_SCALER = 0xb,
    SLEEP_INTERVAL = 0xc,
    SLEEP_TIMER = 0xd,
    PACKET_BYTE0 = 0x12,
    PACKET_BYTE1 = 0x13,
    PACKET_BYTE2 = 0x14,
    PACKET_BYTE3 = 0x15,
    PACKET_BYTE4 = 0x16,
    PACKET_BYTE5 = 0x17,
}
impl Register for Bank0 {
    fn addr(&self) -> u8 {
        *self as u8
    }
    fn read_only(&self) -> bool {
        false
    }
}
