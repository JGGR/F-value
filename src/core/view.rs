use crate::core::*;
use std::ffi::CString;
use raylib::consts::GuiIconName::*;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiDefaultProperty::{TEXT_SPACING, TEXT_SIZE};

pub fn draw_quit_win(d: &mut RaylibDrawHandle, showing_quit_win : &mut bool, should_quit : &mut bool) {
    if *showing_quit_win {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::RAYWHITE.alpha(0.8),
        );
        let itext = d.gui_icon_text(ICON_EXIT, Some(rstr!("Close Window")));
        let itext = CString::new(itext).unwrap();
        let result = d.gui_message_box(
            rrect(
                d.get_screen_width() / 2 - propwidth(&d, 125),
                d.get_screen_height() / 2 - propheight(&d, 50),
                propwidth(&d, 250),
                propheight(&d, 100)
            ),
            Some(itext.as_c_str()),
            Some(rstr!("Vuoi davvero uscire?")),
            Some(rstr!("Si;No")),
        );

        if (result == 0) || (result == 2) {
            *showing_quit_win = false;
        } else if result == 1 {
            *should_quit = true;
        }
    }
}

pub fn draw_info_box(d: &mut RaylibDrawHandle, showing_info_box : &mut bool, font: &WeakFont, default_txt_spacing: i32, default_txt_color : Color, current_font_height : i32) {
    if *showing_info_box {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::RAYWHITE.alpha(0.8),
        );
        let itext = d.gui_icon_text(ICON_INFO, Some(rstr!("Program Info")));
        let itext = CString::new(itext).unwrap();

        let proj_info_str = format!("Version: {SHORT_PROJECT_VERSION}");
        let proj_info_txt_bounds = font.measure_text(&proj_info_str, current_font_height as f32, default_txt_spacing as f32);
        let proj_info_str_y = d.get_screen_height() / 2 - current_font_height / 2;

        // No multiline text.
        let proj_name_str1 = format!("esox - Strumento per il calcolo");
        let proj_name_str2 = format!("NISECI e HFBI");

        let proj_name_str1_txt_bounds = font.measure_text(&proj_name_str1, current_font_height as f32, default_txt_spacing as f32);
        let proj_name_str2_txt_bounds = font.measure_text(&proj_name_str2, current_font_height as f32, default_txt_spacing as f32);

        let proj_name_str1_y = proj_info_str_y + (proj_info_txt_bounds.y as i32 * 2);
        let proj_name_str2_y = proj_name_str1_y + proj_name_str1_txt_bounds.y as i32;

        let widest_line = std::cmp::max(proj_info_txt_bounds.x as i32, std::cmp::max(proj_name_str1_txt_bounds.x as i32, proj_name_str2_txt_bounds.x as i32));

        let infobox_height = propheight(&d, 100) + proj_info_txt_bounds.y as i32 + proj_name_str1_txt_bounds.y as i32 + proj_name_str2_txt_bounds.y as i32;
        let infobox_y = d.get_screen_height() / 2 - infobox_height / 2;
        let infobox_width = propwidth(&d, 50) + widest_line as i32;
        let infobox_x = d.get_screen_width() / 2 - infobox_width / 2;
        let result = d.gui_window_box(
            rrect(
                infobox_x,
                infobox_y,
                infobox_width,
                infobox_height
            ),
            Some(itext.as_c_str()),
        );

        d.draw_text_ex(
            font,
            &proj_info_str,
            Vector2::new(
                (d.get_screen_width() / 2 - proj_info_txt_bounds.x as i32 / 2) as f32,
                proj_info_str_y as f32,
            ),
            current_font_height as f32,
            default_txt_spacing as f32,
            default_txt_color
        );

        d.draw_text_ex(
            font,
            &proj_name_str1,
            Vector2::new(
                (d.get_screen_width() / 2 - proj_name_str1_txt_bounds.x as i32 / 2) as f32,
                proj_name_str1_y as f32,
            ),
            current_font_height as f32,
            default_txt_spacing as f32,
            default_txt_color
        );

        d.draw_text_ex(
            font,
            &proj_name_str2,
            Vector2::new(
                (d.get_screen_width() / 2 - proj_name_str2_txt_bounds.x as i32 / 2) as f32,
                proj_name_str2_y as f32,
            ),
            current_font_height as f32,
            default_txt_spacing as f32,
            default_txt_color
        );

        if result == true {
            *showing_info_box = false;
        }
    }
}

