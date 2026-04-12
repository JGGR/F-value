// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2026 jgabaut, gioninjo

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::controller::update_main;
use super::view::draw_main;
use crate::app::model::Model;
use crate::controllers::Controllers;
use crate::core::SHORT_PROJECT_VERSION;
use crate::views::Views;
use esox::domain::hfbi::AnagraficaHFBIDraft;
use esox::domain::index::Indice;
use esox::domain::niseci::AnagraficaNISECIDraft;
use raylib::prelude::*;
use std::fmt;
use std::path::PathBuf;
//use raylib::color::Color;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiControlProperty::TEXT_COLOR_NORMAL;
use raylib::consts::GuiDefaultProperty::{BACKGROUND_COLOR, TEXT_SIZE, TEXT_SPACING};
use raylib::consts::TraceLogLevel;
use raylib::core::texture::Image;

pub(crate) const EXIT_KEY: raylib::consts::KeyboardKey = raylib::consts::KeyboardKey::KEY_ESCAPE;
pub(crate) const ESOX_SCREEN_WIDTH: i32 = 960;
pub(crate) const ESOX_SCREEN_HEIGHT: i32 = 540;
pub(crate) const DARK_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_dark.rgs");
pub(crate) const BLUISH_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_bluish.rgs");
pub(crate) const CANDY_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_candy.rgs");
pub(crate) const CHERRY_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_cherry.rgs");
pub(crate) const CYBER_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_cyber.rgs");
pub(crate) const JUNGLE_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_jungle.rgs");
pub(crate) const LAVANDA_THEME_DATA: &[u8] =
    include_bytes!("../../assets/styles/style_lavanda.rgs");
pub(crate) const TERMINAL_THEME_DATA: &[u8] =
    include_bytes!("../../assets/styles/style_terminal.rgs");
pub(crate) const ASHES_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_ashes.rgs");
pub(crate) const CONSOLE_FONT_DATA: &[u8] = include_bytes!("../../assets/FreeMono.ttf");
pub(crate) const PROJECT_LOGO_DATA: &[u8] = include_bytes!("../../assets/logo.png");
pub(crate) const PROJECT_LOGO_NAME_DATA: &[u8] = include_bytes!("../../assets/logo_name.png");
pub(crate) const PROJECT_BG_DATA: &[u8] = include_bytes!("../../assets/sfondo.png");
pub(crate) const CISBA_LOGO_DATA: &[u8] = include_bytes!("../../assets/logo_cisba.png");

#[cfg(all(windows, debug_assertions))]
pub(crate) const SUPPORT_HEADLESS: bool = true;

#[cfg(all(windows, not(debug_assertions)))]
pub(crate) const SUPPORT_HEADLESS: bool = false; // This is due to windows_subsystem being "windows"

#[cfg(not(windows))]
pub(crate) const SUPPORT_HEADLESS: bool = true;

#[derive(Clone)]
pub(crate) enum MainAction {
    SetFontHeight(i32),
    SetTheme(i32),
    SetLocale(i32),
    ResetSettings,
    CloseSettings,
    OpenSettings,
    ShowInfo,
    CloseInfo,
    ShowLicense,
    CloseLicense,
    Reset,
    ShowReset,
    CloseReset,
    ShowConsole,
    CloseConsole,
    CloseQuit,
    Quit,
}

impl fmt::Display for MainAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            MainAction::SetFontHeight(_h) => "SetFontHeight",
            MainAction::SetTheme(_ti) => "SetTheme",
            MainAction::SetLocale(_li) => "SetLocale",
            MainAction::ResetSettings => "ResetSettings",
            MainAction::CloseSettings => "CloseSettings",
            MainAction::OpenSettings => "OpenSettings",
            MainAction::ShowInfo => "ShowInfo",
            MainAction::CloseInfo => "CloseInfo",
            MainAction::ShowLicense => "ShowLicense",
            MainAction::CloseLicense => "CloseLicense",
            MainAction::Reset => "Reset",
            MainAction::ShowReset => "ShowReset",
            MainAction::CloseReset => "CloseReset",
            MainAction::ShowConsole => "ShowConsole",
            MainAction::CloseConsole => "CloseConsole",
            MainAction::Quit => "Quit",
            MainAction::CloseQuit => "CloseQuit",
        };
        write!(f, "{}", string_representation)
    }
}

