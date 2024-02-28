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

mod esc;
mod essential;
mod windows;

use std::{
  borrow::Cow,
  env::{self, current_dir},
  ffi::OsStr,
  fs::{metadata, File},
  io::Read,
  mem::{size_of, zeroed},
  os::windows::{ffi::OsStrExt, raw::HANDLE},
  ptr::null_mut,
};

use essential::bitflags;
use windows::{
  GetConsoleMode, GetConsoleScreenBufferInfo, GetStdHandle, GetSystemTime, SetConsoleTitleW,
  CONSOLE_SCREEN_BUFFER_INFO, COORD, SYSTEMTIME, VK_BACK, VK_C, VK_K, VK_L, VK_SPACE, VK_TAB, VK_W,
  WINDOW_BUFFER_SIZE_EVENT,
};

use crate::{
  esc::*,
  windows::{
    GetLastError, ReadConsoleInputW, SetConsoleMode, WriteConsoleW, ENABLE_ECHO_INPUT,
    ENABLE_INSERT_MODE, ENABLE_LINE_INPUT, ENABLE_MOUSE_INPUT, ENABLE_PROCESSED_INPUT,
    ENABLE_QUICK_EDIT_MODE, ENABLE_VIRTUAL_TERMINAL_INPUT, ENABLE_WINDOW_INPUT,
    ENABLE_WRAP_AT_EOL_OUTPUT, INPUT_RECORD, KEY_EVENT, LEFT_CTRL_PRESSED, RIGHT_CTRL_PRESSED,
    STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, VK_D, VK_RETURN,
  },
};

bitflags!(pub(crate) StateFlags: u8 {
  PROMPT = 1 << 0;
});

pub(crate) struct State {
  pub(crate) hstdout: HANDLE,
  pub(crate) bounds: COORD,
  pub(crate) path: Vec<String>,
  pub(crate) flags: StateFlags,
  pub(crate) prompt_len: i16,
  pub(crate) status: u16,
  pub(crate) cmd: String,
  pub(crate) args: Vec<String>,
}

impl State {
  pub(crate) fn new(hstdout: HANDLE) -> Self {
    let mut csbi = unsafe { zeroed::<CONSOLE_SCREEN_BUFFER_INFO>() };
    unsafe { GetConsoleScreenBufferInfo(hstdout, &mut csbi) };
    Self {
      hstdout,
      bounds: csbi.dwSize,
      path: env::var("PATH").map_or(Vec::new(), |path| {
        path
          .replace('\\', "/")
          .split(';')
          .filter_map(|path| {
            metadata(path)
              .ok()
              .and_then(|metadata| metadata.is_dir().then(|| path.to_owned()))
          })
          .collect::<Vec<_>>()
      }),
      flags: StateFlags::empty(),
      prompt_len: 0,
      status: 0,
      cmd: String::new(),
      args: Vec::new(),
    }
  }
}

impl State {
  pub(crate) fn resolve_command(&self) -> Option<String> {
    let mut exec_path = None::<String>;
    for dirname in &self.path {
      let filepath = format!("{dirname}/{}.exe", self.cmd);
      if metadata(&filepath).is_ok_and(|metadata| metadata.is_file()) {
        exec_path = Some(filepath);
      }
    }
    exec_path
  }

  pub(crate) fn validate_command(&self) {
    if !self.cmd.is_empty() {
      let exec_path = self.resolve_command();
      write_console(
        self.hstdout,
        format!(
          "{ESC}{}{MOVE_LEFT}{ESC}{FG}{TRUE_COLOR_256}{}{STYLE}{}{ESC}{RESET}{STYLE}",
          self.cmd.len(),
          if exec_path.is_some() { 143_u8 } else { 131_u8 },
          self.cmd,
          ESC = ESC,
          MOVE_LEFT = MOVE_LEFT,
          FG = FG,
          STYLE = STYLE,
          RESET = RESET,
        ),
      );
    }
  }

