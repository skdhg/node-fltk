#[path = "./window.rs"]
mod window;

#[path = "./utils.rs"]
mod utils;

use fltk::{app};
use napi::Result;

#[napi]
pub struct Application {
  app: app::App
}

#[napi]
impl Application {
  #[napi(constructor)]
  pub fn new() -> Self {
    Application { app: app::App::default() }
  }

  #[napi(getter)]
  pub fn version() -> String {
    app::version_str()
  }

  #[napi]
  pub fn create_window(&self, options: window::CreateWindowOptions) -> Result<window::Window> {
    let window = window::Window::new(options);
    
    Ok(window)
  }

  #[napi]
  pub fn run(&self) {
    self.app.run().unwrap();
  }

  #[napi]
  pub fn load_system_fonts(&self) {
    self.app.load_system_fonts();
  }

  #[napi]
  pub fn redraw(&self) {
    self.app.redraw();
  }

  #[napi]
  pub fn wait(&self) {
    self.app.wait();
  }

  #[napi]
  pub fn quit(&self) {
    self.app.quit();
  }

  #[napi(getter)]
  pub fn fonts(&self) -> Result<Vec<String>> {
    Ok(app::fonts())
  }

  #[napi]
  pub fn set_background_color(&self, color: String) {
    let c = utils::parse_color(color);
    app::background(c.r, c.g, c.b);
  }
}