#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod model;
mod views;
mod controllers;
mod core;

use crate::core::*;
use crate::core::view::*;
use crate::core::controller::*;
use crate::controllers::*;
use crate::views::*;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiDefaultProperty::TEXT_SIZE;

fn main() {
    eprintln!("{PROJECT_VERSION}");
    let home_controller = HomeController::new();
    let mut home_view = HomeView::new();
    let second_controller = SecondController::new();
    let mut second_view = SecondView::new();

    let mut main_state = MainState::new();

    let (mut rl, thread) = raylib::init()
        .size(ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT)
        .title("Controller & View Example")
        .resizable()
        .build();

    rl.set_window_min_size(ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT);
    rl.set_exit_key(None); // This allows capturing the exit key with a message box
    rl.set_target_fps(30);

    let default_font_height : i32 = rl.gui_get_style(DEFAULT, TEXT_SIZE as i32);
    let mut current_font_height : i32 = default_font_height;

    while !main_state.should_quit {

        // Base update step
        update_main(&mut rl, &mut main_state, current_font_height);

        // Current view update step
        match main_state.current_view {
            CurrentView::HOME => {
                home_controller.update(&rl);
            }
            CurrentView::SECOND => {
                second_controller.update(&rl);
            }
        }

        let mut d = rl.begin_drawing(&thread);

        // Ask the view for render, passing the controller for state changes
        // Current view draw step
        match main_state.current_view {
            CurrentView::HOME => {
                home_view.draw(&mut d, &thread, &home_controller, current_font_height);
            }
            CurrentView::SECOND => {
                second_view.draw(&mut d, &thread, &second_controller, current_font_height);
            }
        }

        // Base draw step
        // Render stuff not depending on view
        draw_main(&mut d, &mut main_state, &mut current_font_height, default_font_height);
    }
}

