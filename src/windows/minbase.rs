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

#[repr(C)]
pub(crate) struct SYSTEMTIME {
  pub(crate) wYear: u16,
  pub(crate) wMonth: u16,
  pub(crate) wDayOfWeek: u16,
  pub(crate) wDay: u16,
  pub(crate) wHour: u16,
  pub(crate) wMinute: u16,
  pub(crate) wSecond: u16,
  pub(crate) wMilliseconds: u16,
}
pub(crate) type PSYSTEMTIME = *mut SYSTEMTIME;
pub(crate) type LPSYSTEMTIME = *mut SYSTEMTIME;
