use std::fmt::Formatter;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    RequestDeviceError(wgpu::RequestDeviceError),
    NoDevice,
    SurfaceError(wgpu::SurfaceError),
    IOError(io::Error),
    TextureEncodingError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            RequestDeviceError(err) => write!(f, "Request device error: {}", err),
            SurfaceError(err) => write!(f, "Surface error: {}", err),
            NoDevice => write!(f, "Device not found"),
            IOError(err)=> write!(f, "IO error: {}", err),
            TextureEncodingError => write!(f, "Cannot encode texture")
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
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IOError(error)
    }
}
