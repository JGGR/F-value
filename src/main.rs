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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod args;
mod console;
mod controllers;
mod core;
mod domain;
mod engines;
#[cfg(test)]
mod tests;
mod views;

use crate::app::core::{
    get_locale, MainState, ESOX_SCREEN_HEIGHT, ESOX_SCREEN_WIDTH, PROJECT_LOGO_DATA,
};
use crate::app::model::Model;
use crate::args::handle_args;
use crate::controllers::Controllers;
use crate::core::{prep_logger, SHORT_PROJECT_VERSION};
use crate::views::Views;
use raylib::color::Color;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiControlProperty::TEXT_COLOR_NORMAL;
use raylib::consts::GuiDefaultProperty::{BACKGROUND_COLOR, TEXT_SIZE, TEXT_SPACING};
use raylib::consts::TraceLogLevel;
use raylib::core::texture::Image;

fn main() {
    let _ = prep_logger();

    handle_args();

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

    let mut logo_texture = None;
    if let Some(img) = logo_img {
        logo_texture = Some(rl.load_texture_from_image(&thread, &img).unwrap());
        // Set the window icon
        rl.set_window_icon(&img);
    }

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
    let mut main_state = MainState::new(
        gui_default_font_height,
        gui_current_font_height,
        txt_spacing,
        current_font,
        Color::get_color(txt_color_int as u32),
        Color::get_color(bg_color_int as u32),
        logo_texture,
        locale,
    );

    let mut model = Model::new();

    let controllers = Controllers::new();

    let mut views = Views::new(&mut rl, &thread, gui_current_font_height, txt_spacing);

    main_state.mainloop(&mut rl, &thread, &mut model, &controllers, &mut views);
}
