#![no_std]

use core::fmt::Debug;

use crate::register::Bank0;
use config::PowerMode;
use embedded_hal::{delay::DelayNs, i2c::I2c};

pub use crate::config::Address;
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

impl<I2C, E> Tm040040<I2C>
where
    I2C: I2c<Error = E>,
    E: Debug,
{
    pub fn new(i2c: I2C, address: Address) -> Result<Self, Error<E>> {
        let mut me = Self { i2c, address };

        //TODO: verify device id
        me.set_power_mode(PowerMode::default())?;
        Ok(me)
    }

    pub fn free(self) -> I2C {
        self.i2c
    }

    pub fn set_power_mode(&mut self, power_mode: PowerMode) -> Result<(), Error<E>> {
        self.write_reg(&Bank0::SYS_CONFIG1, power_mode as u8)
    }

    fn read_reg<R: Register>(&mut self, reg: &R) -> Result<u8, Error<E>> {
        let mut buffer = [0u8];
        self.i2c
            .write_read(self.address as u8, &[reg.addr()], &mut buffer)
            .map_err(|e| Error::BusError(e))?;
        Ok(buffer[0])
    }
    fn write_reg<R: Register>(&mut self, reg: &R, value: u8) -> Result<(), Error<E>> {
        if reg.read_only() {
            Err(Error::SensorError(error::SensorError::WriteToReadOnly))
        } else {
            self.i2c
                .write(self.address as u8, &[reg.addr(), value])
                .map_err(|e| Error::BusError(e))
        }
    }
}
