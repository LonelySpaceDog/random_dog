mod image_util;
use image_util::{DogImage, AppState};
//TODO: More refactoring
use druid::{AppLauncher, WindowDesc};
use std::path::Path;
use std::fs::create_dir;

fn main() {

  let dogs_save_dir = Path::new("Dogs/");
  if !dogs_save_dir.exists() {
    create_dir(dogs_save_dir).expect("cant create saving dir");
  }

  let main_window = WindowDesc::new(image_util::make_ui)
    .window_size((650., 450.))
    .title("Random Dog on druid");

  let state = AppState {
    breed: String::from("any"),
    dog_image: DogImage::new(&String::from("any")),
  };

  AppLauncher::with_window(main_window)
    .launch(state)
    .expect("Faild to launc app");
}
