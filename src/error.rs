use thiserror::Error;

#[derive(Debug, Error)]
pub enum MiraError {
    #[error("command {command:?} not found")]
    CommandLoadError { command: &'static str },

    #[error("backend(libloading) error")]
    BackendError(#[from] libloading::Error),
}
