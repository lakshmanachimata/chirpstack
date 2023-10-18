pub trait PrintFullError {
    fn full(&self) -> String;
}

impl PrintFullError for anyhow::Error {
    fn full(&self) -> String {
        format!("{:#}", self)
    }
}

impl PrintFullError for crate::storage::error::Error {
    fn full(&self) -> String {
        format!("{:#}", self)
    }
}
