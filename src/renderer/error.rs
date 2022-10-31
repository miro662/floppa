use std::fmt::Formatter;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    RequestDeviceError(wgpu::RequestDeviceError),
    NoDevice,
    SurfaceError(wgpu::SurfaceError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            RequestDeviceError(err) => write!(f, "Request device error: {}", err),
            SurfaceError(err) => write!(f, "Surface error: {}", err),
            NoDevice => write!(f, "Device not found"),
        }
    }
}

impl error::Error for Error {}

impl From<wgpu::RequestDeviceError> for Error {
    fn from(error: wgpu::RequestDeviceError) -> Self {
        Error::RequestDeviceError(error)
    }
}
impl From<wgpu::SurfaceError> for Error {
    fn from(error: wgpu::SurfaceError) -> Self {
        Error::SurfaceError(error)
    }
}
