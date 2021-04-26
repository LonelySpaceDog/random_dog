
mod app;
use iced::{Result, Application};
use app::RandomDog;
fn main() -> Result {
  RandomDog::run(iced::Settings::default())
}
