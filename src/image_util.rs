#[path = "error.rs"]
mod error;
use error::Error;

use image;
use reqwest;
use serde_json::Value;
use druid::{Color, ContextMenu, Data, ImageBuf, LocalizedString, MenuDesc, MenuItem, Point, Selector};
use druid::widget::{
  prelude::*, SizedBox, Image, WidgetExt, Flex, FillStrat, Button,
};

#[derive(Clone, Data)]
pub struct AppState {
  pub breed: String,
  pub dog_image: DogImage,
}
struct ImageRebuilder {
  inner: Box<dyn Widget<AppState>>,
}

impl ImageRebuilder {
  fn new() -> ImageRebuilder {
    ImageRebuilder {
      inner: SizedBox::empty().boxed(),
    }
  }

  fn rebuild_inner(&mut self, data: &AppState) {
    self.inner = build_widget(&data);
  }
}

impl Widget<AppState> for ImageRebuilder {
  fn event(
    &mut self,
    ctx: &mut EventCtx,
    event: &Event,
    data: &mut AppState,
    env: &Env,
  ) {
    self.inner.event(ctx, event, data, env)
  }

  fn lifecycle(
    &mut self,
    ctx: &mut LifeCycleCtx,
    event: &LifeCycle,
    data: &AppState,
    env: &Env,
  ) {
    if let LifeCycle::WidgetAdded = event {
      self.rebuild_inner(data)
    }
    self.inner.lifecycle(ctx, event, data, env)
  }

  fn update(
    &mut self,
    ctx: &mut UpdateCtx,
    old_data: &AppState,
    data: &AppState,
    _env: &Env,
  ) {
    if !old_data.same(&data) {
      self.rebuild_inner(data);
      ctx.children_changed();
    }
  }

  fn layout(
    &mut self,
    ctx: &mut LayoutCtx,
    bc: &BoxConstraints,
    data: &AppState,
    env: &Env,
  ) -> Size {
    self.inner.layout(ctx, bc, data, env)
  }

  fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
    self.inner.paint(ctx, data, env)
  }

  fn id(&self) -> Option<WidgetId> {
    self.inner.id()
  }
}

#[derive(Clone, Debug, Data)]
pub struct DogImage {
  pub file_name: String,
  #[data(ignore)]
  pub image: image::DynamicImage,
}

impl DogImage {
  pub fn new(breed: &String) -> Self {
    Self::fetch_dog_image(breed).expect("Error in fetch dog")
  }

  fn fetch_dog_image(breed: &String) -> Result<Self, Error> {
    let url = "https://dog.ceo/api/breeds/image/random";
    let response = reqwest::blocking::get(url)?.json::<Value>()?;
    let url = response["message"]
      .as_str()
      .expect("Error in convert response message to &str");
    let image_bytes = reqwest::blocking::get(url)?.bytes()?;
    let image = image::load_from_memory(&image_bytes).unwrap();

    let (_, file_name): (_, &str) = url.rsplit_once('/').unwrap();
    Ok(Self {
      file_name: file_name.to_string(),
      image,
    })
  }

  fn save(&self) {
    self
      .image
      .save(format!("Dogs/{}", self.file_name))
      .expect("Error in save");
  }
}

pub fn make_ui() -> impl Widget<AppState> {
  Flex::column()
    .with_flex_child(
      Flex::row()
        .with_child(Button::new("Find the doggo").on_click(
          |_, data: &mut AppState, _| {
            data.dog_image = DogImage::new(&data.breed);
          },
        ))
        .with_spacer(10.0)
        .with_child(Button::new("Save this doggo").on_click(
          |_, data: &mut AppState, _| {
            data.dog_image.save();
          },
        )),
      2.0,
    )
    .with_spacer(10.0)
    .with_flex_child(ImageRebuilder::new(), 1.0)
    .padding(10.0)
}

fn build_widget(data: &AppState) -> Box<dyn Widget<AppState>> {
  let img =
    Image::new(ImageBuf::from_dynamic_image(data.dog_image.image.clone()))
      .fill_mode(FillStrat::Contain);
  SizedBox::new(img)
    .border(Color::grey(0.6), 1.0)
    .center()
    .boxed()
}
