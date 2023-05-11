use std::fmt;
use std::str::FromStr;

use derive_more::Display;
use eyre::Result;
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};

use crate::str16::Str16;
use crate::str8::Str8;

// These strings have a constant size in memory but are null terminated unless
// they take up the whole max size of the string.
#[must_use]
#[derive(Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Display)]
#[display(fmt = "{}", "String::from(*self)")]
pub struct Str24(pub Str16, pub Str8);

impl fmt::Debug for Str24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl Str24 {
    pub const fn empty() -> Self {
        Self(Str16::empty(), Str8::empty())
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    #[must_use]
    pub const fn max_size() -> usize {
        Str16::max_size() + Str8::max_size()
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

    pub const fn from_literal(s: &'static str) -> Self {
        const MAXSZ0: usize = 16;
        const MAXSZ1: usize = 8;
        assert!(s.len() <= MAXSZ0 + MAXSZ1, "too many bytes to make str");
        let mut b0 = [0; MAXSZ0];
        let mut b1 = [0; MAXSZ1];
        let mut sz = if s.len() < MAXSZ0 + MAXSZ1 { s.len() } else { MAXSZ0 + MAXSZ1 };
        let s = s.as_bytes();
        while sz > MAXSZ0 {
            sz -= 1;
            b1[sz - MAXSZ0] = s[sz];
        }
        while sz > 0 {
            sz -= 1;
            b0[sz] = s[sz];
        }
        Self(Str16::from_bytes(b0), Str8::from_bytes(b1))
    }

    pub const fn to_ascii_lowercase(self) -> Self {
        Self(self.0.to_ascii_lowercase(), self.1.to_ascii_lowercase())
    }

    pub const fn to_ascii_uppercase(self) -> Self {
        Self(self.0.to_ascii_uppercase(), self.1.to_ascii_uppercase())
    }

    #[must_use]
    pub const fn starts_with(&self, s: Str24) -> bool {
        self.0.starts_with(s.0) && self.1.starts_with(s.1)
    }
}

impl From<Str24> for String {
    fn from(v: Str24) -> Self {
        String::from(v.0) + &String::from(v.1)
    }
}

impl FromStr for Str24 {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let s0 = if s.len() < 16 { Str16::from_str(s)? } else { Str16::from_str(&s[..16])? };
        let s1 = if s.len() < 8 { Str8::empty() } else { Str8::from_str(&s[16..])? };
        Ok(Self(s0, s1))
    }
}

impl<'a> Deserialize<'a> for Str24 {
    fn deserialize<D: serde::Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        struct StrVisitor;

        impl<'a> Visitor<'a> for StrVisitor {
            type Value = Str24;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Str24, E>
            where
                E: de::Error,
            {
                v.parse().map_err(E::custom)
            }
        }

        d.deserialize_string(StrVisitor)
    }
}

impl Serialize for Str24 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&String::from(*self))
    }
}

#[macro_export]
macro_rules! s24 {
    ($s:literal) => {
        $crate::Str24::from_literal($s)
    };
    ($s:expr) => {
        $crate::Str24::from_literal($s)
    };
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn str() {
        assert_eq!("asdf", String::from(s24!("asdf")));
        assert_eq!("asdfasdf", String::from(s24!("asdfasdf")));
        assert_eq!("asdfasdfasdfasdf", String::from(s24!("asdfasdfasdfasdf")));
        assert_eq!("asdfasdfasdfasdfa", String::from(s24!("asdfasdfasdfasdfa")));
        assert_eq!("asdfasdfasdfasdfab", String::from(s24!("asdfasdfasdfasdfab")));
        assert_eq!("asdfasdfasdfasdfasdfasdf", String::from(s24!("asdfasdfasdfasdfasdfasdf")));
    }

    #[test]
    fn indexing() {
        let s0 = "abcdefghijklmnopqrstuvwx";
        let s1 = s24!("abcdefghijklmnopqrstuvwx");
        for i in 0..24 {
            assert_eq!(s0.chars().nth(i).unwrap(), s1.index(i));
        }
    }

    #[test]
    fn lowercase() {
        assert_eq!("asdf", String::from(s24!("ASDF").to_ascii_lowercase()));
        assert_eq!("asdfasdf", String::from(s24!("ASDFASDF").to_ascii_lowercase()));
        assert_eq!("asdfasdfasdfasdf", String::from(s24!("ASDFASDFASDFASDF").to_ascii_lowercase()));
        assert_eq!(
            "asdfasdfasdfasdfa",
            String::from(s24!("ASDFASDFASDFASDFA").to_ascii_lowercase())
        );
        assert_eq!(
            "asdfasdfasdfasdfab",
            String::from(s24!("ASDFASDFASDFASDFAB").to_ascii_lowercase())
        );
        assert_eq!(
            "asdfasdfasdfasdfasdfasdf",
            String::from(s24!("ASDFASDFASDFASDFASDFASDF").to_ascii_lowercase())
        );
    }

    #[test]
    fn uppercase() {
        assert_eq!("ASDF", String::from(s24!("asdf").to_ascii_uppercase()));
        assert_eq!("ASDFASDF", String::from(s24!("asdfasdf").to_ascii_uppercase()));
        assert_eq!("ASDFASDFASDFASDF", String::from(s24!("asdfasdfasdfasdf").to_ascii_uppercase()));
        assert_eq!(
            "ASDFASDFASDFASDFA",
            String::from(s24!("asdfasdfasdfasdfa").to_ascii_uppercase())
        );
        assert_eq!(
            "ASDFASDFASDFASDFAB",
            String::from(s24!("asdfasdfasdfasdfab").to_ascii_uppercase())
        );
        assert_eq!(
            "ASDFASDFASDFASDFASDFASDF",
            String::from(s24!("asdfasdfasdfasdfasdfasdf").to_ascii_uppercase())
        );
    }
}
