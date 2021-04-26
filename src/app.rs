#[path = "button_style.rs"]
mod button_style;
use json;
use reqwest;
use iced::{
  image, button, Align, Application, Button, Clipboard, Column, Command,
  Container, Element, Length, Row, Text,
};

use button_style::style;

#[derive(Debug)]
pub enum RandomDog {
  Loading,
  Loaded {
    img: Img,
    search: button::State,
  },
  Errored {
    error: Error,
    try_again: button::State,
  },
}

#[derive(Debug, Clone)]
pub enum Message {
  ImgFound(Result<Img, Error>),
  Search,
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
      RandomDog::Loaded { img, .. } => &img.name,
      RandomDog::Errored { .. } => "Somthing went wrong!",
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
    }
  }

  fn view(&mut self) -> Element<Message> {
    let content = match self {
      RandomDog::Loading => Column::new()
        .width(Length::Shrink)
        .push(Text::new("Searching for Dog...").size(40)),
      RandomDog::Loaded { img, search } => Column::new()
        .max_width(500)
        .spacing(20)
        .align_items(Align::End)
        .push(img.view())
        .push(button(search, "Keep searching!").on_press(Message::Search)),
      RandomDog::Errored { try_again, .. } => Column::new()
        .spacing(20)
        .align_items(Align::End)
        .push(Text::new("Somthing is not right...").size(40))
        .push(button(try_again, "Try again").on_press(Message::Search)),
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
  name: String,
  image: image::Handle,
  image_viewer: image::viewer::State,
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
      .push(image::Viewer::new(
        &mut self.image_viewer,
        self.image.clone(),
      ))
      .into()
  }

  async fn search() -> Result<Img, Error> {
    let image = Self::get_dog_img().await?;
    Ok(Img {
      name: String::from("Dog"),
      image,
      image_viewer: image::viewer::State::new(),
    })
  }

  async fn get_dog_img() -> Result<image::Handle, reqwest::Error> {
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
    Ok(image::Handle::from_memory(img_bytes))
  }
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
  Button::new(state, Text::new(text))
    .padding(10)
    .style(style::Button::Primary)
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Error {
    dbg!(error);

    Error::APIError
  }
}
