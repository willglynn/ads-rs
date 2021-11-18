//! Wrapper for symbol handles.

use crate::{Device, Result};
use crate::index;

/// A handle to a variable within the ADS device.
pub struct Handle<'c> {
    device: Device<'c>,
    handle: u32,
}

impl<'c> Handle<'c> {
    /// Create a new handle to a single symbol.
    pub fn new(device: Device<'c>, symbol: &str) -> Result<Self> {
        let mut handle_bytes = [0; 4];
        device.write_read(index::SYS_GET_SYMHANDLE_BYNAME, 0, symbol.as_bytes(),
                          &mut handle_bytes)?;
        Ok(Self { device, handle: u32::from_le_bytes(handle_bytes) })
    }

    /// Read data from the variable.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.device.read(index::SYS_RW_SYMVAL_BYHANDLE, self.handle, buf)
    }

    /// Write data to the variable.
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.device.write(index::SYS_RW_SYMVAL_BYHANDLE, self.handle, buf)
    }
}

impl<'a> Drop for Handle<'a> {
    fn drop(&mut self) {
        let _ = self.device.write(index::SYS_RELEASE_SYMHANDLE, 0,
                                  &self.handle.to_le_bytes());
    }
}
