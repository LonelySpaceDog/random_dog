#[path = "button_style.rs"]
mod button_style;
use json;
use image;
use reqwest;
use iced::{
  button, Align, Application, Button, Clipboard, Column, Command, Container,
  Element, Length, Row, Text,
};

use button_style::style;

#[derive(Debug)]
pub enum RandomDog {
  Loading,
  Loaded {
    img: Img,
    search: button::State,
    save: button::State,
  },
  Errored {
    error: Error,
    try_again: button::State,
  },
  Saving {
    img: Img,
  },
  Saved,
}

#[derive(Debug, Clone)]
pub enum Message {
  ImgFound(Result<Img, Error>),
  Search,
  Saving,
  Saved,
}

impl Application for RandomDog {
  type Executor = iced::executor::Default;
  type Message = Message;
  type Flags = ();

  fn new(_flags: ()) -> (RandomDog, Command<Message>) {
    (
      RandomDog::Loading,
      Command::perform(Img::search(), Message::ImgFound),
    )
  }

  fn title(&self) -> String {
    let subtitle = match self {
      RandomDog::Loading => "Loading",
      RandomDog::Loaded { img, .. } => &img.breed,
      RandomDog::Errored { .. } => "Somthing went wrong!",
      RandomDog::Saving { .. } => "Saving",
      RandomDog::Saved => "Dog saved on your computer",
    };

    format!("{} - Random Dog", subtitle)
  }

  fn update(
    &mut self,
    message: Message,
    _clipboard: &mut Clipboard,
  ) -> Command<Message> {
    match message {
      Message::ImgFound(Ok(img)) => {
        *self = RandomDog::Loaded {
          img,
          search: button::State::new(),
          save: button::State::new(),
        };

        Command::none()
      }
      Message::ImgFound(Err(error)) => {
        *self = RandomDog::Errored {
          error,
          try_again: button::State::new(),
        };

        Command::none()
      }
      Message::Search => match self {
        &mut RandomDog::Loading => Command::none(),
        _ => {
          *self = RandomDog::Loading;
          Command::perform(Img::search(), Message::ImgFound)
        }
      },
      Message::Saving => Command::none(),
      Message::Saved => Command::none(),
    }
  }

  fn view(&mut self) -> Element<Message> {
    let content = match self {
      RandomDog::Loading => Column::new()
        .width(Length::Shrink)
        .push(Text::new("Searching for Dog...").size(40)),
      RandomDog::Loaded { img, search, save } => Column::new()
        .max_width(500)
        .spacing(20)
        .align_items(Align::Start)
        .push(img.view())
        .push(
          button_search(search, "Keep searching!").on_press(Message::Search),
        )
        .push(button_save(save, "Save this dog").on_press(Message::Saving)),
      RandomDog::Errored { try_again, .. } => Column::new()
        .spacing(20)
        .align_items(Align::End)
        .push(Text::new("Somthing is not right...").size(40))
        .push(button_search(try_again, "Try again").on_press(Message::Search)),
      RandomDog::Saving {..} => {todo!()},
      RandomDog::Saved  => {todo!()},
    };

    Container::new(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x()
      .center_y()
      .into()
  }
}

#[derive(Debug, Clone)]
pub struct Img {
  breed: String,
  image: iced::image::Handle,
  image_bytes: Vec<u8>,
  image_viewer: iced::image::viewer::State,
}

#[derive(Debug, Clone)]
pub enum Error {
  APIError,
}

impl Img {
  fn view(&mut self) -> Element<Message> {
    Row::new()
      .spacing(20)
      .align_items(iced::Align::Center)
      .push(iced::image::Viewer::new(
        &mut self.image_viewer,
        self.image.clone(),
      ))
      .into()
  }

  async fn search() -> Result<Img, Error> {
    let (image, image_bytes): (iced::image::Handle, Vec<u8>) =
      Self::get_dog_img().await?;
    Ok(Img {
      breed: String::from("any"),
      image,
      image_bytes,
      image_viewer: iced::image::viewer::State::new(),
    })
  }

  async fn get_dog_img(
  ) -> Result<(iced::image::Handle, Vec<u8>), reqwest::Error> {
    let url = "https://dog.ceo/api/breeds/image/random";
    let res = reqwest::get(url).await?.text().await?;
    let img_bytes = reqwest::get(
      json::parse(&res).expect("Parse error")["message"].to_string(),
    )
    .await?
    .bytes()
    .await?
    .as_ref()
    .to_vec();
    Ok((
      iced::image::Handle::from_memory(img_bytes.clone()),
      img_bytes,
    ))
  }
}

fn button_search<'a>(
  state: &'a mut button::State,
  text: &str,
) -> Button<'a, Message> {
  Button::new(state, Text::new(text))
    .padding(10)
    .style(style::Button::Primary)
}

fn button_save<'a>(
  state: &'a mut button::State,
  text: &str,
) -> Button<'a, Message> {
  Button::new(state, Text::new(text))
    .padding(20)
    .style(style::Button::Primary)
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Error {
    dbg!(error);

    Error::APIError
  }
}