pub fn draw_settings_box(d: &mut RaylibDrawHandle, main_state : &mut MainState) {
    if main_state.showing_settings_box {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::RAYWHITE.alpha(0.8),
        );
        let itext = d.gui_icon_text(ICON_GEAR, Some(rstr!("Impostazioni")));
        let itext = CString::new(itext).unwrap();
        let settingsbox_width = propwidth(&d, 250);
        let settingsbox_x =  d.get_screen_width() / 2 - settingsbox_width / 2;
        let settingsbox_height = propheight(&d, 300);
        let settingsbox_y = d.get_screen_height() / 2 - settingsbox_height / 2;
        let result = d.gui_window_box(
            rrect(
                settingsbox_x,
                settingsbox_y,
                settingsbox_width,
                settingsbox_height
            ),
            Some(itext.as_c_str()),
        );
        let y_spacing = propheight(&d, 15);
        let x_spacing = propwidth(&d, 15);
        let fontsize_label_width = settingsbox_width / 2 - x_spacing - x_spacing / 2;
        let fontsize_label_x = settingsbox_x + x_spacing;
        let fontsize_label_height = settingsbox_height / 10;
        let fontsize_label_y = settingsbox_y + y_spacing * 2;
        if d.gui_label(
            rrect(
                fontsize_label_x,
                fontsize_label_y,
                fontsize_label_width,
                fontsize_label_height
            ),
            Some(rstr!("Dimensione Font"))
        ) { }
        let fontspinner_x = fontsize_label_x + fontsize_label_width + x_spacing;
        let fontspinner_y = fontsize_label_y;
        let fontspinner_width = fontsize_label_width;
        let fontspinner_height = fontsize_label_height;
        let mut curr_font_height = main_state.current_font_height;
        if d.gui_spinner(
            rrect(
                fontspinner_x,
                fontspinner_y,
                fontspinner_width,
                fontspinner_height
            ),
            None,
            &mut curr_font_height,
            1,
            128,
            false,
        ) {
            println!("HI");
            //main_state.spinner_font_height_edit_mode = !main_state.spinner_font_height_edit_mode;
        }

        if curr_font_height != main_state.current_font_height {
            //Detecting this and acting here is better than doing so in
            //update_main() since we can avoid a hot call on gui_set_style()
            main_state.current_font_height = curr_font_height;
            d.gui_set_style(DEFAULT, TEXT_SIZE as i32, main_state.current_font_height);
        }

        let gui_theme_label_width = fontsize_label_width;
        let gui_theme_label_x = fontsize_label_x;
        let gui_theme_label_height = fontsize_label_height;
        let gui_theme_label_y = fontsize_label_y + y_spacing *2;
        if d.gui_label(
            rrect(
                gui_theme_label_x,
                gui_theme_label_y,
                gui_theme_label_width,
                gui_theme_label_height
            ),
            Some(rstr!("Tema"))
        ) { }
        let gui_theme_button_x = gui_theme_label_x + gui_theme_label_width + x_spacing;
        let gui_theme_button_y = gui_theme_label_y;
        let gui_theme_button_width = gui_theme_label_width;
        let gui_theme_button_height = gui_theme_label_height;

        let gui_theme_cstr = CString::new(GUI_THEME_COMBOBOX_STR).unwrap();

        d.gui_combo_box(
            rrect(
                gui_theme_button_x,
                gui_theme_button_y,
                gui_theme_button_width,
                gui_theme_button_height),
            Some(gui_theme_cstr.as_c_str()),
            &mut main_state.gui_theme_combobox_active
        );

        // Reset settings button
        if d.gui_button(rrect(fontsize_label_x, fontsize_label_y + fontsize_label_height * 3, fontsize_label_width, fontsize_label_height), Some(rstr!("Reset"))) {
            main_state.current_font_height = main_state.default_font_height;
            d.gui_set_style(DEFAULT, TEXT_SIZE as i32, main_state.current_font_height);
            main_state.gui_theme_combobox_active = GuiTheme::Light as i32;
        }

        if result == true {
            main_state.showing_settings_box = false;
        }
    }
}

