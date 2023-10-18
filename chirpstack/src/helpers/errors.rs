use anyhow::Error;

pub trait PrintFullError {
    fn full(&self) -> String;
}

impl PrintFullError for Error {
    fn full(&self) -> String {
        format!("{:#}", self)
    }
}
