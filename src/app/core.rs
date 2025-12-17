// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2025 jgabaut, gioninjo

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
use esox::domain::hfbi::AnagraficaHFBIDraft;
use esox::domain::index::Indice;
use esox::domain::niseci::AnagraficaNISECIDraft;
use crate::views::Views;
use raylib::prelude::*;
use std::fmt;
use std::path::PathBuf;

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
pub(crate) const CISBA_LOGO_DATA: &[u8] = include_bytes!("../../assets/logo_cisba.png");
pub(crate) const ISPRA_LOGO_DATA: &[u8] = include_bytes!("../../assets/logo_ispra.png");

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

//TODO: add test to check if this string respects the discriminant ordering in GuiTheme
pub(crate) const GUI_THEME_COMBOBOX_STR: &str =
    "Light;Dark;Bluish;Candy;Cherry;Cyber;Jungle;Lavanda;Terminal;Ashes";

//TODO: add test to check if this string respects the discriminant ordering in Localize
pub(crate) const LOCALE_COMBOBOX_STR: &str = "Italian;International";

#[derive(Copy, Clone)]
pub(crate) enum GuiTheme {
    Light,
    Dark,
    Bluish,
    Candy,
    Cherry,
    Cyber,
    Jungle,
    Lavanda,
    Terminal,
    Ashes,
}

impl fmt::Display for GuiTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            GuiTheme::Light => "Light",
            GuiTheme::Dark => "Dark",
            GuiTheme::Bluish => "Bluish",
            GuiTheme::Candy => "Candy",
            GuiTheme::Cherry => "Cherry",
            GuiTheme::Cyber => "Cyber",
            GuiTheme::Jungle => "Jungle",
            GuiTheme::Lavanda => "Lavanda",
            GuiTheme::Terminal => "Terminal",
            GuiTheme::Ashes => "Ashes",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for GuiTheme {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == GuiTheme::Light as i32 => Ok(GuiTheme::Light),
            x if x == GuiTheme::Dark as i32 => Ok(GuiTheme::Dark),
            x if x == GuiTheme::Bluish as i32 => Ok(GuiTheme::Bluish),
            x if x == GuiTheme::Candy as i32 => Ok(GuiTheme::Candy),
            x if x == GuiTheme::Cherry as i32 => Ok(GuiTheme::Cherry),
            x if x == GuiTheme::Cyber as i32 => Ok(GuiTheme::Cyber),
            x if x == GuiTheme::Jungle as i32 => Ok(GuiTheme::Jungle),
            x if x == GuiTheme::Lavanda as i32 => Ok(GuiTheme::Lavanda),
            x if x == GuiTheme::Terminal as i32 => Ok(GuiTheme::Terminal),
            x if x == GuiTheme::Ashes as i32 => Ok(GuiTheme::Ashes),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Localize {
    Italian,
    International,
}

impl TryFrom<i32> for Localize {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Localize::Italian as i32 => Ok(Localize::Italian),
            x if x == Localize::International as i32 => Ok(Localize::International),
            _ => Err(()),
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
    pub(crate) default_txt_color: Color,
    pub(crate) current_font: WeakFont,
    pub(crate) default_bg_color: Color,
    pub(crate) logo_texture: Option<Texture2D>,
    pub(crate) locale: Localize,
    pub(crate) locale_combobox_active: i32,
}

impl MainState {
    pub(crate) fn new(
        default_font_height: i32,
        current_font_height: i32,
        default_txt_spacing: i32,
        current_font: WeakFont,
        default_txt_color: Color,
        default_bg_color: Color,
        logo_texture: Option<Texture2D>,
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
            default_txt_color,
            current_font,
            default_bg_color,
            logo_texture,
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

    pub(crate) fn mainloop(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        state: &mut Model,
        controllers: &Controllers,
        views: &mut Views,
    ) {
        let mut actions = Vec::<Action>::new();
        let mut main_actions = Vec::<MainAction>::new();
        while !self.should_quit {
            // Base update step
            update_main(rl, &mut main_actions, self);

            controllers.update(rl, state, &mut actions, self);

            let mut d = rl.begin_drawing(thread);

            let lock_view = self.get_gui_should_lock();

            if lock_view {
                d.gui_lock();
            }

            // Ask the view for render, passing the controller for state changes
            // Current view draw step
            actions = views.draw(&mut d, thread, state, self);

            if lock_view {
                d.gui_unlock();
            }

            // Base draw step
            // Render stuff not depending on view
            main_actions = draw_main(&mut d, self);
        }
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
