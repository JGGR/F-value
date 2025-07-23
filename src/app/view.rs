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

use super::core::{
    propheight, propwidth, CurrentView, GuiTheme, MainState, GUI_THEME_COMBOBOX_STR,
};
use crate::core::{
    rrect, AUTHOR_GIONINJO, AUTHOR_GIONINJO_LINK, AUTHOR_JGABAUT, AUTHOR_JGABAUT_LINK,
    COPYRIGHT_INFO, SHORT_PROJECT_VERSION,
};
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiDefaultProperty::TEXT_SIZE;
use raylib::consts::GuiIconName::*;
use raylib::prelude::*;

pub(crate) fn draw_reset_win(
    d: &mut RaylibDrawHandle,
    showing_reset_win: &mut bool,
    should_reset: &mut bool,
) {
    if *showing_reset_win {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::RAYWHITE.alpha(0.8),
        );
        let itext = d.gui_icon_text(ICON_EXIT, "Reset");
        let result = d.gui_message_box(
            rrect(
                d.get_screen_width() / 2 - propwidth(d, 125),
                d.get_screen_height() / 2 - propheight(d, 50),
                propwidth(d, 250),
                propheight(d, 100),
            ),
            itext.as_str(),
            "Vuoi davvero resettare?",
            "Si;No",
        );

        if (result == 0) || (result == 2) {
            *showing_reset_win = false;
        } else if result == 1 {
            *should_reset = true;
            *showing_reset_win = false;
        }
    }
}

pub(crate) fn draw_quit_win(
    d: &mut RaylibDrawHandle,
    showing_quit_win: &mut bool,
    should_quit: &mut bool,
) {
    if *showing_quit_win {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::RAYWHITE.alpha(0.8),
        );
        let itext = d.gui_icon_text(ICON_EXIT, "Close Window");
        let result = d.gui_message_box(
            rrect(
                d.get_screen_width() / 2 - propwidth(d, 125),
                d.get_screen_height() / 2 - propheight(d, 50),
                propwidth(d, 250),
                propheight(d, 100),
            ),
            itext.as_str(),
            "Vuoi davvero uscire?",
            "Si;No",
        );

        if (result == 0) || (result == 2) {
            *showing_quit_win = false;
        } else if result == 1 {
            *should_quit = true;
        }
    }
}

pub(crate) fn draw_license_box(
    d: &mut RaylibDrawHandle,
    showing_license_box: &mut bool,
    font: &WeakFont,
    default_txt_spacing: i32,
    current_font_height: i32,
) {
    if *showing_license_box {
        let x_padding = propwidth(d, 50);
        let y_padding = propheight(d, 100);
        let bar_height = 23; // Height of the "x" bar
        let copyright_notice_txt_bounds = font.measure_text(
            COPYRIGHT_INFO,
            current_font_height as f32,
            default_txt_spacing as f32,
        );
        let licensebox_height = copyright_notice_txt_bounds.y as i32 + bar_height;
        let licensebox_y = d.get_screen_height() / 2 - licensebox_height / 2;
        let licensebox_width = x_padding * 2 + copyright_notice_txt_bounds.x as i32;
        let licensebox_x = d.get_screen_width() / 2 - licensebox_width / 2;
        let result = d.gui_window_box(
            rrect(
                licensebox_x,
                licensebox_y,
                licensebox_width,
                licensebox_height,
            ),
            "License",
        );

        let copyright_label_width = copyright_notice_txt_bounds.x as i32;
        let copyright_label_x = licensebox_x + x_padding;
        let copyright_label_y = licensebox_y + y_padding;
        let copyright_label_height = copyright_notice_txt_bounds.y as i32;

        let copyright_label = COPYRIGHT_INFO;

        d.gui_label(
            rrect(
                copyright_label_x,
                copyright_label_y,
                copyright_label_width,
                copyright_label_height,
            ),
            copyright_label,
        );

        if result {
            *showing_license_box = false;
        }
    }
}

