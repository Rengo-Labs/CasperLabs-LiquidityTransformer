use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    InvalidAddress = 65,
    InvalidCallDetected = 66,
    InvalidWiseContractAddress = 67,
    InvalidTransferHelperAddress = 68,
    InvalidDeposit = 69,
    InvalidState = 70,
    AlreadyDefined = 71,
    DepositDisabled = 72,
    NotOwner = 73,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
