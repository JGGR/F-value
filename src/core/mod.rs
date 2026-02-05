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

pub(crate) mod cli;
pub(crate) mod pdf;

pub(crate) const AUTHOR_JGABAUT: &str = "jgabaut";
pub(crate) const AUTHOR_GIONINJO: &str = "gioninjo";
pub(crate) const AUTHOR_GIONINJO_LINK: &str = "https://github.com/gioninjo";
pub(crate) const AUTHOR_JGABAUT_LINK: &str = "https://github.com/jgabaut";
pub(crate) const COPYRIGHT_INFO: &str = "Copyright (C) 2024-2025  jgabaut, gioninjo

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.";

pub(crate) const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const PROJECT_VERSION_FULL: &str = env!("VERSION_STRING");
pub(crate) const SHORT_PROJECT_VERSION: &str = env!("SHORT_VERSION_STRING");
pub(crate) const PROJECT_BUILD_TYPE: &str = env!("BUILD_TYPE");
pub(crate) const PROJECT_BRANCH: &str = env!("BRANCH_NAME");
pub(crate) const _COMMIT_HASH: &str = env!("COMMIT_HASH");
pub(crate) const COMMIT_HASH_PLUS: &str = env!("COMMIT_HASH_PLUS");
pub(crate) const BUILD_DATE: &str = env!("BUILD_DATE");

use chrono::Local;
use raylib::math::Rectangle;
use raylib::misc::AsF32;
use std::path::PathBuf;

/// A convenience function for making a new `Rectangle`.
#[inline]
pub fn rrect<T1: AsF32, T2: AsF32, T3: AsF32, T4: AsF32>(
    x: T1,
    y: T2,
    width: T3,
    height: T4,
) -> Rectangle {
    Rectangle::new(x.as_f32(), y.as_f32(), width.as_f32(), height.as_f32())
}

pub(crate) trait CommaFormat {
    fn comma(self) -> String;
}

impl CommaFormat for f32 {
    fn comma(self) -> String {
        let mut s = self.to_string();
        if let Some(pos) = s.find('.') {
            s.replace_range(pos..=pos, ",");
        }
        s
    }
}

/// This function does not handle Unicode validation.
pub fn sanitize_filename(input: &str) -> String {
    // Empty input → single underscore
    if input.is_empty() {
        return "_".to_string();
    }

    let mut out = String::with_capacity(input.len());

    for c in input.chars() {
        let valid = match c {
            '\0' => false,

            c if c.is_whitespace() => false,

            // path separators
            '/' => false,
            '\\' if cfg!(windows) => false,

            // Windows forbidden characters
            '<' | '>' | ':' | '"' | '|' | '?' | '*' if cfg!(windows) => false,

            // control characters
            c if c.is_control() => false,

            _ => true,
        };

        out.push(if valid { c } else { '_' });
    }

    #[cfg(windows)]
    {
        // Windows forbids trailing dots and spaces
        while out.ends_with('.') || out.ends_with(' ') {
            out.pop();
        }
    }

    if out.is_empty() {
        out.push('_');
    }

    // Disallow "." and ".." everywhere
    if out == "." || out == ".." {
        out.insert(0, '_');
    }

    #[cfg(windows)]
    {
        // Avoid reserved device names (CON, NUL, COM1, ...)
        let upper = out.to_ascii_uppercase();
        let base = upper.split('.').next().unwrap();

        if matches!(
            base,
            "CON"
                | "PRN"
                | "AUX"
                | "NUL"
                | "COM1"
                | "COM2"
                | "COM3"
                | "COM4"
                | "COM5"
                | "COM6"
                | "COM7"
                | "COM8"
                | "COM9"
                | "LPT1"
                | "LPT2"
                | "LPT3"
                | "LPT4"
                | "LPT5"
                | "LPT6"
                | "LPT7"
                | "LPT8"
                | "LPT9"
        ) {
            out.insert(0, '_');
        }
    }

    out
}

pub fn gen_logfile_name(
    ref_samp_filename: &PathBuf,
    station_code: String,
    is_main_log: bool,
) -> String {
    let date = Local::now().format("%d%m%Y").to_string();
    let ref_samp_filename_noext = ref_samp_filename
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let tail = if is_main_log { "log" } else { "intermediates" };
    let unsafe_name = format!(
        "{}_{}_{}_{}.csv",
        date, ref_samp_filename_noext, station_code, tail
    );
    let name = sanitize_filename(&unsafe_name);
    name
}
