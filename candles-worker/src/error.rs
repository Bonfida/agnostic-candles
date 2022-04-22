use std::fmt::Display;

#[derive(Debug)]
pub enum WorkerError {
    DeserializationError,
    RpcError,
    AccountNotFound,
    DbCommitError,
}

impl Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkerError::DeserializationError => write!(f, "Error deserializing data"),
            WorkerError::RpcError => write!(f, "Rpc error"),
            WorkerError::AccountNotFound => write!(f, "Account not found"),
            WorkerError::DbCommitError => write!(f, "Failed to commit account to database"),
        }
    }
}

impl From<WorkerError> for std::io::Error {
    fn from(e: WorkerError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    }
}

impl std::error::Error for WorkerError {}
