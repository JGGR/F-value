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

use crate::app::core::{propheight, propwidth, Action, Action::*};
use crate::core::rrect;
use raylib::consts::GuiIconName::ICON_MONITOR;
use raylib::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub(crate) struct Console {
    columns: usize,               // How many chars are shown per line
    messages: VecDeque<String>,   // Stores all console messages
    max_messages: usize,          // Limit on messages to keep in memory
    view_offset: usize,           // Offset for the currently visible messages
    max_lines_visible: usize,     // Number of lines that fit in the view
    autoscroll: bool,             // Flag to track autoscroll state
    prompt: String,               // User prompt
    env: HashMap<String, String>, // Console environment
}

impl Console {
    pub(crate) fn new(
        columns: usize,
        max_messages: usize,
        max_lines_visible: usize,
        env: HashMap<String, String>,
    ) -> Self {
        Console {
            columns,
            messages: VecDeque::with_capacity(max_messages),
            max_messages,
            view_offset: 0,
            max_lines_visible,
            autoscroll: true, // Start with autoscroll enabled
            prompt: String::new(),
            env,
        }
    }

    pub(crate) fn set_env(&mut self, (key, val): (String, String)) {
        self.env.insert(key, val);
    }

    pub(crate) fn remove_env(&mut self, key: String) -> Option<String> {
        self.env.remove(&key)
    }

    pub(crate) fn reset(&mut self) {
        self.messages.clear();
        self.set_env(("riferimento_niseci".to_string(), "Vuoto".to_string()));
        self.set_env(("campionamento_niseci".to_string(), "Vuoto".to_string()));
        self.set_env(("anagrafica_niseci".to_string(), "Vuoto".to_string()));
        self.set_env(("risultato_niseci".to_string(), "Vuoto".to_string()));
        self.set_env(("campionamento_hfbi".to_string(), "Vuoto".to_string()));
        self.set_env(("anagrafica_hfbi".to_string(), "Vuoto".to_string()));
        self.set_env(("risultato_hfbi".to_string(), "Vuoto".to_string()));
    }

    pub(crate) fn _get_len(&self) -> usize {
        self.messages.len()
    }

    fn scroll_to_bottom(&mut self) {
        self.view_offset = self.messages.len().saturating_sub(self.max_lines_visible);
    }

    fn _is_at_bottom(&self) -> bool {
        self.view_offset == self.messages.len().saturating_sub(self.max_lines_visible)
    }

    pub(crate) fn add_message(&mut self, msg: String) {
        let lines = msg.lines();

        for line in lines {
            let chunk_size = self.columns;

            let chunks: Vec<String> = line
                .chars()
                .collect::<Vec<_>>() // Collect into a vector of chars
                .chunks(chunk_size) // Split into chunks
                .map(|chunk| chunk.iter().collect::<String>()) // Convert each chunk to a String
                .collect();

            for chunk in chunks {
                if self.messages.len() == self.max_messages {
                    self.messages.pop_front();
                }
                self.messages.push_back(chunk);

                // Automatically adjust view if autoscroll is enabled
                if self.autoscroll {
                    self.scroll_to_bottom();
                }
            }
        }
    }

    /// Handle character input (e.g., from `raylib` key events)
    pub(crate) fn handle_input(
        &mut self,
        _rl: &RaylibHandle,
        input_char: Option<char>,
        is_enter_pressed: bool,
        is_backspace_pressed: bool,
    ) {
        if let Some(c) = input_char {
            self.prompt.push(c);
        }

        // Handle backspace key
        if is_backspace_pressed {
            self.prompt.pop();
        }

        // Handle enter key
        if is_enter_pressed {
            let user_prompt = self.prompt.clone();

            let mut parts = user_prompt.splitn(2, char::is_whitespace); // Split only once at the first whitespace

            let command;
            if let Some(cmd) = parts.next() {
                command = cmd.to_string();
            } else {
                command = "".to_string();
            }

            let args;
            if let Some(a) = parts.next() {
                args = a.to_string();
            } else {
                args = "".to_string();
            }

            let args_split = args.split_whitespace();
            let mut args_vec = Vec::<&str>::new();

            let mut args_num = 0;

            for arg in args_split {
                args_vec.push(arg);
                args_num += 1;
            }

            match command.as_str() {
                "help" => {
                    if args_num < 1 {
                        self.add_message(
                            "F-value prompt, comandi disponibili:\n  echo\n  info\n  clear\n  help"
                                .to_string(),
                        );
                    } else {
                        let cmd = args_vec[0];
                        match cmd {
                            "info" => {
                                self.add_message(
                                    "comando info:\n  uso: info <name>\nNomi disponibili: {"
                                        .to_string(),
                                );
                                let keys: Vec<_> = self.env.keys().cloned().collect();
                                for k in keys {
                                    self.add_message(format!("  {k}"));
                                }
                                self.add_message("}".to_string());
                            }
                            "clear" => {
                                self.add_message("comando clear:\n  uso: clear".to_string());
                            }
                            "echo" => {
                                self.add_message(
                                    "comando echo:\n  uso: echo <args...>".to_string(),
                                );
                            }
                            "help" => {
                                self.add_message(
                                    "comando help:\n  uso: help <command>\nComandi disponibili: {\n  echo\n  info\n  clear\n  help\n}"
                                        .to_string(),
                                );
                            }
                            _ => {
                                self.add_message(format!("help: Comando sconosciuto: {cmd}"));
                            }
                        }
                    }
                }
                "echo" => {
                    self.add_message(args);
                }
                "info" => {
                    if args_num < 1 {
                        self.add_message("info: missing argument".to_string());
                        self.add_message("usage: info <name>".to_string());
                        self.add_message("for available names: help info".to_string());
                    } else {
                        let name = args_vec[0].to_string();

                        let keys: Vec<_> = self.env.keys().cloned().collect();

                        if keys.contains(&name) {
                            let val = self.env.get(&name).unwrap();
                            self.add_message(format!("info: {name}: {{{val}}}"));
                        } else {
                            self.add_message(format!("info: Nome sconosciuto: {name}"));
                            self.add_message("for available names: help info".to_string());
                        }
                    }
                }
                "clear" => {
                    self.messages.clear();
                }
                _ => {
                    self.add_message(format!("Unknown command: {command}"));
                    self.add_message("Run \"help\" for a list of available commands".to_string());
                }
            }
            self.prompt.clear();
        }
    }

