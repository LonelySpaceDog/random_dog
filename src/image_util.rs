use std::{fs, path::Path};
use json;
use image;
use reqwest;
use iced::{Align, Element, Length, Row};

use super::{Message, error::Error};

#[derive(Debug, Clone)]
pub struct Img {
  pub breed: String,
  pub image: iced::image::Handle,
  pub image_bytes: Vec<u8>,
  pub image_viewer: iced::image::viewer::State,
  pub file_name: String,
}

impl Img {
  pub fn view(&mut self) -> Element<Message> {
    Row::new()
      .spacing(20)
      .align_items(Align::Center)
      .height(Length::Units(500))
      .width(Length::Units(500))
      .push(iced::image::Viewer::new(
        &mut self.image_viewer,
        self.image.clone(),
      ))
      .into()
  }

  pub async fn search() -> Result<Img, Error> {
    let (image, image_bytes, file_name): (
      iced::image::Handle,
      Vec<u8>,
      String,
    ) = Self::get_dog_img().await?;
    Ok(Img {
      breed: String::from("any"),
      image,
      image_bytes,
      image_viewer: iced::image::viewer::State::new(),
      file_name,
    })
  }

  pub async fn save(
    image_bytes: Vec<u8>,
    breed: String,
    file_name: String,
  ) -> Result<(), Error> {
    let loaded_image = image::load_from_memory(&image_bytes)?;
    if !Path::new("./Dogs/").exists() {
      fs::create_dir("./Dogs").expect("Cant create Dogs dir");
    }
    let path_str = format!("./Dogs/dog_{}_{}.jpeg", breed, file_name);
    let path = Path::new(&path_str);
    loaded_image.save_with_format(path, image::ImageFormat::Jpeg)?;
    Ok(())
  }

  async fn get_dog_img(
  ) -> Result<(iced::image::Handle, Vec<u8>, String), reqwest::Error> {
    let url = "https://dog.ceo/api/breeds/image/random";
    let res = reqwest::get(url).await?.text().await?;
    let url = json::parse(&res).expect("Parse error")["message"].to_string();
    let img_bytes = reqwest::get(&url).await?.bytes().await?.as_ref().to_vec();
    Ok((
      iced::image::Handle::from_memory(img_bytes.clone()),
      img_bytes,
      Path::new(&url)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap(),
    ))
  }
}
