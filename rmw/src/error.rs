use rmw_sys::{
    rmw_ret_t, RMW_RET_BAD_ALLOC, RMW_RET_ERROR, RMW_RET_INCORRECT_RMW_IMPLEMENTATION,
    RMW_RET_INVALID_ARGUMENT, RMW_RET_NODE_NAME_NON_EXISTENT, RMW_RET_TIMEOUT, RMW_RET_UNSUPPORTED,
};
use tracing::error;

pub enum Error {
    BadAlloc,
    Error,
    IncorrectRmwImplementation,
    InvalidArgument,
    NodeNameNonExistent,
    Timeout,
    Unsupported,
    Other(::anyhow::Error),
}

impl Error {
    pub(crate) fn into_rmw_ret(self) -> rmw_ret_t {
        match self {
            Error::BadAlloc => RMW_RET_BAD_ALLOC,
            Error::Error => RMW_RET_ERROR,
            Error::IncorrectRmwImplementation => RMW_RET_INCORRECT_RMW_IMPLEMENTATION,
            Error::InvalidArgument => RMW_RET_INVALID_ARGUMENT,
            Error::Timeout => RMW_RET_TIMEOUT,
            Error::Unsupported => RMW_RET_UNSUPPORTED,
            Error::NodeNameNonExistent => RMW_RET_NODE_NAME_NON_EXISTENT,
            Error::Other(error) => {
                error!("error");
                RMW_RET_ERROR
            }
        }
    }
}

impl From<::anyhow::Error> for Error {
    fn from(error: ::anyhow::Error) -> Self {
        Self::Other(error)
    }
}
