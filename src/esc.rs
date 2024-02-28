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

#![allow(dead_code)]

pub(crate) const ESC: &str = "\x1b[";

pub(crate) const MOVE_HOME: char = 'H';
pub(crate) const MOVE_RIGHT: char = 'C';
pub(crate) const MOVE_LEFT: char = 'D';
pub(crate) const MOVE_COL: char = 'G';

pub(crate) const END: &str = "";
pub(crate) const BEGIN: char = '1';
pub(crate) const ENTIRE: char = '2';
pub(crate) const ERASE_SCREEN: char = 'J';
pub(crate) const ERASE_LINE: char = 'K';

pub(crate) const ALT_BUF: &str = "?1049";
pub(crate) const ENABLE: char = 'h';
pub(crate) const DISABLE: char = 'l';

pub(crate) const STYLE: char = 'm';

pub(crate) const RESET: char = '0';
pub(crate) const BOLD: char = '1';
pub(crate) const RESET_BOLD: &str = "22";
pub(crate) const DIM: char = '2';
pub(crate) const RESET_DIM: &str = "22";
pub(crate) const ITALIC: char = '3';
pub(crate) const RESET_ITALIC: &str = "23";
pub(crate) const UNDERLINE: char = '4';
pub(crate) const RESET_UNDERLINE: &str = "24";
pub(crate) const BLINK: char = '5';
pub(crate) const RESET_BLINK: &str = "25";
pub(crate) const REVERSE: char = '7';
pub(crate) const RESET_REVERSE: &str = "27";
pub(crate) const HIDDEN: char = '8';
pub(crate) const RESET_HIDDEN: &str = "28";
pub(crate) const STRIKETHROUGH: char = '9';
pub(crate) const RESET_STRIKETHROUGH: &str = "29";

pub(crate) const FG: char = '3';
pub(crate) const BG: char = '4';

pub(crate) const TRUE_COLOR_256: &str = "8;5;";

pub(crate) const BLACK: char = '0';
pub(crate) const RED: char = '1';
pub(crate) const GREEN: char = '2';
pub(crate) const YELLOW: char = '3';
pub(crate) const BLUE: char = '4';
pub(crate) const MAGENTA: char = '5';
pub(crate) const CYAN: char = '6';
pub(crate) const WHITE: char = '7';
pub(crate) const DEFAULT: char = '9';
