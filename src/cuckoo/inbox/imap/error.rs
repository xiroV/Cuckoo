use super::{IMAPErrorHandler, IMAPError, IMAPErrorType};

impl IMAPErrorHandler for IMAPError {
    fn new(error_type: IMAPErrorType) -> Self {
        IMAPError {
            err_type: error_type,
        }
    }
}
