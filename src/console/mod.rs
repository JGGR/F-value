use std::collections::VecDeque;
use raylib::prelude::*;
use crate::{propwidth, propheight};
use crate::SHORT_PROJECT_VERSION;


#[derive(Clone)]
pub struct Console {
    columns: usize,             // How many chars are shown per line
    messages: VecDeque<String>, // Stores all console messages
    max_messages: usize,        // Limit on messages to keep in memory
    view_offset: usize,         // Offset for the currently visible messages
    max_lines_visible: usize,   // Number of lines that fit in the view
    autoscroll: bool,           // Flag to track autoscroll state
    prompt : String,            // User prompt
}

impl Console {
    pub fn new(columns: usize, max_messages: usize, max_lines_visible: usize) -> Self {
        Console {
            columns: columns,
            messages: VecDeque::with_capacity(max_messages),
            max_messages,
            view_offset: 0,
            max_lines_visible,
            autoscroll: true, // Start with autoscroll enabled
            prompt: String::new(),
        }
    }

    pub fn get_len(&self) -> usize {
        return self.messages.len();
    }

    fn scroll_to_bottom(&mut self) {
        self.view_offset = self.messages.len().saturating_sub(self.max_lines_visible);
    }

    fn is_at_bottom(&self) -> bool {
        self.view_offset == self.messages.len().saturating_sub(self.max_lines_visible)
    }

    pub fn add_message(&mut self, msg: String) {
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
    pub fn handle_input(&mut self, _rl: &RaylibHandle, input_char: Option<char>, is_enter_pressed: bool, is_backspace_pressed: bool) {
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
                    self.add_message(format!("esox prompt, comandi disponibili:\n  echo\n  info\n  clear\n  help"));
                }
                "echo" => {
                    self.add_message(args);
                }
                "info" => {
                    if args_num < 1 {
                        self.add_message(format!("info: missing argument"));
                        self.add_message(format!("usage: info <name>"));
                    } else {
                        let name = args_vec[0];
                        match name {
                            "version" => {
                                self.add_message(format!("esox v{SHORT_PROJECT_VERSION}"));
                            }
                            _ => {
                                self.add_message(format!("info: TODO: report on {name}"));
                            }
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

    pub fn scroll_up(&mut self, lines: usize) {
        self.view_offset = self.view_offset.saturating_sub(lines);
        self.autoscroll = false; // Disable autoscroll when user scrolls up
    }

    pub fn scroll_down(&mut self, lines: usize) {
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

    pub fn draw(&self, d: &mut RaylibDrawHandle, txt_color: Color, font_size: i32, font: &Font) {
        let line_height = propheight(&d, font_size + 4); // Adjust as needed
        let console_height = (self.max_lines_visible +1) * line_height as usize; // +1 for user
                                                                                 // prompt

        let top_y_padding = propheight(&d, 50);
        let console_start_y = top_y_padding; //propheight(&d, screen_height - console_height as i32);

        let txt_left_x_padding = propwidth(&d, 10);

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
                Vector2::new(txt_left_x_padding as f32, (console_start_y + (i as i32 * line_height)) as f32),
                font_size as f32,
                0.0,
                txt_color
            );
        }

        let prompt_y_padding = propheight(&d, 10);
        let prompt_y = console_start_y + console_height as i32 + prompt_y_padding - line_height/2;
        let prompt_color = txt_color;

        // Draw the prompt at the bottom of the console
        d.draw_text_ex(
            font,
            &format!("> {}", self.prompt),
            Vector2::new(txt_left_x_padding as f32, prompt_y as f32),
            font_size as f32,
            0.0,
            prompt_color
        );
    }
}