  pub(crate) fn clear_cmd(&mut self) {
    self.cmd.clear();
    self.args.clear();
  }

  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  pub(crate) fn print_prompt(&mut self) {
    let cwpath = current_dir().unwrap();
    let cwdir = cwpath
      .file_name()
      .map_or(Cow::Borrowed("/"), |filename| filename.to_string_lossy());
    let mut component_iter = cwpath.components();
    let drive = component_iter
      .next()
      .map_or(Cow::Borrowed(":"), |component| {
        component.as_os_str().to_string_lossy()
      });
    let mut components = component_iter
      .skip(1)
      .filter_map(|component| component.as_os_str().to_string_lossy().chars().next())
      .fold(String::new(), |mut acc, c| {
        acc.push(c);
        acc += "/";
        acc
      });
    components.truncate(components.len() - 2);
    unsafe {
      SetConsoleTitleW(
        format!("✧ {drive}/{components}{cwdir}\0")
          .to_wide()
          .as_ptr(),
      )
    };

    let mut csbi = unsafe { zeroed::<CONSOLE_SCREEN_BUFFER_INFO>() };
    unsafe { GetConsoleScreenBufferInfo(self.hstdout, &mut csbi) };
    if csbi.dwCursorPosition.X != 0 {
      write_console(
        self.hstdout,
        format!("{ESC}{BOLD};{FG}{BLACK}{STYLE}⏎{ESC}{RESET}{STYLE}\n"),
      );
    }

    let mut system_time = unsafe { zeroed::<SYSTEMTIME>() };
    unsafe { GetSystemTime(&mut system_time) };
    let time = format!(
      "{}:{:0>2}:{:0>2} {}",
      if system_time.wHour == 12 {
        system_time.wHour
      } else {
        system_time.wHour % 12
      },
      system_time.wMinute,
      system_time.wSecond,
      if system_time.wHour < 12 { "AM" } else { "PM" },
    );
    write_console(
      self.hstdout,
      format!(
        "{ESC}{}{MOVE_COL}{ESC}{DIM};{FG}{WHITE}{STYLE}{time}{ESC}{RESET}{STYLE}{ESC}{MOVE_COL}",
        self.bounds.X - time.len() as i16,
      ),
    );

    let mut gitpath = cwpath.clone();
    let branch = loop {
      if let Ok(mut file) = File::open(format!("{}/.git/HEAD", gitpath.to_string_lossy())) {
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_ok() {
          break buf.rsplit('/').next().map(|ref_| ref_.trim().to_owned());
        }
        break None;
      }
      if !gitpath.pop() {
        break None;
      }
    };

    let prompt = format!(
      "{ESC}{BOLD};{FG}{}{STYLE}➜  {ESC}{BOLD};{FG}{CYAN}{STYLE}{cwdir}{}{ESC}{RESET}{STYLE} ",
      if self.status == 0 { GREEN } else { RED },
      branch.map_or_else(
        || Cow::Borrowed(""),
        |branch| Cow::Owned(format!(
          " {ESC}{FG}{BLUE}{STYLE}git:({ESC}{FG}{RED}{STYLE}{branch}{ESC}{FG}{BLUE}{STYLE})"
        )),
      )
    );

    self.flags |= StateFlags::PROMPT;
    self.prompt_len = prompt.len() as i16;
    write_console(self.hstdout, prompt);

    write_console(self.hstdout, self.cmd.as_str());
    self.validate_command();
    for arg in &self.args {
      write_console(self.hstdout, format!(" {arg}"));
    }
  }
}

trait ToWide {
  fn to_wide(self) -> Vec<u16>;
}

impl ToWide for u8 {
  fn to_wide(self) -> Vec<u16> {
    Vec::from([u16::from(self)])
  }
}

impl ToWide for u16 {
  fn to_wide(self) -> Vec<u16> {
    Vec::from([self])
  }
}

impl ToWide for String {
  fn to_wide(self) -> Vec<u16> {
    self.as_str().to_wide()
  }
}

impl ToWide for &str {
  fn to_wide(self) -> Vec<u16> {
    AsRef::<OsStr>::as_ref(self)
      .encode_wide()
      .collect::<Vec<_>>()
  }
}

fn write_console(hstdout: HANDLE, text: impl ToWide) {
  let wide = text.to_wide();
  unsafe {
    #[allow(clippy::cast_possible_truncation)]
    WriteConsoleW(
      hstdout,
      wide.as_ptr().cast(),
      wide.len() as u32,
      &mut 0,
      null_mut(),
    );
  }
}

