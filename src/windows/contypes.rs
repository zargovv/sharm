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

#![allow(non_camel_case_types)]

use std::fmt::{self, Debug, Formatter};

/// the right alt key is pressed.
pub(crate) const RIGHT_ALT_PRESSED: u32 = 0x0001;
/// the left alt key is pressed.
pub(crate) const LEFT_ALT_PRESSED: u32 = 0x0002;
/// the right ctrl key is pressed.
pub(crate) const RIGHT_CTRL_PRESSED: u32 = 0x0004;
/// the left ctrl key is pressed.
pub(crate) const LEFT_CTRL_PRESSED: u32 = 0x0008;
/// the shift key is pressed.
pub(crate) const SHIFT_PRESSED: u32 = 0x0010;
/// the numlock light is on.
pub(crate) const NUMLOCK_ON: u32 = 0x0020;
/// the scrolllock light is on.
pub(crate) const SCROLLLOCK_ON: u32 = 0x0040;
/// the capslock light is on.
pub(crate) const CAPSLOCK_ON: u32 = 0x0080;
/// the key is enhanced.
pub(crate) const ENHANCED_KEY: u32 = 0x0100;
/// DBCS for JPN: SBCS/DBCS mode.
pub(crate) const NLS_DBCSCHAR: u32 = 0x0001_0000;
/// DBCS for JPN: Alphanumeric mode.
pub(crate) const NLS_ALPHANUMERIC: u32 = 0x0000_0000;
/// DBCS for JPN: Katakana mode.
pub(crate) const NLS_KATAKANA: u32 = 0x0002_0000;
/// DBCS for JPN: Hiragana mode.
pub(crate) const NLS_HIRAGANA: u32 = 0x0004_0000;
/// DBCS for JPN: Roman/Noroman mode.
pub(crate) const NLS_ROMAN: u32 = 0x0040_0000;
/// DBCS for JPN: IME conversion.
pub(crate) const NLS_IME_CONVERSION: u32 = 0x0080_0000;
/// `AltNumpad` OEM char (copied from ntuser\inc\kbd.h) ;`internal_NT`
pub(crate) const ALTNUMPAD_BIT: u32 = 0x0400_0000;
/// DBCS for JPN: IME enable/disable.
pub(crate) const NLS_IME_DISABLE: u32 = 0x2000_0000;

#[repr(C)]
pub(crate) union CHAR_INFO_0 {
  pub(crate) UnicodeChar: u16,
  pub(crate) AsciiChar: i8,
}

#[repr(C)]
pub(crate) struct CHAR_INFO {
  pub(crate) Char: CHAR_INFO_0,
  pub(crate) Attributes: u16,
}
pub(crate) type PCHAR_INFO = *mut CHAR_INFO;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct SMALL_RECT {
  pub(crate) Left: i16,
  pub(crate) Top: i16,
  pub(crate) Right: i16,
  pub(crate) Bottom: i16,
}
pub(crate) type PSMALL_RECT = *mut SMALL_RECT;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct COORD {
  pub(crate) X: i16,
  pub(crate) Y: i16,
}
type PCOORD = *mut COORD;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union KEY_EVENT_RECORD_0 {
  pub(crate) UnicodeChar: u16,
  pub(crate) AsciiChar: i8,
}

impl Debug for KEY_EVENT_RECORD_0 {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    f.write_fmt(format_args!("0x{:0>2}", unsafe { self.UnicodeChar }))
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct KEY_EVENT_RECORD {
  pub(crate) bKeyDown: i32,
  pub(crate) wRepeatCount: u16,
  pub(crate) wVirtualKeyCode: u16,
  pub(crate) wVirtualScanCode: u16,
  pub(crate) uChar: KEY_EVENT_RECORD_0,
  pub(crate) dwControlKeyState: u32,
}
pub(crate) type PKEY_EVENT_RECORD = *mut KEY_EVENT_RECORD;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct MOUSE_EVENT_RECORD {
  pub(crate) dwMousePosition: COORD,
  pub(crate) dwButtonState: u32,
  pub(crate) dwControlKeyState: u32,
  pub(crate) dwEventFlags: u32,
}
pub(crate) type PMOUSE_EVENT_RECORD = *mut MOUSE_EVENT_RECORD;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WINDOW_BUFFER_SIZE_RECORD {
  pub(crate) dwSize: COORD,
}
pub(crate) type PWINDOW_BUFFER_SIZE_RECORD = *mut WINDOW_BUFFER_SIZE_RECORD;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct MENU_EVENT_RECORD {
  dwCommandId: u32,
}
pub(crate) type PMENU_EVENT_RECORD = *mut MENU_EVENT_RECORD;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct FOCUS_EVENT_RECORD {
  bSetFocus: i32,
}
pub(crate) type PFOCUS_EVENT_RECORD = *mut FOCUS_EVENT_RECORD;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union INPUT_RECORD_0 {
  pub(crate) KeyEvent: KEY_EVENT_RECORD,
  pub(crate) MouseEvent: MOUSE_EVENT_RECORD,
  pub(crate) WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
  pub(crate) MenuEvent: MENU_EVENT_RECORD,
  pub(crate) FocusEvent: FOCUS_EVENT_RECORD,
}

/// Event contains key event record.
pub(crate) const KEY_EVENT: u16 = 0x0001;
/// Event contains mouse event record.
pub(crate) const MOUSE_EVENT: u16 = 0x0002;
/// Event contains window change event record.
pub(crate) const WINDOW_BUFFER_SIZE_EVENT: u16 = 0x0004;
/// Event contains menu event record.
pub(crate) const MENU_EVENT: u16 = 0x0008;
/// event contains focus change.
pub(crate) const FOCUS_EVENT: u16 = 0x0010;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct INPUT_RECORD {
  pub(crate) EventType: u16,
  pub(crate) Event: INPUT_RECORD_0,
}
pub(crate) type PINPUT_RECORD = *mut INPUT_RECORD;

impl Debug for INPUT_RECORD {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self.EventType {
      KEY_EVENT => unsafe { self.Event.KeyEvent }.fmt(f),
      MOUSE_EVENT => unsafe { self.Event.MouseEvent }.fmt(f),
      WINDOW_BUFFER_SIZE_EVENT => unsafe { self.Event.WindowBufferSizeEvent }.fmt(f),
      MENU_EVENT => unsafe { self.Event.MenuEvent }.fmt(f),
      FOCUS_EVENT => unsafe { self.Event.FocusEvent }.fmt(f),
      kind => panic!("Unknown event {kind:0>4x}"),
    }
  }
}
