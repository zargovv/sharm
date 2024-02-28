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

macro_rules! bitflags {
  ($vis:vis $ident:ident: $ty:ty { $($key:ident = $flag:expr;)* }) => {
    #[derive(Clone, Copy)]
    $vis struct $ident($ty);

    impl $ident {
      $($vis const $key: Self = Self($flag);)*
    }

    impl $ident {
      $vis const fn empty() -> Self {
        Self(0)
      }

      $vis const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
      }
    }

    impl ::std::ops::Not for $ident {
      type Output = Self;

      fn not(self) -> Self::Output {
        Self(!self.0)
      }
    }

    impl ::std::ops::BitAndAssign for $ident {
      fn bitand_assign(&mut self, rhs: Self)  {
        self.0 &= rhs.0;
      }
    }

    impl ::std::ops::BitOrAssign for $ident {
      fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
      }
    }

    impl ::std::fmt::Debug for $ident {
      fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut x = false;
        $(
          #[allow(unused_assignments)]
          if self.contains(Self::$key) {
            if x {
              f.write_str(" | ")?;
            }
            x = true;
            f.write_str(stringify!($key))?;
          }
        )*
        Ok(())
      }
    }
  }
}
pub(crate) use bitflags;