#[derive(Clone)]
pub(crate) enum Action {
    ConsoleBackout,
    UserContinued,
    UserWantsInfo,
    PickIndice(Indice),
    PickRiferimentoPath(Option<PathBuf>),
    PickCampionamentoPath(Option<PathBuf>),
    ValidaRiferimentoPath(bool),
    ValidaCampionamentoPath(bool),
    SubmitAnagraficaNISECI(AnagraficaNISECIDraft),
    SubmitAnagraficaHFBI(AnagraficaHFBIDraft),
    CheckAnagrafica,
    BackoutAnagrafica,
    RunCalc,
    ConfirmCalc,
    ExportPdf(PathBuf),
    Reset,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            Action::ConsoleBackout => "ConsoleBackout",
            Action::UserContinued => "UserContinued",
            Action::UserWantsInfo => "UserWantsInfo",
            Action::PickIndice(_indice) => "PickIndice",
            Action::PickRiferimentoPath(_path) => "PickRiferimentoPath",
            Action::PickCampionamentoPath(_path) => "PickCampionamentoPath",
            Action::ValidaRiferimentoPath(_has_headers) => "ValidaRiferimentoPath",
            Action::ValidaCampionamentoPath(_has_headers) => "ValidaCampionamentoPath",
            Action::SubmitAnagraficaNISECI(_a) => "SubmitAnagraficaNISECI",
            Action::SubmitAnagraficaHFBI(_a) => "SubmitAnagraficaHFBI",
            Action::CheckAnagrafica => "CheckAnagrafica",
            Action::BackoutAnagrafica => "BackoutAnagrafica",
            Action::RunCalc => "RunCalc",
            Action::ConfirmCalc => "ConfirmCalc",
            Action::ExportPdf(_path) => "ExportPdf",
            Action::Reset => "Reset",
        };
        write!(f, "{}", string_representation)
    }
}

#[derive(Copy, Clone)]
pub(crate) enum CurrentView {
    Home,
    Help,
    SelezioneIndice,
    SelezioneFileInput,
    ValidazioneFileInput,
    SelezioneInfoAggiuntive,
    ValidazioneInfoAggiuntive,
    ProduzioneOutput,
    ProduzionePDF,
    Console,
}

impl fmt::Display for CurrentView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            CurrentView::Home => "Home",
            CurrentView::Help => "Help",
            CurrentView::SelezioneIndice => "Selezione Indice",
            CurrentView::SelezioneFileInput => "Selezione File Input",
            CurrentView::ValidazioneFileInput => "Validazione File Input",
            CurrentView::SelezioneInfoAggiuntive => "Selezione Info Aggiuntive",
            CurrentView::ValidazioneInfoAggiuntive => "Validazione Info Aggiuntive",
            CurrentView::ProduzioneOutput => "Produzione Output",
            CurrentView::ProduzionePDF => "Produzione PDF",
            CurrentView::Console => "Console",
        };
        write!(f, "{}", string_representation)
    }
}

macro_rules! define_enum_with_str {
    (
        $name:ident => [
            $first:ident $(=> $first_str:expr)?
            $(, $rest:ident $(=> $rest_str:expr)?)* $(,)?
        ]
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(i32)]
        pub(crate) enum $name {
            $first,
            $($rest),*
        }

        impl $name {
            pub(crate) const COMBOBOX_STR: &'static str = concat!(
                define_enum_with_str!(@label $first $(=> $first_str)?)
                $(, ";", define_enum_with_str!(@label $rest $(=> $rest_str)?))*
            );
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    Self::$first => define_enum_with_str!(@label $first $(=> $first_str)?),
                    $(Self::$rest => define_enum_with_str!(@label $rest $(=> $rest_str)?)),*
                };
                f.write_str(s)
            }
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    x if x == Self::$first as i32 => Ok(Self::$first),
                    $(x if x == Self::$rest as i32 => Ok(Self::$rest),)*
                    _ => Err(()),
                }
            }
        }
    };

    // helper: use explicit string if provided, otherwise stringify!(ident)
    (@label $ident:ident => $label:expr) => { $label };
    (@label $ident:ident) => { stringify!($ident) };
}

define_enum_with_str!(GuiTheme => [ Light, Dark, Bluish, Candy, Cherry, Cyber, Jungle, Lavanda, Terminal, Ashes ]);

define_enum_with_str!(Localize => [ Italian, International ]);

define_enum_with_str!(RegioneItaliana => [
            Abruzzo, Basilicata, Calabria, Campania, EmiliaRomagna => "Emilia-Romagna",
            FriuliVeneziaGiulia => "Friuli-Venezia-Giulia", Lazio, Liguria,Lombardia,Marche,
            Molise,Piemonte,Puglia,Sardegna,Sicilia,Toscana,
            TrentinoAltoAdige => "Trentino-Alto-Adige",Umbria,ValleDAosta => "Valle d'Aosta",Veneto
]);

