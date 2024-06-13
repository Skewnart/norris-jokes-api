pub struct NorrisError {
    error: String
}

impl From<ReqwestError> for NorrisError {
    fn from(err: ReqwestError) -> Self { 
        err.value
    } 
}
