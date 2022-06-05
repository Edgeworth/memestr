use std::str::FromStr;
use std::{fmt, mem};

use derive_more::Display;
use eyre::{eyre, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};

#[derive(Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Display)]
#[display(fmt = "{}", "String::from(*self)")]
pub struct Str8(pub u64);

impl fmt::Debug for Str8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Str8 {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn max_size() -> usize {
        mem::size_of::<u64>()
    }

    #[must_use]
    pub const fn index(&self, i: usize) -> char {
        assert!(i < Self::max_size(), "access out of bounds");
        ((self.0 & (0xFF << (i * 8))) >> (i * 8)) as u8 as char
    }

    #[must_use]
    pub const fn from_bytes(s: [u8; mem::size_of::<u64>()]) -> Self {
        Self(u64::from_le_bytes(s))
    }

    #[must_use]
    pub const fn from_literal(s: &'static str) -> Self {
        const MAXSZ: usize = Str8::max_size();
        assert!(s.len() <= MAXSZ, "too many bytes to make sid");
        let mut bytes = [0; MAXSZ];
        let mut sz = if s.len() < MAXSZ { s.len() } else { MAXSZ };
        let s = s.as_bytes();
        while sz > 0 {
            sz -= 1;
            bytes[sz] = s[sz];
        }
        Self(u64::from_le_bytes(bytes))
    }

    #[must_use]
    pub const fn to_ascii_lowercase(self) -> Self {
        let v = self.0
            | (((self.0 + 0x3f3f3f3f3f3f3f3f)
                & !(self.0 + 0x2525252525252525)
                & 0x8080808080808080)
                >> 2);
        Self(v)
    }

    #[must_use]
    pub const fn to_ascii_uppercase(self) -> Self {
        let v = self.0
            & !(((self.0 + 0x1f1f1f1f1f1f1f1f)
                & !(self.0 + 0x0505050505050505)
                & 0x8080808080808080)
                >> 2);
        Self(v)
    }

    #[must_use]
    pub const fn starts_with(&self, s: Str8) -> bool {
        let a = self.0.to_le_bytes();
        let b = s.0.to_le_bytes();
        let mut i = 0;
        while i < a.len() {
            if b[i] == 0 {
                return true;
            }
            if a[i] != b[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl From<Str8> for String {
    fn from(v: Str8) -> Self {
        let bytes = v.0.to_le_bytes();
        let sz = bytes.iter().position(|&v| v == 0).unwrap_or(mem::size_of::<u64>());
        String::from_utf8_lossy(&bytes[..sz]).into()
    }
}

impl FromStr for Str8 {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() > mem::size_of::<u64>() {
            return Err(eyre!("too many bytes to make sid: {}", s));
        }
        let mut bytes = [0; mem::size_of::<u64>()];
        bytes[..s.len()].copy_from_slice(s.as_bytes());
        Ok(Str8(u64::from_le_bytes(bytes)))
    }
}

impl<'a> Deserialize<'a> for Str8 {
    fn deserialize<D: serde::Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        struct StrVisitor;

        impl<'a> Visitor<'a> for StrVisitor {
            type Value = Str8;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Str8, E>
            where
                E: de::Error,
            {
                v.parse().map_err(E::custom)
            }
        }

        d.deserialize_string(StrVisitor)
    }
}

impl Serialize for Str8 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&String::from(*self))
    }
}

#[macro_export]
macro_rules! s8 {
    ($s:literal) => {
        $crate::Str8::from_literal($s)
    };
    ($s:expr) => {
        $crate::Str8::from_literal($s)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn str() {
        assert_eq!("asdf", String::from(s8!("asdf")));
        assert_eq!("asdfa", String::from(s8!("asdfa")));
        assert_eq!("asdfasdf", String::from(s8!("asdfasdf")));
    }

    #[test]
    fn indexing() {
        let s0 = "abcdefgh";
        let s1 = s8!("abcdefgh");
        for i in 0..8 {
            assert_eq!(s0.chars().nth(i).unwrap(), s1.index(i));
        }
    }

    #[test]
    fn lowercase() {
        assert_eq!("asdf", String::from(s8!("ASDF").to_ascii_lowercase()));
        assert_eq!("asdfa", String::from(s8!("ASDFA").to_ascii_lowercase()));
        assert_eq!("asdfasdf", String::from(s8!("ASDFASDF").to_ascii_lowercase()));
    }

    #[test]
    fn uppercase() {
        assert_eq!("ASDF", String::from(s8!("asdf").to_ascii_uppercase()));
        assert_eq!("ASDFA", String::from(s8!("asdfa").to_ascii_uppercase()));
        assert_eq!("ASDFASDF", String::from(s8!("asdfasdf").to_ascii_uppercase()));
    }
}
