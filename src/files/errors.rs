

#[derive(Debug)]
pub enum ErrorCode {
    FileAlreadyExists = 50,
    StorageDirCreationFailed = 22,
    FileCopyFailed = 27,
    FileRemoveFailed = 41,
    RowRetrievalFailed = 60,
}