#![warn(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    nonstandard_style,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    trivial_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused
)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::items_after_statements,
    clippy::many_single_char_names,
    clippy::match_on_vec_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

use std::str::FromStr;
use std::{fmt, mem};

use derive_more::Display;
use eyre::{eyre, Result};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};

// These strings have a constant size in memory but are null terminated unless
// they take up the whole max size of the string.
#[derive(Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Display)]
#[display(fmt = "{}", "String::from(*self)")]
pub struct Str(pub Str16, pub Str16);

impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Str {
    #[must_use]
    pub const fn empty() -> Self {
        Self(Str16::empty(), Str16::empty())
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    #[must_use]
    pub const fn max_size() -> usize {
        Str16::max_size() * 2
    }

    #[must_use]
    pub const fn index(&self, i: usize) -> char {
        assert!(i < Self::max_size(), "access out of bounds");
        if i < Str16::max_size() {
            self.0.index(i)
        } else {
            self.1.index(i - Str16::max_size())
        }
    }

    #[must_use]
    pub const fn from_literal(s: &'static str) -> Str {
        const MAXSZ: usize = 16;
        assert!(s.len() <= MAXSZ * 2, "too many bytes to make str");
        let mut b0 = [0; MAXSZ];
        let mut b1 = [0; MAXSZ];
        let mut sz = if s.len() < MAXSZ * 2 { s.len() } else { MAXSZ * 2 };
        let s = s.as_bytes();
        while sz > MAXSZ {
            sz -= 1;
            b1[sz - MAXSZ] = s[sz];
        }
        while sz > 0 {
            sz -= 1;
            b0[sz] = s[sz];
        }
        Self(Str16::from_bytes(b0), Str16::from_bytes(b1))
    }

    #[must_use]
    pub const fn to_ascii_lowercase(self) -> Self {
        Self(self.0.to_ascii_lowercase(), self.1.to_ascii_lowercase())
    }

    #[must_use]
    pub const fn to_ascii_uppercase(self) -> Self {
        Self(self.0.to_ascii_uppercase(), self.1.to_ascii_uppercase())
    }

    #[must_use]
    pub const fn starts_with(&self, s: Str) -> bool {
        self.0.starts_with(s.0) && self.1.starts_with(s.1)
    }
}

impl From<Str> for String {
    fn from(v: Str) -> Self {
        String::from(v.0) + &String::from(v.1)
    }
}

impl FromStr for Str {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let s0 = if s.len() < 16 { Str16::from_str(s)? } else { Str16::from_str(&s[..16])? };
        let s1 = if s.len() < 16 { Str16::empty() } else { Str16::from_str(&s[16..])? };
        Ok(Self(s0, s1))
    }
}

impl<'a> Deserialize<'a> for Str {
    fn deserialize<D: serde::Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        struct StrVisitor;

        impl<'a> Visitor<'a> for StrVisitor {
            type Value = Str;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Str, E>
            where
                E: de::Error,
            {
                v.parse().map_err(E::custom)
            }
        }

        d.deserialize_string(StrVisitor)
    }
}

impl Serialize for Str {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&String::from(*self))
    }
}

#[macro_export]
macro_rules! s {
    ($s:literal) => {
        $crate::Str::from_literal($s)
    };
    ($s:expr) => {
        $crate::Str::from_literal($s)
    };
}

#[derive(Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Display)]
#[display(fmt = "{}", "String::from(*self)")]
pub struct Str16(pub u128);

