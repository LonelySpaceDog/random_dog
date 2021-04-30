#[path = "image_util.rs"]
mod image_util;
#[path = "button_style.rs"]
mod button_style;
#[path = "error.rs"]
mod error;

use iced::{
  button, Align, Application, Button, Clipboard, Column, Command, Container,
  Element, Length, Row, Text,
};

use image_util::Img;
use error::Error;
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
  Saving,
  Saved {
    search: button::State,
  },
}

#[derive(Debug, Clone)]
pub enum Message {
  ImgFound(Result<Img, Error>),
  Search,
  Saving((Vec<u8>, String, String)),
  Saved(Result<(), Error>),
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
      RandomDog::Saved { .. } => "Dog saved on your computer",
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
      Message::Saving((img_buf, breed, file_name)) => {
        *self = RandomDog::Saving;
        Command::perform(Img::save(img_buf, breed, file_name), Message::Saved)
      }
      Message::Saved(Ok(())) => {
        *self = RandomDog::Saved {
          search: button::State::new(),
        };
        Command::none()
      }
      Message::Saved(Err(error)) => {
        *self = RandomDog::Errored {
          error,
          try_again: button::State::new(),
        };

        Command::none()
      }
    }
  }

  fn view(&mut self) -> Element<Message> {
    let content = match self {
      RandomDog::Loading => Column::new()
        .width(Length::Shrink)
        .push(Text::new("Searching for Dog...").size(40)),
      RandomDog::Loaded { img, search, save } => {
        let image_bytes = img.image_bytes.clone();
        let breed = img.breed.clone();
        let file_name = img.file_name.clone();
        Column::new()
          .max_width(500)
          .spacing(20)
          .align_items(Align::Start)
          .push(img.view())
          .push(
            Row::new()
              .spacing(20)
              .push(
                button_search(search, "Keep searching!")
                  .on_press(Message::Search),
              )
              .push(
                button_save(save, "Save this dog")
                  .on_press(Message::Saving((image_bytes, breed,file_name))),
              ),
          )
      }
      RandomDog::Errored { try_again, .. } => Column::new()
        .spacing(20)
        .align_items(Align::End)
        .push(Text::new("Somthing is not right...").size(40))
        .push(button_search(try_again, "Try again").on_press(Message::Search)),
      RandomDog::Saving { .. } => {
        Column::new().push(Text::new("Saving Dog...").size(20))
      }
      RandomDog::Saved { search } => Column::new()
        .spacing(20)
        .align_items(Align::Center)
        .push(Text::new("Dog has been saved").size(20))
        .push(
          button_search(search, "Search for new Dog").on_press(Message::Search),
        ),
    };

    Container::new(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x()
      .center_y()
      .into()
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
    .padding(10)
    .style(style::Button::Primary)
}