pub(crate) fn draw_info_box(
    d: &mut RaylibDrawHandle,
    showing_info_box: &mut bool,
    font: &WeakFont,
    default_txt_spacing: i32,
    default_txt_color: Color,
    current_font_height: i32,
) {
    if *showing_info_box {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::RAYWHITE.alpha(0.8),
        );

        let bar_height = propheight(d, 23); // Height of the "x" bar

        let itext = d.gui_icon_text(ICON_INFO, "Program Info");

        let proj_info_str = format!("esox {SHORT_PROJECT_VERSION}");
        let proj_info_txt_bounds = font.measure_text(
            &proj_info_str,
            current_font_height as f32,
            default_txt_spacing as f32,
        );
        //let proj_info_str_y = propheight(&d, 100) - current_font_height / 2;

        // No multiline text.
        let proj_name_str1 = "Strumento per il calcolo".to_string();
        let proj_name_str2 = "NISECI e HFBI".to_string();

        let proj_name_str1_txt_bounds = font.measure_text(
            &proj_name_str1,
            current_font_height as f32,
            default_txt_spacing as f32,
        );
        let proj_name_str2_txt_bounds = font.measure_text(
            &proj_name_str2,
            current_font_height as f32,
            default_txt_spacing as f32,
        );

        //let proj_name_str1_y = proj_info_str_y + (proj_info_txt_bounds.y as i32 * 2);
        //let proj_name_str2_y = proj_name_str1_y + proj_name_str1_txt_bounds.y as i32;

        let copyright_display_link = "Copyright (C) 2024-2025 jgabaut, gioninjo";
        let copyright_actual_link = "https://spdx.org/licenses/GPL-3.0-only.html";

        let copyright_display_link_txt_bounds = font.measure_text(
            copyright_display_link,
            current_font_height as f32,
            default_txt_spacing as f32,
        );

        let widest_line_x_bound: i32 = copyright_display_link_txt_bounds.x as i32;

        let infobox_height = propheight(d, 200)
            + proj_info_txt_bounds.y as i32
            + proj_name_str1_txt_bounds.y as i32
            + proj_name_str2_txt_bounds.y as i32;
        let infobox_y = d.get_screen_height() / 2 - infobox_height / 2;
        let infobox_width = propwidth(d, 100) + widest_line_x_bound;
        let infobox_x = d.get_screen_width() / 2 - infobox_width / 2;
        let result = d.gui_window_box(
            rrect(infobox_x, infobox_y, infobox_width, infobox_height),
            itext.as_str(),
        );

        let text_y_spacing = propheight(d, 12);
        let text_x_spacing = propwidth(d, 120);
        let proj_info_str_y = infobox_y + bar_height + text_y_spacing;
        let proj_info_str_x = infobox_x + text_x_spacing;
        let proj_name_str1_y = proj_info_str_y + (proj_info_txt_bounds.y as i32 * 2);
        let proj_name_str1_x = proj_info_str_x;
        let proj_name_str2_y = proj_name_str1_y + proj_name_str1_txt_bounds.y as i32;
        let proj_name_str2_x = proj_name_str1_x;

        d.draw_text_ex(
            font,
            &proj_info_str,
            Vector2::new(proj_info_str_x as f32, proj_info_str_y as f32),
            current_font_height as f32,
            default_txt_spacing as f32,
            default_txt_color,
        );

        d.draw_text_ex(
            font,
            &proj_name_str1,
            Vector2::new(proj_name_str1_x as f32, proj_name_str1_y as f32),
            current_font_height as f32,
            default_txt_spacing as f32,
            default_txt_color,
        );

        d.draw_text_ex(
            font,
            &proj_name_str2,
            Vector2::new(proj_name_str2_x as f32, proj_name_str2_y as f32),
            current_font_height as f32,
            default_txt_spacing as f32,
            default_txt_color,
        );

        let copyright_link_str = copyright_display_link;
        let copyright_link_x = infobox_x + propwidth(d, 10);
        let copyright_link_y =
            proj_name_str2_y + proj_name_str2_txt_bounds.y as i32 + text_y_spacing * 2;
        let copyright_link_width = infobox_width - text_x_spacing;
        let copyright_link_height = propheight(d, 25);

        if d.gui_label_button(
            rrect(
                copyright_link_x,
                copyright_link_y,
                copyright_link_width,
                copyright_link_height,
            ),
            copyright_link_str,
        ) {
            raylib::core::misc::open_url(copyright_actual_link);
        }

        let display_link = "Example: esox-website.it";
        let actual_link = "https://duckduckgo.com/?t=h_&q=esox+lucius";
        let link_str = display_link;
        let link_x = proj_name_str2_x;
        let link_y = copyright_link_y + copyright_link_height;
        let link_width = infobox_width - text_x_spacing;
        let link_height = propheight(d, 25);

        d.gui_label(
            rrect(
                infobox_x + propwidth(d, 10),
                link_y,
                text_x_spacing,
                link_height,
            ),
            "Info:",
        );

        if d.gui_label_button(rrect(link_x, link_y, link_width, link_height), link_str) {
            raylib::core::misc::open_url(actual_link);
        }

        let support_email = "mario@rossi.it";
        let mail_display_link = "Example: ".to_owned() + support_email;
        let mail_actual_link = "mailto:".to_owned() + support_email;
        let mail_link_str = mail_display_link;
        let mail_link_x = link_x;
        let mail_link_y = link_y + link_height;
        let mail_link_width = link_width;
        let mail_link_height = link_height;

        d.gui_label(
            rrect(
                infobox_x + propwidth(d, 10),
                mail_link_y,
                text_x_spacing,
                mail_link_height,
            ),
            "Support:",
        );

        if d.gui_label_button(
            rrect(mail_link_x, mail_link_y, mail_link_width, mail_link_height),
            mail_link_str.as_str(),
        ) {
            raylib::core::misc::open_url(&mail_actual_link);
        }

        let author_display_links: Vec<String> =
            vec![format!("{AUTHOR_JGABAUT}"), format!("{AUTHOR_GIONINJO}")];
        let author_actual_links: Vec<String> = vec![
            format!("{AUTHOR_JGABAUT_LINK}"),
            format!("{AUTHOR_GIONINJO_LINK}"),
        ];
        let author_links_str: Vec<String> = author_display_links.clone();
        let author_display_links_width: Vec<i32> = vec![
            font.measure_text(
                &format!("{}, ", author_display_links[0]),
                current_font_height as f32,
                default_txt_spacing as f32,
            )
            .x as i32,
            font.measure_text(
                &format!("{}, ", author_display_links[1]),
                current_font_height as f32,
                default_txt_spacing as f32,
            )
            .x as i32,
        ];

        let author_links_x: Vec<i32> = vec![
            link_x,
            link_x
                + font
                    .measure_text(
                        &format!("{}, ", author_display_links[0]),
                        current_font_height as f32,
                        default_txt_spacing as f32,
                    )
                    .x as i32,
        ];
        let author_links_y = mail_link_y + mail_link_height;
        let author_links_height = link_height;

        d.gui_label(
            rrect(
                infobox_x + propwidth(d, 10),
                author_links_y,
                text_x_spacing,
                author_links_height,
            ),
            "Authors:",
        );
        for (i, link_str) in author_links_str.iter().enumerate() {
            if d.gui_label_button(
                rrect(
                    author_links_x[i],
                    author_links_y,
                    author_display_links_width[i],
                    author_links_height,
                ),
                link_str.as_str(),
            ) {
                raylib::core::misc::open_url(&author_actual_links[i]);
            }
        }

        if result {
            *showing_info_box = false;
        }
    }
}

