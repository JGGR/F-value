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
