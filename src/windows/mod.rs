// Sharm - The appropriate shell for Microsoft Windows.
// Copyright (C) 2024  Max Zargov <zargovv@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(dead_code, non_snake_case, clippy::upper_case_acronyms)]

mod base;
mod console;
mod contypes;
mod error;
mod handle;
mod minbase;
mod sysinfo;
mod user;

pub(crate) use base::*;
pub(crate) use console::*;
pub(crate) use contypes::*;
pub(crate) use error::*;
pub(crate) use handle::*;
pub(crate) use minbase::*;
pub(crate) use sysinfo::*;
pub(crate) use user::*;
