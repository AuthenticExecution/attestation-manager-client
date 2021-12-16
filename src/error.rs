#[derive(Debug)]
pub enum Error {
    OpenFileError(String),
    HandlerNotImplemented,
    ServerError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}