#[allow(clippy::too_many_lines)] // TODO: Remove at some point.
fn event_loop(state: &mut State, event: &INPUT_RECORD) -> u8 {
  if !state.flags.contains(StateFlags::PROMPT) {
    state.print_prompt();
  }

  match event.EventType {
    WINDOW_BUFFER_SIZE_EVENT => {
      let wbs_event = unsafe { event.Event.WindowBufferSizeEvent };
      state.bounds = wbs_event.dwSize;
    }
    KEY_EVENT => {
      let key_event = unsafe { event.Event.KeyEvent };
      if key_event.bKeyDown == 1 {
        if (key_event.dwControlKeyState & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED)) != 0 {
          // Visuals.
          match key_event.wVirtualKeyCode {
            VK_K => write_console(state.hstdout, format!("{ESC}{ENTIRE}{ERASE_SCREEN}")),
            VK_L => {
              write_console(
                state.hstdout,
                format!("{ESC}{MOVE_HOME}{ESC}{ERASE_SCREEN}"),
              );
              state.print_prompt();
            }
            vk @ VK_C if !state.cmd.is_empty() => write_console(
              state.hstdout,
              format!(
                "{ESC}{BG}{WHITE};{FG}{BLACK}{STYLE}^{}{ESC}{RESET}{STYLE}",
                unsafe { char::from_u32_unchecked(u32::from(vk)) },
              ),
            ),
            vk @ VK_D => write_console(
              state.hstdout,
              format!(
                "{ESC}{BG}{WHITE};{FG}{BLACK}{STYLE}^{}{ESC}{RESET}{STYLE}",
                unsafe { char::from_u32_unchecked(u32::from(vk)) },
              ),
            ),
            _ => {}
          }

          // Action.
          match key_event.wVirtualKeyCode {
            VK_W if !state.cmd.is_empty() => {
              let mut inc = 0_usize;
              while state.args.last().map_or(false, String::is_empty) {
                inc += 1;
                state.args.pop();
              }
              let arg = state.args.last_mut().unwrap_or(&mut state.cmd);
              let len = arg.len();
              arg.clear();
              let mv = len + inc;
              write_console(
                state.hstdout,
                format!("{ESC}{mv}{MOVE_LEFT}{}{ESC}{mv}{MOVE_LEFT}", " ".repeat(mv)),
              );
            }
            VK_C if !state.cmd.is_empty() => {
              write_console(state.hstdout, b'\n');
              state.clear_cmd();
              state.print_prompt();
            }
            VK_D => return 0,
            _ => {}
          }
        } else if unsafe { key_event.uChar.UnicodeChar } != 0 {
          match key_event.wVirtualKeyCode {
            VK_BACK => {
              if !state.cmd.is_empty() {
                if state.args.last().map_or(false, String::is_empty) {
                  state.args.pop();
                } else {
                  let arg = state.args.last_mut().unwrap_or(&mut state.cmd);
                  arg.pop();
                }
                write_console(state.hstdout, format!("{ESC}{MOVE_LEFT} {ESC}{MOVE_LEFT}"));
                if state.args.is_empty() {
                  state.validate_command();
                }
              }
            }
            VK_TAB => {}
            _ => {
              let unicode_char = unsafe { key_event.uChar.UnicodeChar };
              write_console(
                state.hstdout,
                if key_event.wVirtualKeyCode == VK_RETURN {
                  u16::from(b'\n')
                } else {
                  unicode_char
                },
              );

              match key_event.wVirtualKeyCode {
                VK_SPACE => state.args.push(String::new()),
                VK_RETURN => {
                  if state.args.last().map_or(false, String::is_empty) {
                    state.args.pop();
                  }
                  if !state.cmd.is_empty() {
                    if let Some(cmd) = state.resolve_command() {
                      // TODO: Execute
                      state.status = 0;
                    } else {
                      write_console(state.hstdout, format!("{}: command not found\n", state.cmd));
                      state.status = 1;
                    }
                  }
                  state.clear_cmd();
                  state.print_prompt();
                }
                _ => {
                  let arg = state.args.last_mut().unwrap_or(&mut state.cmd);
                  arg.push(unsafe { char::from_u32_unchecked(u32::from(unicode_char)) });

                  if state.args.is_empty() {
                    state.validate_command();
                  }
                }
              }
            }
          }
        }
      }
    }
    _ => {}
  }

  // println!("{event:#?}");
  1
}

fn main() {
  let hstdin = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
  let hstdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

  if hstdin.is_null() {
    eprintln!("Failed to get std input handle: {}", unsafe {
      GetLastError()
    });
    return;
  }

  if hstdout.is_null() {
    eprintln!("Failed to get std output handle: {}", unsafe {
      GetLastError()
    });
    return;
  }

  let mut old_inmode = 0_u32;
  if unsafe { GetConsoleMode(hstdin, &mut old_inmode) } == 0 {
    eprintln!("Failed to get console mode: {}", unsafe { GetLastError() });
    return;
  }

  // println!("inmode: {old_inmode:0>4x}");

  let inmode = old_inmode
    & !(ENABLE_ECHO_INPUT
      | ENABLE_INSERT_MODE
      | ENABLE_LINE_INPUT
      | ENABLE_PROCESSED_INPUT
      | ENABLE_WINDOW_INPUT
      | ENABLE_VIRTUAL_TERMINAL_INPUT
      | ENABLE_WRAP_AT_EOL_OUTPUT)
    | ENABLE_MOUSE_INPUT
    | ENABLE_QUICK_EDIT_MODE;

  if unsafe { SetConsoleMode(hstdin, inmode) } == 0 {
    eprintln!("Failed to set input mode: {}", unsafe { GetLastError() });
    return;
  }

  let mut state = State::new(hstdout);

  let mut event_buf = [unsafe { zeroed::<INPUT_RECORD>() }; if size_of::<INPUT_RECORD>() > 4096 {
    1
  } else {
    4096 / size_of::<INPUT_RECORD>()
  }];
  let mut nread = 0_u32;
  #[allow(clippy::cast_possible_truncation)]
  'outer: while unsafe {
    ReadConsoleInputW(
      hstdin,
      event_buf.as_mut_ptr(),
      event_buf.len() as u32,
      &mut nread,
    )
  } != 0
  {
    // println!("Recv: [{nread}]");
    for event in &event_buf[..nread as usize] {
      if event_loop(&mut state, event) == 0 {
        break 'outer;
      }
    }
  }

  if unsafe { SetConsoleMode(hstdin, old_inmode) } == 0 {
    eprintln!("Failed to return console mode: {}", unsafe {
      GetLastError()
    });
  }
}
