use std::fmt;
use std::str::FromStr;

use derive_more::Display;
use eyre::Result;
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};

use crate::str16::Str16;

// These strings have a constant size in memory but are null terminated unless
// they take up the whole max size of the string.
#[derive(Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Display)]
#[display(fmt = "{}", "String::from(*self)")]
pub struct Str32(pub Str16, pub Str16);

impl fmt::Debug for Str32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Str32 {
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
    pub const fn from_literal(s: &'static str) -> Str32 {
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
    pub const fn starts_with(&self, s: Str32) -> bool {
        self.0.starts_with(s.0) && self.1.starts_with(s.1)
    }
}

impl From<Str32> for String {
    fn from(v: Str32) -> Self {
        String::from(v.0) + &String::from(v.1)
    }
}

impl FromStr for Str32 {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let s0 = if s.len() < 16 { Str16::from_str(s)? } else { Str16::from_str(&s[..16])? };
        let s1 = if s.len() < 16 { Str16::empty() } else { Str16::from_str(&s[16..])? };
        Ok(Self(s0, s1))
    }
}

impl<'a> Deserialize<'a> for Str32 {
    fn deserialize<D: serde::Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        struct StrVisitor;

        impl<'a> Visitor<'a> for StrVisitor {
            type Value = Str32;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Str32, E>
            where
                E: de::Error,
            {
                v.parse().map_err(E::custom)
            }
        }

        d.deserialize_string(StrVisitor)
    }
}

impl Serialize for Str32 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&String::from(*self))
    }
}

#[macro_export]
macro_rules! s32 {
    ($s:literal) => {
        $crate::Str32::from_literal($s)
    };
    ($s:expr) => {
        $crate::Str32::from_literal($s)
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn str() {
        assert_eq!("asdf", String::from(s32!("asdf")));
        assert_eq!("asdfasdf", String::from(s32!("asdfasdf")));
        assert_eq!("asdfasdfasdfasdf", String::from(s32!("asdfasdfasdfasdf")));
        assert_eq!("asdfasdfasdfasdfa", String::from(s32!("asdfasdfasdfasdfa")));
        assert_eq!("asdfasdfasdfasdfab", String::from(s32!("asdfasdfasdfasdfab")));
        assert_eq!(
            "asdfasdfasdfasdfasdfasdfasdfasdf",
            String::from(s32!("asdfasdfasdfasdfasdfasdfasdfasdf"))
        );
    }

    #[test]
    fn indexing() {
        let s0 = "abcdefghijklmnopqrstuvwxyz012345";
        let s1 = s32!("abcdefghijklmnopqrstuvwxyz012345");
        for i in 0..32 {
            assert_eq!(s0.chars().nth(i).unwrap(), s1.index(i));
        }
    }

    #[test]
    fn lowercase() {
        assert_eq!("asdf", String::from(s32!("ASDF").to_ascii_lowercase()));
        assert_eq!("asdfasdf", String::from(s32!("ASDFASDF").to_ascii_lowercase()));
        assert_eq!("asdfasdfasdfasdf", String::from(s32!("ASDFASDFASDFASDF").to_ascii_lowercase()));
        assert_eq!(
            "asdfasdfasdfasdfa",
            String::from(s32!("ASDFASDFASDFASDFA").to_ascii_lowercase())
        );
        assert_eq!(
            "asdfasdfasdfasdfab",
            String::from(s32!("ASDFASDFASDFASDFAB").to_ascii_lowercase())
        );
        assert_eq!(
            "asdfasdfasdfasdfasdfasdfasdfasdf",
            String::from(s32!("ASDFASDFASDFASDFASDFASDFASDFASDF").to_ascii_lowercase())
        );
    }

    #[test]
    fn uppercase() {
        assert_eq!("ASDF", String::from(s32!("asdf").to_ascii_uppercase()));
        assert_eq!("ASDFASDF", String::from(s32!("asdfasdf").to_ascii_uppercase()));
        assert_eq!("ASDFASDFASDFASDF", String::from(s32!("asdfasdfasdfasdf").to_ascii_uppercase()));
        assert_eq!(
            "ASDFASDFASDFASDFA",
            String::from(s32!("asdfasdfasdfasdfa").to_ascii_uppercase())
        );
        assert_eq!(
            "ASDFASDFASDFASDFAB",
            String::from(s32!("asdfasdfasdfasdfab").to_ascii_uppercase())
        );
        assert_eq!(
            "ASDFASDFASDFASDFASDFASDFASDFASDF",
            String::from(s32!("asdfasdfasdfasdfasdfasdfasdfasdf").to_ascii_uppercase())
        );
    }
}