pub(crate) fn draw_settings_box(d: &mut RaylibDrawHandle, main_state: &mut MainState) {
    if main_state.showing_settings_box {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::RAYWHITE.alpha(0.8),
        );
        let itext = d.gui_icon_text(ICON_GEAR, "Impostazioni");
        let settingsbox_width = propwidth(d, 250);
        let settingsbox_x = d.get_screen_width() / 2 - settingsbox_width / 2;
        let settingsbox_height = propheight(d, 300);
        let settingsbox_y = d.get_screen_height() / 2 - settingsbox_height / 2;
        let result = d.gui_window_box(
            rrect(
                settingsbox_x,
                settingsbox_y,
                settingsbox_width,
                settingsbox_height,
            ),
            itext.as_str(),
        );
        let y_spacing = propheight(d, 15);
        let x_spacing = propwidth(d, 15);
        let fontsize_label_width = settingsbox_width / 2 - x_spacing - x_spacing / 2;
        let fontsize_label_x = settingsbox_x + x_spacing;
        let fontsize_label_height = settingsbox_height / 10;
        let fontsize_label_y = settingsbox_y + y_spacing * 2;
        d.gui_label(
            rrect(
                fontsize_label_x,
                fontsize_label_y,
                fontsize_label_width,
                fontsize_label_height,
            ),
            "Dimensione Font",
        );
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
                fontspinner_height,
            ),
            "",
            &mut curr_font_height,
            1,
            128,
            false,
        ) {
            //println!("HI");
            //main_state.spinner_font_height_edit_mode = !main_state.spinner_font_height_edit_mode;
        }

        if curr_font_height != main_state.current_font_height {
            //Detecting this and acting here is better than doing so in
            //update_main() since we can avoid a hot call on gui_set_style()
            main_state.current_font_height = curr_font_height;
            d.gui_set_style(DEFAULT, TEXT_SIZE, main_state.current_font_height);
        }

        let gui_theme_label_width = fontsize_label_width;
        let gui_theme_label_x = fontsize_label_x;
        let gui_theme_label_height = fontsize_label_height;
        let gui_theme_label_y = fontsize_label_y + y_spacing * 2;
        d.gui_label(
            rrect(
                gui_theme_label_x,
                gui_theme_label_y,
                gui_theme_label_width,
                gui_theme_label_height,
            ),
            "Tema",
        );
        let gui_theme_button_x = gui_theme_label_x + gui_theme_label_width + x_spacing;
        let gui_theme_button_y = gui_theme_label_y;
        let gui_theme_button_width = gui_theme_label_width;
        let gui_theme_button_height = gui_theme_label_height;

        let gui_theme_str = GUI_THEME_COMBOBOX_STR;

        d.gui_combo_box(
            rrect(
                gui_theme_button_x,
                gui_theme_button_y,
                gui_theme_button_width,
                gui_theme_button_height,
            ),
            gui_theme_str,
            &mut main_state.gui_theme_combobox_active,
        );

        // Reset settings button
        if d.gui_button(
            rrect(
                fontsize_label_x,
                fontsize_label_y + fontsize_label_height * 3,
                fontsize_label_width,
                fontsize_label_height,
            ),
            "Reset",
        ) {
            main_state.current_font_height = main_state.default_font_height;
            d.gui_set_style(DEFAULT, TEXT_SIZE, main_state.current_font_height);
            main_state.gui_theme_combobox_active = GuiTheme::Light as i32;
        }

        if result {
            main_state.showing_settings_box = false;
        }
    }
}

