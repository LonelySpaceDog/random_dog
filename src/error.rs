
#[derive(Debug, Clone)]
pub enum Error {
  APIError,
  ImgSaveError
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Error {
    dbg!(error);

    Error::APIError
  }
}

impl From<image::ImageError> for Error {
  fn from(error: image::ImageError) -> Error{
    dbg!(error);

    Error::ImgSaveError
  }
}
