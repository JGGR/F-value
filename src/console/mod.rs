use std::collections::VecDeque;
use raylib::prelude::*;
use crate::{propwidth, propheight};


#[derive(Clone)]
pub struct Console {
    messages: VecDeque<String>, // Stores all console messages
    max_messages: usize,        // Limit on messages to keep in memory
    view_offset: usize,         // Offset for the currently visible messages
    max_lines_visible: usize,   // Number of lines that fit in the view
    autoscroll: bool,           // Flag to track autoscroll state
    prompt : String,            // User prompt
}

fn wrap_text(rl: &RaylibHandle, text: &str, max_width: i32, font_size: i32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        let new_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        if rl.measure_text(&new_line, font_size) > max_width {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = new_line;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

impl Console {
    pub fn new(max_messages: usize, max_lines_visible: usize) -> Self {
        Console {
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

    pub fn add_message(&mut self, rl: &RaylibHandle, msg: String, max_width: i32, font_size: i32) {
        let wrapped_lines = wrap_text(rl, &msg, max_width, font_size);
        for line in wrapped_lines {
            if self.messages.len() == self.max_messages {
                self.messages.pop_front();
            }
            self.messages.push_back(line);

            // Automatically adjust view if autoscroll is enabled
            if self.autoscroll {
                self.scroll_to_bottom();
            }
        }
    }

    /// Handle character input (e.g., from `raylib` key events)
    pub fn handle_input(&mut self, rl: &RaylibHandle, input_char: Option<char>, current_font_size: i32, is_enter_pressed: bool, is_backspace_pressed: bool) {
        if let Some(c) = input_char {
            self.prompt.push(c);
        }

        // Handle backspace key
        if is_backspace_pressed {
            self.prompt.pop();
        }

        // Handle enter key
        if is_enter_pressed {
            self.add_message(rl, self.prompt.clone(), 780, current_font_size);
            self.prompt.clear();
        }
    }

    pub fn scroll_up(&mut self, lines: usize) {
        self.view_offset = self.view_offset.saturating_sub(lines);
        self.autoscroll = false; // Disable autoscroll when user scrolls up
    }

    pub fn scroll_down(&mut self, lines: usize) {
        self.view_offset = (self.view_offset + lines).min(self.messages.len());
        // If scrolling reaches the bottom, re-enable autoscroll
        if self.view_offset == self.messages.len().saturating_sub(self.max_lines_visible) {
            self.autoscroll = true;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, font_size: i32, _screen_width: i32, screen_height: i32) {
        let line_height = propheight(&d, font_size + 4); // Adjust as needed
        let console_height = self.max_lines_visible * line_height as usize;

        let start_y = propheight(&d, screen_height - console_height as i32);

        for (i, line) in self
            .messages
            .iter()
            .skip(self.view_offset)
            .take(self.max_lines_visible)
            .enumerate()
        {
            d.draw_text(
                line,
                propwidth(&d, 10),
                start_y + (i as i32 * line_height),
                font_size,
                Color::WHITE,
            );
        }

         // Draw the prompt at the bottom of the console
        d.draw_text(
            &format!("> {}", self.prompt),
            propwidth(&d, 10),
            screen_height - line_height,
            font_size,
            Color::YELLOW,
        );
    }
}