pub(crate) struct ColorState {
    pub(crate) default_txt_color: Color,
    pub(crate) default_bg_color: Color,
}

impl ColorState {
    pub(crate) fn new(default_txt_color: Color, default_bg_color: Color) -> Self {
        Self {
            default_txt_color,
            default_bg_color,
        }
    }
}

pub(crate) struct MainState {
    pub(crate) frame_counter: u32,
    pub(crate) showing_reset_win: bool,
    pub(crate) should_reset: bool,
    pub(crate) showing_quit_win: bool,
    pub(crate) should_quit: bool,
    pub(crate) showing_info_box: bool,
    pub(crate) showing_license_box: bool,
    pub(crate) showing_settings_box: bool,
    pub(crate) current_view: CurrentView,
    pub(crate) previous_view: CurrentView,
    pub(crate) theme: GuiTheme,
    pub(crate) gui_theme_combobox_active: i32,
    pub(crate) default_font_height: i32,
    pub(crate) current_font_height: i32,
    pub(crate) default_txt_spacing: i32,
    pub(crate) colors: ColorState,
    pub(crate) current_font: WeakFont,
    pub(crate) textures: AppTextures,
    pub(crate) locale: Localize,
    pub(crate) locale_combobox_active: i32,
}

impl MainState {
    fn new(
        default_font_height: i32,
        current_font_height: i32,
        default_txt_spacing: i32,
        current_font: WeakFont,
        colors: ColorState,
        textures: AppTextures,
        locale: Localize,
    ) -> Self {
        Self {
            frame_counter: 0,
            showing_reset_win: false,
            should_reset: false,
            showing_quit_win: false,
            should_quit: false,
            showing_info_box: false,
            showing_license_box: false,
            showing_settings_box: false,
            current_view: CurrentView::Home,
            previous_view: CurrentView::Home,
            theme: GuiTheme::Light,
            gui_theme_combobox_active: GuiTheme::Light as i32,
            default_font_height,
            current_font_height,
            default_txt_spacing,
            colors,
            current_font,
            textures,
            locale,
            locale_combobox_active: locale as i32,
        }
    }

    pub(crate) fn set_current_view(&mut self, view: CurrentView) {
        self.previous_view = self.current_view;
        self.current_view = view;
    }

    pub(crate) fn get_gui_should_lock(&self) -> bool {
        self.showing_reset_win
            || self.showing_quit_win
            || self.showing_info_box
            || self.showing_settings_box
            || self.showing_license_box
    }
}

pub(crate) fn propwidth(d: &RaylibDrawHandle<'_>, to_scale: i32) -> i32 {
    if !(0..=ESOX_SCREEN_WIDTH).contains(&to_scale) {
        panic!("propw():  invalid to_scale value received: {to_scale}");
    }
    let current_screen_width = d.get_screen_width();
    current_screen_width * to_scale / ESOX_SCREEN_WIDTH
}

pub(crate) fn propheight(d: &RaylibDrawHandle<'_>, to_scale: i32) -> i32 {
    if !(0..=ESOX_SCREEN_HEIGHT).contains(&to_scale) {
        panic!("proph():  invalid to_scale value received: {to_scale}");
    }
    let current_screen_height = d.get_screen_height();
    current_screen_height * to_scale / ESOX_SCREEN_HEIGHT
}

pub(crate) fn get_locale() -> Localize {
    let locale_str = locale_config::Locale::user_default();
    for l in locale_str.as_ref().split(",") {
        match l.replace("-", "_").as_str() {
            "it_IT" | "it_CH" | "it_SM" | "it_VA" => {
                return Localize::Italian;
            }
            _ => {}
        }
    }
    Localize::International
}

pub(crate) struct AppTextures {
    pub(crate) logo_texture: Option<Texture2D>,
    pub(crate) logo_name_texture: Option<Texture2D>,
    pub(crate) bg_texture: Option<Texture2D>,
}

impl AppTextures {
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let img_load_res = Image::load_image_from_mem(".png", PROJECT_LOGO_DATA);

        let mut logo_img = None;
        match img_load_res {
            Ok(img) => {
                logo_img = Some(img);
            }
            Err(err) => {
                println!("Error loading logo img: {err}");
            }
        }

        let img_load_res = Image::load_image_from_mem(".png", PROJECT_LOGO_NAME_DATA);

        let mut logo_name_img = None;
        match img_load_res {
            Ok(img) => {
                logo_name_img = Some(img);
            }
            Err(err) => {
                println!("Error loading logo name img: {err}");
            }
        }

        let img_load_res = Image::load_image_from_mem(".png", PROJECT_BG_DATA);

