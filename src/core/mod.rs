pub mod view;
pub mod controller;

use raylib::prelude::*;
use std::fmt;

pub const EXIT_KEY: raylib::consts::KeyboardKey = raylib::consts::KeyboardKey::KEY_ESCAPE;
pub const PROJECT_VERSION: &'static str = env!("VERSION_STRING");
pub const SHORT_PROJECT_VERSION: &'static str = env!("SHORT_VERSION_STRING");
pub const ESOX_SCREEN_WIDTH : i32 = 960;
pub const ESOX_SCREEN_HEIGHT : i32 = 540;

pub enum CurrentView {
    HOME,
    SECOND,
    SelezioneIndice,
    SelezioneFileInput,
    ValidazioneFileInput,
    SelezioneInfoAggiuntive,
    ValidazioneInfoAggiuntive,
    ProduzioneOutput,
    ProduzionePDF,
}

impl fmt::Display for CurrentView {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = match *self {
      CurrentView::HOME => "HOME",
      CurrentView::SECOND => "SECOND",
      CurrentView::SelezioneIndice => "Selezione Indice",
      CurrentView::SelezioneFileInput => "Selezione File Input",
      CurrentView::ValidazioneFileInput => "Validazione File Input",
      CurrentView::SelezioneInfoAggiuntive => "Selezione Info Aggiuntive",
      CurrentView::ValidazioneInfoAggiuntive => "Validazione Info Aggiuntive",
      CurrentView::ProduzioneOutput => "Produzione Output",
      CurrentView::ProduzionePDF => "Produzione PDF",
    };
    write!(f, "{}", string_representation)
  }
}


pub struct MainState {
    pub showing_quit_win : bool,
    pub should_quit : bool,
    pub showing_info_box : bool,
    pub showing_settings_box : bool,
    pub spinner_font_height_edit_mode : bool,
    pub current_view : CurrentView,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            showing_quit_win : false,
            should_quit : false,
            showing_info_box : false,
            showing_settings_box : false,
            spinner_font_height_edit_mode : false,
            current_view : CurrentView::HOME
        }
    }
}

pub fn propwidth(d : &RaylibDrawHandle<'_>, to_scale : i32) -> i32
{
    if to_scale < 0 || to_scale > ESOX_SCREEN_WIDTH {
        panic!("propw():  invalid to_scale value received: {to_scale}");
    }
    let current_screen_width = d.get_screen_width();
    return current_screen_width * to_scale / ESOX_SCREEN_WIDTH;
}

pub fn propheight(d : &RaylibDrawHandle<'_>, to_scale : i32) -> i32
{
    if to_scale < 0 || to_scale > ESOX_SCREEN_HEIGHT {
        panic!("proph():  invalid to_scale value received: {to_scale}");
    }
    let current_screen_height = d.get_screen_height();
    return current_screen_height * to_scale / ESOX_SCREEN_HEIGHT;
}
