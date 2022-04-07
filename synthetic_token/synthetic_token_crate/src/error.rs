use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    NotOwner = 0,
    Div1,
    Div2,
    Div3,
    Div4,
    Div5,
    Div6,
    Div7,
    Div8,
    Sub1,
    NotSCSPR,
    WithdrawFailed,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