    pub(crate) fn scroll_up(&mut self, lines: usize) {
        self.view_offset = self.view_offset.saturating_sub(lines);
        self.autoscroll = false; // Disable autoscroll when user scrolls up
    }

    pub(crate) fn scroll_down(&mut self, lines: usize) {
        // Ensure we don't scroll beyond the available messages
        let max_offset = self.messages.len().saturating_sub(self.max_lines_visible);

        if self.view_offset.saturating_add(lines) <= max_offset {
            self.view_offset = self.view_offset.saturating_add(lines);
        } else {
            self.view_offset = max_offset;
        }

        // If scrolling reaches the bottom, re-enable autoscroll
        if self.view_offset == max_offset {
            self.autoscroll = true;
        }
    }

    pub(crate) fn draw(
        &self,
        d: &mut RaylibDrawHandle,
        txt_color: Color,
        font_size: i32,
        font_spacing: i32,
        font: &Font,
    ) -> Vec<Action> {
        let mut actions = Vec::<Action>::new();
        let line_height = propheight(d, font_size + 4); // Adjust as needed
        let console_height = (self.max_lines_visible + 1) * line_height as usize; // +1 for user
                                                                                  // prompt
        let monospaced_width = font
            .measure_text("w", font_size as f32, font_spacing as f32)
            .x;
        let console_width = monospaced_width as i32 * self.columns as i32;

        let top_y_padding = propheight(d, 50);
        let console_start_y = top_y_padding; //propheight(&d, screen_height - console_height as i32);
        let txt_left_x_padding = propwidth(d, 10);

        // Using txt_color
        d.draw_rectangle_lines(
            txt_left_x_padding,
            console_start_y,
            console_width,
            console_height as i32,
            txt_color,
        );

        let sidebox_x_padding = txt_left_x_padding;
        let sidebox_x = txt_left_x_padding + console_width + sidebox_x_padding;
        let sidebox_width = d.get_screen_width() - sidebox_x - sidebox_x_padding;
        let sidebox_y = console_start_y;
        let sidebox_height = console_height as i32;

        // Using txt_color
        d.draw_rectangle_lines(
            sidebox_x,
            sidebox_y,
            sidebox_width,
            sidebox_height,
            txt_color,
        );

        let userinfo_y_padding = propheight(d, 75);
        // Backout button
        let backout_itext = d.gui_icon_text(ICON_MONITOR, ": Indietro");
        let backout_x = sidebox_x + sidebox_x_padding;
        let backout_y = userinfo_y_padding;
        let backout_width = sidebox_width - sidebox_x_padding * 2;
        let backout_height = propheight(d, 30);

        if d.gui_button(
            rrect(backout_x, backout_y, backout_width, backout_height),
            backout_itext.as_str(),
        ) {
            actions.push(ConsoleBackout);
        }

        for (i, line) in self
            .messages
            .iter()
            .skip(self.view_offset)
            .take(self.max_lines_visible)
            .enumerate()
        {
            d.draw_text_ex(
                font,
                line,
                Vector2::new(
                    txt_left_x_padding as f32,
                    (console_start_y + (i as i32 * line_height)) as f32,
                ),
                font_size as f32,
                0.0,
                txt_color,
            );
        }

        let prompt_y_padding = propheight(d, 10);
        let prompt_y = console_start_y + console_height as i32 + prompt_y_padding - line_height / 2;
        let prompt_color = txt_color;

        // Draw the prompt at the bottom of the console
        d.draw_text_ex(
            font,
            &format!("> {}", self.prompt),
            Vector2::new(txt_left_x_padding as f32, prompt_y as f32),
            font_size as f32,
            0.0,
            prompt_color,
        );
        actions
    }
}
