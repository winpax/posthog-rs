#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Connection Error: {0}")]
    Connection(String),
    #[error("Serialization Error: {0}")]
    Serialization(String),
    #[error("Client already initialized")]
    AlreadyInitialized,
    #[error("Client not initialized")]
    NotInitialized,
}
