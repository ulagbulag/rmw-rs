use rmw_sys::{rmw_ret_t, RMW_RET_OK};

use crate::error::Error;

pub trait RmwValueResult<T> {
    unsafe fn write(self, target: *mut T) -> rmw_ret_t;
}

impl<T> RmwValueResult<T> for Result<T> {
    unsafe fn write(self, target: *mut T) -> rmw_ret_t {
        match self {
            Ok(value) => {
                *target = value;
                RMW_RET_OK
            }
            Err(error) => error.into_rmw_ret(),
        }
    }
}

pub trait RmwProcessResult {
    unsafe fn finish(self) -> rmw_ret_t;
}

impl RmwProcessResult for Result<()> {
    unsafe fn finish(self) -> rmw_ret_t {
        match self {
            Ok(value) => RMW_RET_OK,
            Err(error) => error.into_rmw_ret(),
        }
    }
}

pub type Result<T, E = Error> = ::core::result::Result<T, E>;
