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
pub(crate) mod csv;

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

#[cfg(feature="logged")]
use flexi_logger::{FileSpec, Logger, WriteMode};
#[cfg(feature="logged")]
use std::path::PathBuf;
#[cfg(feature="logged")]
use dirs::document_dir;
#[cfg(feature="logged")]
use log::Record;

use raylib::misc::AsF32;
use raylib::math::Rectangle;

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

#[cfg(feature="logged")]
pub(crate) fn prep_logger() -> Result<(),String> {
    let log_file_path;
    if let Some(documents_dir) = document_dir() {
        log_file_path = documents_dir.join("esox").join("log.txt");
    } else {
        log_file_path = PathBuf::from("./esox/log.txt");
    }

    if let Ok(logger_filespec) = FileSpec::try_from(log_file_path) {
        if let Ok(logger) = Logger::try_with_str("info, core=trace") {
            if let Err(e) = logger
            .log_to_file(logger_filespec)
            .write_mode(WriteMode::BufferAndFlush)
            .format(|_writer, _now, record: &Record| {
                writeln!(_writer, "{}", record.args())
            })
            .start() {
                eprintln!("Failed starting logger.");
                eprintln!("Error was: {e}");
                return Err(format!("Failed starting logger: {e}"));
            }
        } else {
            return Err("Failed loading logger from str LogSpecification".to_string());
        }
    } else {
        return Err("Failed loading logger filespec".to_string());
    }
    Ok(())
}