        let mut bg_img = None;
        match img_load_res {
            Ok(img) => {
                bg_img = Some(img);
            }
            Err(err) => {
                println!("Error loading bg img: {err}");
            }
        }

        let mut logo_texture = None;
        if let Some(img) = logo_img {
            logo_texture = Some(rl.load_texture_from_image(thread, &img).unwrap());
            // Set the window icon
            rl.set_window_icon(&img);
        }

        let mut logo_name_texture = None;
        if let Some(img) = logo_name_img {
            logo_name_texture = Some(rl.load_texture_from_image(thread, &img).unwrap());
        }

        let mut bg_texture = None;
        if let Some(img) = bg_img {
            bg_texture = Some(rl.load_texture_from_image(thread, &img).unwrap());
        }
        Self {
            logo_texture,
            logo_name_texture,
            bg_texture,
        }
    }
}

/// Application root.
///
/// ## Drop Order Invariant
///
/// Types like `Font` and `Texture` (used inside `Views`) are backed by GPU
/// resources and require a valid OpenGL context to be destroyed safely.
///
/// `RaylibHandle` owns that context. If it is dropped first, any later
/// destruction of GPU resources will cause a segfault inside raylib.
///
/// Rust drops struct fields in declaration order (top to bottom),
/// so we ensure correct destruction order by placing `rl` last.
///
/// Dependency chain:
/// Views → Font → Texture → OpenGL context (rl)
///
/// Therefore:
/// - `views` must be dropped BEFORE `rl`
/// - `rl` must be the LAST field in this struct
pub(crate) struct App {
    main_state: MainState,
    model: Model,
    controllers: Controllers,
    views: Views,
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl App {
    pub(crate) fn new() -> App {
        let window_title = format!("F-value v{SHORT_PROJECT_VERSION}");

        let (mut rl, thread) = raylib::init()
            .size(ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT)
            .title(&window_title)
            .log_level(TraceLogLevel::LOG_ERROR) // Gets rid of raylib init text in the terminal
            .resizable()
            .build();

        rl.set_window_min_size(ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT);
        rl.set_exit_key(None); // This allows capturing the exit key with a message box
        rl.set_target_fps(30);

        let app_textures = AppTextures::new(&mut rl, &thread);

        // 10 is way too small for the default font height
        let gui_default_font_height: i32 = rl.gui_get_style(DEFAULT, TEXT_SIZE) * 2;
        rl.gui_set_style(DEFAULT, TEXT_SIZE, gui_default_font_height);
        let gui_current_font_height: i32 = gui_default_font_height;

        let txt_color_int = rl.gui_get_style(DEFAULT, TEXT_COLOR_NORMAL);
        let bg_color_int = rl.gui_get_style(DEFAULT, BACKGROUND_COLOR);
        let txt_spacing = rl.gui_get_style(DEFAULT, TEXT_SPACING) * 2;
        rl.gui_set_style(DEFAULT, TEXT_SPACING, txt_spacing);
        let current_font = rl.gui_get_font();
        let locale = get_locale();
        let colors = ColorState::new(
            Color::get_color(txt_color_int as u32),
            Color::get_color(bg_color_int as u32),
        );
        let main_state = MainState::new(
            gui_default_font_height,
            gui_current_font_height,
            txt_spacing,
            current_font,
            colors,
            app_textures,
            locale,
        );
        let model = Model::new();
        let controllers = Controllers::new();
        let views = Views::new(
            &mut rl,
            &thread,
            main_state.current_font_height,
            main_state.default_txt_spacing,
        );
        Self {
            main_state,
            model,
            controllers,
            views,
            rl,
            thread,
        }
    }

    pub(crate) fn run(&mut self) {
        let mut actions = Vec::<Action>::new();
        let mut main_actions = Vec::<MainAction>::new();
        while !self.main_state.should_quit {
            // Base update step
            update_main(&mut self.rl, &mut main_actions, &mut self.main_state);

            self.controllers.update(
                &mut self.rl,
                &mut self.model,
                &mut actions,
                &mut self.main_state,
            );

            let mut d = self.rl.begin_drawing(&self.thread);

            let lock_view = self.main_state.get_gui_should_lock();

            if lock_view {
                d.gui_lock();
            }

            // Ask the view for render, passing the controller for state changes
            // Current view draw step
            actions = self
                .views
                .draw(&mut d, &self.thread, &self.model, &self.main_state);

            if lock_view {
                d.gui_unlock();
            }

            // Base draw step
            // Render stuff not depending on view
            main_actions = draw_main(&mut d, &self.main_state);
        }
    }
}
