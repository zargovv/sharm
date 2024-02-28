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

/// Left mouse button
pub(crate) const VK_LBUTTON: u16 = 0x01;
/// Right mouse button
pub(crate) const VK_RBUTTON: u16 = 0x02;
/// Control-break processing
pub(crate) const VK_CANCEL: u16 = 0x03;
/// Middle mouse button
pub(crate) const VK_MBUTTON: u16 = 0x04;
/// X1 mouse button
pub(crate) const VK_XBUTTON1: u16 = 0x05;
/// X2 mouse button
pub(crate) const VK_XBUTTON2: u16 = 0x06;
// pub(crate) const -: u16 = 0x07	Reserved
/// BACKSPACE key
pub(crate) const VK_BACK: u16 = 0x08;
/// TAB key
pub(crate) const VK_TAB: u16 = 0x09;
// pub(crate) const -: u16 = 0x0A-0B	Reserved
/// CLEAR key
pub(crate) const VK_CLEAR: u16 = 0x0C;
/// ENTER key
pub(crate) const VK_RETURN: u16 = 0x0D;
// pub(crate) const -: u16 = 0x0E-0F	Unassigned
/// SHIFT key
pub(crate) const VK_SHIFT: u16 = 0x10;
/// CTRL key
pub(crate) const VK_CONTROL: u16 = 0x11;
/// ALT key
pub(crate) const VK_MENU: u16 = 0x12;
/// PAUSE key
pub(crate) const VK_PAUSE: u16 = 0x13;
/// CAPS LOCK key
pub(crate) const VK_CAPITAL: u16 = 0x14;
/// IME Kana mode
pub(crate) const VK_KANA: u16 = 0x15;
/// IME Hangul mode
pub(crate) const VK_HANGUL: u16 = 0x15;
/// IME On
pub(crate) const VK_IME_ON: u16 = 0x16;
/// IME Junja mode
pub(crate) const VK_JUNJA: u16 = 0x17;
/// IME final mode
pub(crate) const VK_FINAL: u16 = 0x18;
/// IME Hanja mode
pub(crate) const VK_HANJA: u16 = 0x19;
/// IME Kanji mode
pub(crate) const VK_KANJI: u16 = 0x19;
/// IME Off
pub(crate) const VK_IME_OFF: u16 = 0x1A;
/// ESC key
pub(crate) const VK_ESCAPE: u16 = 0x1B;
/// IME convert
pub(crate) const VK_CONVERT: u16 = 0x1C;
/// IME nonconvert
pub(crate) const VK_NONCONVERT: u16 = 0x1D;
/// IME accept
pub(crate) const VK_ACCEPT: u16 = 0x1E;
/// IME mode change request
pub(crate) const VK_MODECHANGE: u16 = 0x1F;
/// SPACEBAR
pub(crate) const VK_SPACE: u16 = 0x20;
/// PAGE UP key
pub(crate) const VK_PRIOR: u16 = 0x21;
/// PAGE DOWN key
pub(crate) const VK_NEXT: u16 = 0x22;
/// END key
pub(crate) const VK_END: u16 = 0x23;
/// HOME key
pub(crate) const VK_HOME: u16 = 0x24;
/// LEFT ARROW key
pub(crate) const VK_LEFT: u16 = 0x25;
/// UP ARROW key
pub(crate) const VK_UP: u16 = 0x26;
/// RIGHT ARROW key
pub(crate) const VK_RIGHT: u16 = 0x27;
/// DOWN ARROW key
pub(crate) const VK_DOWN: u16 = 0x28;
/// SELECT key
pub(crate) const VK_SELECT: u16 = 0x29;
/// PRINT key
pub(crate) const VK_PRINT: u16 = 0x2A;
/// EXECUTE key
pub(crate) const VK_EXECUTE: u16 = 0x2B;
/// PRINT SCREEN key
pub(crate) const VK_SNAPSHOT: u16 = 0x2C;
/// INS key
pub(crate) const VK_INSERT: u16 = 0x2D;
/// DEL key
pub(crate) const VK_DELETE: u16 = 0x2E;
/// HELP key
pub(crate) const VK_HELP: u16 = 0x2F;
pub(crate) const VK_0: u16 = 0x30;
pub(crate) const VK_1: u16 = 0x31;
pub(crate) const VK_2: u16 = 0x32;
pub(crate) const VK_3: u16 = 0x33;
pub(crate) const VK_4: u16 = 0x34;
pub(crate) const VK_5: u16 = 0x35;
pub(crate) const VK_6: u16 = 0x36;
pub(crate) const VK_7: u16 = 0x37;
pub(crate) const VK_8: u16 = 0x38;
pub(crate) const VK_9: u16 = 0x39;
// pub(crate) const -: u16 = 0x3A-40	Undefined
pub(crate) const VK_A: u16 = 0x41;
pub(crate) const VK_B: u16 = 0x42;
pub(crate) const VK_C: u16 = 0x43;
pub(crate) const VK_D: u16 = 0x44;
pub(crate) const VK_E: u16 = 0x45;
pub(crate) const VK_F: u16 = 0x46;
pub(crate) const VK_G: u16 = 0x47;
pub(crate) const VK_H: u16 = 0x48;
pub(crate) const VK_I: u16 = 0x49;
pub(crate) const VK_J: u16 = 0x4A;
pub(crate) const VK_K: u16 = 0x4B;
pub(crate) const VK_L: u16 = 0x4C;
pub(crate) const VK_M: u16 = 0x4D;
pub(crate) const VK_N: u16 = 0x4E;
pub(crate) const VK_O: u16 = 0x4F;
pub(crate) const VK_P: u16 = 0x50;
pub(crate) const VK_Q: u16 = 0x51;
pub(crate) const VK_R: u16 = 0x52;
pub(crate) const VK_S: u16 = 0x53;
pub(crate) const VK_T: u16 = 0x54;
pub(crate) const VK_U: u16 = 0x55;
pub(crate) const VK_V: u16 = 0x56;
pub(crate) const VK_W: u16 = 0x57;
pub(crate) const VK_X: u16 = 0x58;
pub(crate) const VK_Y: u16 = 0x59;
pub(crate) const VK_Z: u16 = 0x5A;
/// Left Windows key
pub(crate) const VK_LWIN: u16 = 0x5B;
/// Right Windows key
pub(crate) const VK_RWIN: u16 = 0x5C;
/// Applications key
pub(crate) const VK_APPS: u16 = 0x5D;
// pub(crate) const -: u16 = 0x5E	Reserved
/// Computer Sleep key
pub(crate) const VK_SLEEP: u16 = 0x5F;
/// Numeric keypad 0 key
pub(crate) const VK_NUMPAD0: u16 = 0x60;
/// Numeric keypad 1 key
pub(crate) const VK_NUMPAD1: u16 = 0x61;
/// Numeric keypad 2 key
pub(crate) const VK_NUMPAD2: u16 = 0x62;
/// Numeric keypad 3 key
pub(crate) const VK_NUMPAD3: u16 = 0x63;
/// Numeric keypad 4 key
pub(crate) const VK_NUMPAD4: u16 = 0x64;
/// Numeric keypad 5 key
pub(crate) const VK_NUMPAD5: u16 = 0x65;
/// Numeric keypad 6 key
pub(crate) const VK_NUMPAD6: u16 = 0x66;
/// Numeric keypad 7 key
pub(crate) const VK_NUMPAD7: u16 = 0x67;
/// Numeric keypad 8 key
pub(crate) const VK_NUMPAD8: u16 = 0x68;
/// Numeric keypad 9 key
pub(crate) const VK_NUMPAD9: u16 = 0x69;
/// Multiply key
pub(crate) const VK_MULTIPLY: u16 = 0x6A;
/// Add key
pub(crate) const VK_ADD: u16 = 0x6B;
/// Separator key
pub(crate) const VK_SEPARATOR: u16 = 0x6C;
/// Subtract key
pub(crate) const VK_SUBTRACT: u16 = 0x6D;
/// Decimal key
pub(crate) const VK_DECIMAL: u16 = 0x6E;
/// Divide key
pub(crate) const VK_DIVIDE: u16 = 0x6F;
/// F1 key
pub(crate) const VK_F1: u16 = 0x70;
/// F2 key
pub(crate) const VK_F2: u16 = 0x71;
/// F3 key
pub(crate) const VK_F3: u16 = 0x72;
/// F4 key
pub(crate) const VK_F4: u16 = 0x73;
/// F5 key
pub(crate) const VK_F5: u16 = 0x74;
/// F6 key
pub(crate) const VK_F6: u16 = 0x75;
/// F7 key
pub(crate) const VK_F7: u16 = 0x76;
/// F8 key
pub(crate) const VK_F8: u16 = 0x77;
/// F9 key
pub(crate) const VK_F9: u16 = 0x78;
/// F10 key
pub(crate) const VK_F10: u16 = 0x79;
/// F11 key
pub(crate) const VK_F11: u16 = 0x7A;
/// F12 key
pub(crate) const VK_F12: u16 = 0x7B;
/// F13 key
pub(crate) const VK_F13: u16 = 0x7C;
/// F14 key
pub(crate) const VK_F14: u16 = 0x7D;
/// F15 key
pub(crate) const VK_F15: u16 = 0x7E;
/// F16 key
pub(crate) const VK_F16: u16 = 0x7F;
/// F17 key
pub(crate) const VK_F17: u16 = 0x80;
/// F18 key
pub(crate) const VK_F18: u16 = 0x81;
/// F19 key
pub(crate) const VK_F19: u16 = 0x82;
/// F20 key
pub(crate) const VK_F20: u16 = 0x83;
/// F21 key
pub(crate) const VK_F21: u16 = 0x84;
/// F22 key
pub(crate) const VK_F22: u16 = 0x85;
/// F23 key
pub(crate) const VK_F23: u16 = 0x86;
/// F24 key
pub(crate) const VK_F24: u16 = 0x87;
// pub(crate) const -: u16 = 0x88-8F	Reserved
/// NUM LOCK key
pub(crate) const VK_NUMLOCK: u16 = 0x90;
/// SCROLL LOCK key
pub(crate) const VK_SCROLL: u16 = 0x91;
// pub(crate) const -: u16 = 0x92-96	OEM specific
// pub(crate) const -: u16 = 0x97-9F	Unassigned
/// Left SHIFT key
pub(crate) const VK_LSHIFT: u16 = 0xA0;
/// Right SHIFT key
pub(crate) const VK_RSHIFT: u16 = 0xA1;
/// Left CONTROL key
pub(crate) const VK_LCONTROL: u16 = 0xA2;
/// Right CONTROL key
pub(crate) const VK_RCONTROL: u16 = 0xA3;
/// Left ALT key
pub(crate) const VK_LMENU: u16 = 0xA4;
/// Right ALT key
pub(crate) const VK_RMENU: u16 = 0xA5;
/// Browser Back key
pub(crate) const VK_BROWSER_BACK: u16 = 0xA6;
/// Browser Forward key
pub(crate) const VK_BROWSER_FORWARD: u16 = 0xA7;
/// Browser Refresh key
pub(crate) const VK_BROWSER_REFRESH: u16 = 0xA8;
/// Browser Stop key
pub(crate) const VK_BROWSER_STOP: u16 = 0xA9;
/// Browser Search key
pub(crate) const VK_BROWSER_SEARCH: u16 = 0xAA;
/// Browser Favorites key
pub(crate) const VK_BROWSER_FAVORITES: u16 = 0xAB;
/// Browser Start and Home key
pub(crate) const VK_BROWSER_HOME: u16 = 0xAC;
/// Volume Mute key
pub(crate) const VK_VOLUME_MUTE: u16 = 0xAD;
/// Volume Down key
pub(crate) const VK_VOLUME_DOWN: u16 = 0xAE;
/// Volume Up key
pub(crate) const VK_VOLUME_UP: u16 = 0xAF;
/// Next Track key
pub(crate) const VK_MEDIA_NEXT_TRACK: u16 = 0xB0;
/// Previous Track key
pub(crate) const VK_MEDIA_PREV_TRACK: u16 = 0xB1;
/// Stop Media key
pub(crate) const VK_MEDIA_STOP: u16 = 0xB2;
/// Play/Pause Media key
pub(crate) const VK_MEDIA_PLAY_PAUSE: u16 = 0xB3;
/// Start Mail key
pub(crate) const VK_LAUNCH_MAIL: u16 = 0xB4;
/// Select Media key
pub(crate) const VK_LAUNCH_MEDIA_SELECT: u16 = 0xB5;
/// Start Application 1 key
pub(crate) const VK_LAUNCH_APP1: u16 = 0xB6;
/// Start Application 2 key
pub(crate) const VK_LAUNCH_APP2: u16 = 0xB7;
// pub(crate) const -: u16 = 0xB8-B9	Reserved
/// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the
/// ;: key
pub(crate) const VK_OEM_1: u16 = 0xBA;
/// For any country/region, the + key
pub(crate) const VK_OEM_PLUS: u16 = 0xBB;
/// For any country/region, the , key
pub(crate) const VK_OEM_COMMA: u16 = 0xBC;
/// For any country/region, the - key
pub(crate) const VK_OEM_MINUS: u16 = 0xBD;
/// For any country/region, the . key
pub(crate) const VK_OEM_PERIOD: u16 = 0xBE;
/// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the /?
/// key
pub(crate) const VK_OEM_2: u16 = 0xBF;
/// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the
/// `` `~`` key
pub(crate) const VK_OEM_3: u16 = 0xC0;
/// DA Reserved
// pub(crate) const -: u16 = 0xC1;
/// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the [{
/// key
pub(crate) const VK_OEM_4: u16 = 0xDB;
/// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the
/// \\| key
pub(crate) const VK_OEM_5: u16 = 0xDC;
/// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ]}
/// key
pub(crate) const VK_OEM_6: u16 = 0xDD;
/// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '"
/// key
pub(crate) const VK_OEM_7: u16 = 0xDE;
/// Used for miscellaneous characters; it can vary by keyboard.
pub(crate) const VK_OEM_8: u16 = 0xDF;
// pub(crate) const -: u16 = 0xE0	Reserved
// pub(crate) const -: u16 = 0xE1	OEM specific
/// The <> keys on the US standard keyboard, or the \\| key on the non-US 102-key keyboard
pub(crate) const VK_OEM_102: u16 = 0xE2;
// pub(crate) const -: u16 = 0xE3-E4	OEM specific
/// IME PROCESS key
pub(crate) const VK_PROCESSKEY: u16 = 0xE5;
// pub(crate) const -: u16 = 0xE6	OEM specific
/// Used to pass Unicode characters as if they were keystrokes. The `VK_PACKET` key is the low word
/// of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see
/// Remark in KEYBDINPUT, `SendInput`, `WM_KEYDOWN`, and `WM_KEYUP`
pub(crate) const VK_PACKET: u16 = 0xE7;
// pub(crate) const -: u16 = 0xE8	Unassigned
// pub(crate) const -: u16 = 0xE9-F5	OEM specific
/// Attn key
pub(crate) const VK_ATTN: u16 = 0xF6;
/// `CrSel` key
pub(crate) const VK_CRSEL: u16 = 0xF7;
/// `ExSel` key
pub(crate) const VK_EXSEL: u16 = 0xF8;
/// Erase EOF key
pub(crate) const VK_EREOF: u16 = 0xF9;
/// Play key
pub(crate) const VK_PLAY: u16 = 0xFA;
/// Zoom key
pub(crate) const VK_ZOOM: u16 = 0xFB;
/// Reserved
pub(crate) const VK_NONAME: u16 = 0xFC;
/// PA1 key
pub(crate) const VK_PA1: u16 = 0xFD;
/// Clear key
pub(crate) const VK_OEM_CLEAR: u16 = 0xFE;
