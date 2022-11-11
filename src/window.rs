use fltk::{prelude::*, window::Window as FltkWindow};
use napi::{Result};


#[napi]
pub struct Window {
  window: FltkWindow
}

#[napi(object)]
pub struct CreateWindowOptions {
  pub name: String,
  pub width: i32,
  pub height: i32,
}

#[napi]
impl Window {
  #[napi(constructor)]
  pub fn new(options: CreateWindowOptions) -> Self {
    Window { window: FltkWindow::default().with_size(options.width, options.height).with_label(options.name.as_str()) }
  }

  #[napi]
  pub fn set_size(&mut self, width: i32, height: i32) {
    self.window.set_size(width, height);
  }

  #[napi]
  pub fn end(&self) {
    self.window.end();
  }

  #[napi]
  pub fn hide(&mut self) {
    self.window.hide();
  }

  #[napi]
  pub fn show(&mut self) {
    self.window.show();
  }

  #[napi]
  pub fn set_resizable(&mut self, resizable: bool) {
    self.window.make_resizable(resizable);
  }

  #[napi]
  pub fn flush(&mut self) {
    self.window.flush();
  }

  #[napi(getter)]
  pub fn width(&mut self) -> Result<i32> {
    Ok(self.window.width())
  }

  #[napi(getter)]
  pub fn height(&mut self) -> Result<i32> {
    Ok(self.window.height())
  }

  #[napi]
  pub fn set_title(&mut self, title: String) {
    self.window.set_label(title.as_str());
  }

  #[napi(getter)]
  pub fn title(&mut self) -> Result<String> {
    Ok(self.window.label())
  }

  #[napi(getter)]
  pub fn visible(&mut self) -> Result<bool> {
    Ok(self.window.shown())
  }

  #[napi]
  pub fn center(&self) {
    self.window.clone().center_screen();
  }

  #[napi]
  pub fn set_fullscreen(&mut self, fullscreen: bool) {
    self.window.fullscreen(fullscreen);
  }
}