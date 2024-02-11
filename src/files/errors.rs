#[derive(Debug)]
pub enum ErrorCode {
    FileAlreadyExists,
    StorageDirCreationFailed(std::io::Error),
    FileMoveFailed(std::io::Error),
    RowRetrievalFailed(postgres::Error),
    ClientConnectError(postgres::error::Error),
}
