use reqwest::Error as reqwest_error;
#[derive(Debug)]
pub enum Error {
  ImageBufError,
  ReqwestError,
}

impl From<reqwest_error> for Error {
  fn from(error: reqwest_error) -> Error {
    Self::ReqwestError
  }
}
