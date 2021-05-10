use reqwest::Error as ReqwestError;
#[derive(Debug)]
pub enum Error {
  ImageError,
  ReqwestError,
}

impl From<ReqwestError> for Error {
  fn from(error: ReqwestError) -> Error {
    dbg!(error);

    Self::ReqwestError
  }
}