impl fmt::Debug for Str16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Str16 {
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
        mem::size_of::<u128>()
    }

    #[must_use]
    pub const fn index(&self, i: usize) -> char {
        assert!(i < Self::max_size(), "access out of bounds");
        ((self.0 & (0xFF << (i * 8))) >> (i * 8)) as u8 as char
    }

    #[must_use]
    pub const fn from_bytes(s: [u8; mem::size_of::<u128>()]) -> Self {
        Self(u128::from_le_bytes(s))
    }

    #[must_use]
    pub const fn from_literal(s: &'static str) -> Self {
        const MAXSZ: usize = Str16::max_size();
        assert!(s.len() <= MAXSZ, "too many bytes to make sid");
        let mut bytes = [0; MAXSZ];
        let mut sz = if s.len() < MAXSZ { s.len() } else { MAXSZ };
        let s = s.as_bytes();
        while sz > 0 {
            sz -= 1;
            bytes[sz] = s[sz];
        }
        Self(u128::from_le_bytes(bytes))
    }

    #[must_use]
    pub const fn to_ascii_lowercase(self) -> Self {
        let v = self.0
            | (((self.0 + 0x3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f)
                & !(self.0 + 0x25252525252525252525252525252525)
                & 0x80808080808080808080808080808080)
                >> 2);
        Self(v)
    }

    #[must_use]
    pub const fn to_ascii_uppercase(self) -> Self {
        let v = self.0
            & !(((self.0 + 0x1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f)
                & !(self.0 + 0x05050505050505050505050505050505)
                & 0x80808080808080808080808080808080)
                >> 2);
        Self(v)
    }

    #[must_use]
    pub const fn starts_with(&self, s: Str16) -> bool {
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

impl From<Str16> for String {
    fn from(v: Str16) -> Self {
        let bytes = v.0.to_le_bytes();
        let sz = bytes.iter().position(|&v| v == 0).unwrap_or(mem::size_of::<u128>());
        String::from_utf8_lossy(&bytes[..sz]).into()
    }
}

impl FromStr for Str16 {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() > mem::size_of::<u128>() {
            return Err(eyre!("too many bytes to make sid: {}", s));
        }
        let mut bytes = [0; mem::size_of::<u128>()];
        bytes[..s.len()].copy_from_slice(s.as_bytes());
        Ok(Str16(u128::from_le_bytes(bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str() {
        assert_eq!("asdf", String::from(s!("asdf")));
        assert_eq!("asdfasdf", String::from(s!("asdfasdf")));
        assert_eq!("asdfasdfasdfasdf", String::from(s!("asdfasdfasdfasdf")));
        assert_eq!("asdfasdfasdfasdfa", String::from(s!("asdfasdfasdfasdfa")));
        assert_eq!("asdfasdfasdfasdfab", String::from(s!("asdfasdfasdfasdfab")));
        assert_eq!(
            "asdfasdfasdfasdfasdfasdfasdfasdf",
            String::from(s!("asdfasdfasdfasdfasdfasdfasdfasdf"))
        );
    }

    #[test]
    fn indexing() {
        let s0 = "abcdefghijklmnopqrstuvwxyz012345";
        let s1 = s!("abcdefghijklmnopqrstuvwxyz012345");
        for i in 0..32 {
            assert_eq!(s0.chars().nth(i).unwrap(), s1.index(i));
        }
    }

    #[test]
    fn lowercase() {
        assert_eq!("asdf", String::from(s!("ASDF").to_ascii_lowercase()));
        assert_eq!("asdfasdf", String::from(s!("ASDFASDF").to_ascii_lowercase()));
        assert_eq!("asdfasdfasdfasdf", String::from(s!("ASDFASDFASDFASDF").to_ascii_lowercase()));
        assert_eq!("asdfasdfasdfasdfa", String::from(s!("ASDFASDFASDFASDFA").to_ascii_lowercase()));
        assert_eq!(
            "asdfasdfasdfasdfab",
            String::from(s!("ASDFASDFASDFASDFAB").to_ascii_lowercase())
        );
        assert_eq!(
            "asdfasdfasdfasdfasdfasdfasdfasdf",
            String::from(s!("ASDFASDFASDFASDFASDFASDFASDFASDF").to_ascii_lowercase())
        );
    }

    #[test]
    fn uppercase() {
        assert_eq!("ASDF", String::from(s!("asdf").to_ascii_uppercase()));
        assert_eq!("ASDFASDF", String::from(s!("asdfasdf").to_ascii_uppercase()));
        assert_eq!("ASDFASDFASDFASDF", String::from(s!("asdfasdfasdfasdf").to_ascii_uppercase()));
        assert_eq!("ASDFASDFASDFASDFA", String::from(s!("asdfasdfasdfasdfa").to_ascii_uppercase()));
        assert_eq!(
            "ASDFASDFASDFASDFAB",
            String::from(s!("asdfasdfasdfasdfab").to_ascii_uppercase())
        );
        assert_eq!(
            "ASDFASDFASDFASDFASDFASDFASDFASDF",
            String::from(s!("asdfasdfasdfasdfasdfasdfasdfasdf").to_ascii_uppercase())
        );
    }
}
