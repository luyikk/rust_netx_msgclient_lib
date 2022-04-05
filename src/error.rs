use anyhow::Error;
use interoptopus::ffi_type;
use interoptopus::patterns::result::FFIError;
// The error FFI users should see
#[ffi_type(patterns(ffi_error))]
#[repr(C)]
#[derive(Debug)]
pub enum NetXFFIError {
    Ok = 0,
    NullPassed = 1,
    Panic = 2,
    AnyHowError = 3,
    NotConnect = 4,
}

// Gives special meaning to some of your error variants.
impl FFIError for NetXFFIError {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::NullPassed;
    const PANIC: Self = Self::Panic;
}

impl From<anyhow::Error> for NetXFFIError {
    fn from(err: Error) -> Self {
        log::error!("error:{:?}", err);
        NetXFFIError::AnyHowError
    }
}