pub fn draw_main(d : &mut RaylibDrawHandle, main_state : &mut MainState) {

    let current_view_name = main_state.current_view.to_string();
    let current_view_banner_x = propwidth(&d, 100);
    let current_view_banner_y = propheight(&d, 25);

    let text_spacing = d.gui_get_style(DEFAULT, TEXT_SPACING as i32);
    d.draw_text_ex(
        &main_state.current_font,
        &current_view_name,
        Vector2::new(current_view_banner_x as f32, current_view_banner_y as f32),
        (main_state.current_font_height) as f32,
        text_spacing as f32,
        main_state.default_txt_color.alpha(0.8)
    );

    let info_button_width = propwidth(&d, 50);
    let info_button_x = propwidth(&d, 5);
    let info_button_height = propwidth(&d, 50);
    let info_button_y = propheight(&d, 5);

    let y_spacing = propheight(&d, 5);

    // Info button
    let itext = d.gui_icon_text(ICON_INFO, Some(rstr!("")));
    let itext = CString::new(itext).unwrap();
    if d.gui_button(rrect(info_button_x, info_button_y, info_button_width, info_button_height), Some(itext.as_c_str())) {
        main_state.showing_info_box = true;
    }

    let changeview_button_width = info_button_width;
    let changeview_button_x = info_button_x;
    let changeview_button_height = info_button_height;
    let changeview_button_y = info_button_y + info_button_height + y_spacing;

    // "Change view" button
    let itext = d.gui_icon_text(ICON_PLAYER_NEXT, Some(rstr!("")));
    let itext = CString::new(itext).unwrap();
    if d.gui_button(rrect(changeview_button_x, changeview_button_y, changeview_button_width, changeview_button_height), Some(itext.as_c_str())) {
        match main_state.current_view {
            CurrentView::HOME => {
                main_state.current_view = CurrentView::SECOND;
            }
            CurrentView::SECOND => {
                main_state.current_view = CurrentView::SelezioneIndice;
            }
            CurrentView::SelezioneIndice => {
                main_state.current_view = CurrentView::SelezioneFileInput;
            }
            CurrentView::SelezioneFileInput => {
                main_state.current_view = CurrentView::ValidazioneFileInput;
            }
            CurrentView::ValidazioneFileInput => {
                main_state.current_view = CurrentView::SelezioneInfoAggiuntive;
            }
            CurrentView::SelezioneInfoAggiuntive => {
                main_state.current_view = CurrentView::ValidazioneInfoAggiuntive;
            }
            CurrentView::ValidazioneInfoAggiuntive => {
                main_state.current_view = CurrentView::ProduzioneOutput;
            }
            CurrentView::ProduzioneOutput => {
                main_state.current_view = CurrentView::ProduzionePDF;
            }
            CurrentView::ProduzionePDF => {
                main_state.current_view = CurrentView::HOME;
            }
        }
    }

    let settings_button_width = changeview_button_width;
    let settings_button_x = changeview_button_x;
    let settings_button_height = changeview_button_height;
    let settings_button_y = changeview_button_y + changeview_button_height + y_spacing;

    // Settings button
    let itext = d.gui_icon_text(ICON_GEAR, Some(rstr!("")));
    let itext = CString::new(itext).unwrap();
    if d.gui_button(rrect(settings_button_x, settings_button_y, settings_button_width,  settings_button_height), Some(itext.as_c_str())) {
        main_state.showing_settings_box = true;
    }

    draw_settings_box(d, main_state);
    draw_info_box(d, &mut main_state.showing_info_box, &main_state.current_font, main_state.default_txt_spacing, main_state.default_txt_color, main_state.current_font_height);
    draw_quit_win(d, &mut main_state.showing_quit_win, &mut main_state.should_quit);
}