pub(crate) fn draw_main(d: &mut RaylibDrawHandle, main_state: &mut MainState) {
    let lock_gui = main_state.get_gui_should_lock();

    if lock_gui {
        d.gui_lock();
    }

    let status_bar_height = propheight(d, 35);
    let status_bar_width = d.get_screen_width();
    let status_bar_x = 0;
    let status_bar_y = d.get_screen_height() - status_bar_height;

    let current_view_name = main_state.current_view.to_string();

    let status_bar_txt = current_view_name;

    d.gui_status_bar(
        rrect(
            status_bar_x,
            status_bar_y,
            status_bar_width,
            status_bar_height,
        ),
        status_bar_txt.as_str(),
    );

    let navbar_height = status_bar_height;
    let navbar_width = status_bar_width;
    let navbar_x = 0;
    let navbar_y = 0;

    let core_button_width = propwidth(d, 25);
    let core_button_heigth = core_button_width;
    let core_buttons_count = 6;
    let core_buttons_x_padding = propwidth(d, 5);
    let core_buttons_y_padding = core_buttons_x_padding;
    let core_buttons_panel_height = navbar_height;
    let core_buttons_panel_y = navbar_y;
    let core_buttons_panel_width = (core_buttons_count * core_button_width)
        + ((1 + core_buttons_count) * core_buttons_x_padding);
    let core_buttons_panel_x = d.get_screen_width() - core_buttons_panel_width;

    // When raylib-rs 5.5.x includes the fix to handle ""
    // in gui_panel() as per 5.0.x, we could go back to using gui_panel instead
    let panels_line_thickness = 1.0;
    d.draw_rectangle_lines_ex(
        rrect(navbar_x, navbar_y, navbar_width, navbar_height),
        panels_line_thickness,
        main_state.default_txt_color,
    );
    d.draw_rectangle_lines_ex(
        rrect(
            core_buttons_panel_x,
            core_buttons_panel_y,
            core_buttons_panel_width,
            core_buttons_panel_height,
        ),
        panels_line_thickness,
        main_state.default_txt_color,
    );

    let info_button_width = core_button_width;
    let info_button_x = core_buttons_panel_x + core_buttons_x_padding;
    let info_button_height = core_button_heigth;
    let info_button_y = core_buttons_panel_y + core_buttons_y_padding;

    // Info button
    let itext = d.gui_icon_text(ICON_INFO, "");
    if d.gui_button(
        rrect(
            info_button_x,
            info_button_y,
            info_button_width,
            info_button_height,
        ),
        itext.as_str(),
    ) {
        main_state.showing_info_box = true;
    }

    let reset_button_width = core_button_width;
    let reset_button_x = info_button_x + info_button_width + core_buttons_x_padding;
    let reset_button_height = info_button_height;
    let reset_button_y = info_button_y;

    // Reset button
    let itext = d.gui_icon_text(ICON_RESTART, "");
    if d.gui_button(
        rrect(
            reset_button_x,
            reset_button_y,
            reset_button_width,
            reset_button_height,
        ),
        itext.as_str(),
    ) {
        main_state.showing_reset_win = true;
    }

    let changeview_button_width = reset_button_width;
    let changeview_button_x = reset_button_x + reset_button_width + core_buttons_x_padding;
    let changeview_button_height = reset_button_height;
    let changeview_button_y = reset_button_y;

    // "Change view" button
    let itext = d.gui_icon_text(ICON_PLAYER_NEXT, "");
    if d.gui_button(
        rrect(
            changeview_button_x,
            changeview_button_y,
            changeview_button_width,
            changeview_button_height,
        ),
        itext.as_str(),
    ) {
        match main_state.current_view {
            CurrentView::Home => {
                main_state.set_current_view(CurrentView::Second);
            }
            CurrentView::Second => {
                main_state.set_current_view(CurrentView::SelezioneIndice);
            }
            CurrentView::SelezioneIndice => {
                main_state.set_current_view(CurrentView::SelezioneFileInput);
            }
            CurrentView::SelezioneFileInput => {
                main_state.set_current_view(CurrentView::ValidazioneFileInput);
            }
            CurrentView::ValidazioneFileInput => {
                main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
            }
            CurrentView::SelezioneInfoAggiuntive => {
                main_state.set_current_view(CurrentView::ValidazioneInfoAggiuntive);
            }
            CurrentView::ValidazioneInfoAggiuntive => {
                main_state.set_current_view(CurrentView::ProduzioneOutput);
            }
            CurrentView::ProduzioneOutput => {
                main_state.set_current_view(CurrentView::ProduzionePDF);
            }
            CurrentView::ProduzionePDF => {
                main_state.set_current_view(CurrentView::Home);
            }
            _ => {}
        }
    }

    // License button
    let license_button_width = changeview_button_width;
    let license_button_x = changeview_button_x + changeview_button_width + core_buttons_x_padding;
    let license_button_height = changeview_button_height;
    let license_button_y = changeview_button_y;
    let itext = d.gui_icon_text(ICON_TEXT_NOTES, "");
    if d.gui_button(
        rrect(
            license_button_x,
            license_button_y,
            license_button_width,
            license_button_height,
        ),
        itext.as_str(),
    ) {
        main_state.showing_license_box = true;
    }

    let settings_button_width = license_button_width;
    let settings_button_x = license_button_x + license_button_width + core_buttons_x_padding;
    let settings_button_height = license_button_height;
    let settings_button_y = license_button_y;

    // Settings button
    let itext = d.gui_icon_text(ICON_GEAR, "");
    if d.gui_button(
        rrect(
            settings_button_x,
            settings_button_y,
            settings_button_width,
            settings_button_height,
        ),
        itext.as_str(),
    ) {
        main_state.showing_settings_box = true;
    }

    let console_button_width = settings_button_width;
    let console_button_x = settings_button_x + settings_button_width + core_buttons_x_padding;
    let console_button_height = settings_button_height;
    let console_button_y = settings_button_y;

    // "Console view" button
    let itext = d.gui_icon_text(ICON_MONITOR, "");
    if d.gui_button(
        rrect(
            console_button_x,
            console_button_y,
            console_button_width,
            console_button_height,
        ),
        itext.as_str(),
    ) {
        match main_state.current_view {
            CurrentView::Console => {
                let prev = main_state.previous_view;
                main_state.set_current_view(prev);
            }
            _ => {
                main_state.set_current_view(CurrentView::Console);
            }
        }
    }

    if lock_gui && main_state.showing_settings_box {
        d.gui_unlock();
    }
    draw_settings_box(d, main_state);
    if lock_gui && main_state.showing_settings_box {
        d.gui_lock();
    }

    if lock_gui && main_state.showing_info_box {
        d.gui_unlock();
    }
    draw_info_box(
        d,
        &mut main_state.showing_info_box,
        &main_state.current_font,
        main_state.default_txt_spacing,
        main_state.default_txt_color,
        main_state.current_font_height,
    );
    if lock_gui && main_state.showing_info_box {
        d.gui_lock();
    }

    if lock_gui && main_state.showing_license_box {
        d.gui_unlock();
    }
    draw_license_box(
        d,
        &mut main_state.showing_license_box,
        &main_state.current_font,
        main_state.default_txt_spacing,
        main_state.current_font_height,
    );
    if lock_gui && main_state.showing_license_box {
        d.gui_lock();
    }

    if lock_gui && main_state.showing_quit_win {
        d.gui_unlock();
    }
    draw_quit_win(
        d,
        &mut main_state.showing_quit_win,
        &mut main_state.should_quit,
    );
    if lock_gui && main_state.showing_quit_win {
        d.gui_lock();
    }

    if lock_gui && main_state.showing_reset_win {
        d.gui_unlock();
    }
    draw_reset_win(
        d,
        &mut main_state.showing_reset_win,
        &mut main_state.should_reset,
    );
    if lock_gui && main_state.showing_reset_win {
        d.gui_lock();
    }

    if lock_gui {
        d.gui_unlock();
    }
}
