use std::fmt;
use std::ops::Index;
use std::str::FromStr;

use derive_more::Display;
use eyre::{Result, eyre};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};

#[must_use]
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Display)]
#[display("{}", String::from(*self))]
pub struct StrN<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Default for StrN<N> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const N: usize> fmt::Debug for StrN<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<const N: usize> StrN<N> {
    pub const fn empty() -> Self {
        Self { data: [0; N] }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data[0] == 0
    }

    #[must_use]
    pub const fn max_size() -> usize {
        N
    }

    #[must_use]
    pub const fn index(&self, i: usize) -> u8 {
        self.data[i]
    }

    pub const fn from_array(s: [u8; N]) -> Self {
        Self { data: s }
    }

    pub fn from_bytes(s: &[u8]) -> Result<Self> {
        if s.len() > N {
            return Err(eyre!("too many bytes"));
        }
        let mut data = [0; N];
        data[..s.len()].copy_from_slice(s);
        Ok(Self { data })
    }

    pub const fn from_literal(s: &'static str) -> Self {
        assert!(s.len() <= N, "too many bytes to make sid");
        let s = s.as_bytes();
        let mut data = [0; N];
        let mut sz = if s.len() < N { s.len() } else { N };
        while sz > 0 {
            sz -= 1;
            data[sz] = s[sz];
        }
        Self { data }
    }

    pub fn to_ascii_lowercase(mut self) -> Self {
        self.data.make_ascii_lowercase();
        self
    }

    pub const fn to_ascii_lowercase_const(self) -> Self {
        let mut data = self.data;
        let mut i = 0;
        while i < N {
            data[i] = data[i].to_ascii_lowercase();
            i += 1;
        }
        Self { data }
    }

    pub fn to_ascii_uppercase(mut self) -> Self {
        self.data.make_ascii_uppercase();
        self
    }

    pub const fn to_ascii_uppercase_const(self) -> Self {
        let mut data = self.data;
        let mut i = 0;
        while i < N {
            data[i] = data[i].to_ascii_uppercase();
            i += 1;
        }
        Self { data }
    }

    #[must_use]
    pub fn starts_with(&self, s: StrN<N>) -> bool {
        self.data.starts_with(&s.data)
    }

    #[must_use]
    pub fn starts_with_const(&self, s: StrN<N>) -> bool {
        let mut i = 0;
        while i < N {
            if s.index(i) == 0 {
                return true;
            }
            if self.index(i) != s.index(i) {
                return false;
            }
            i += 1;
        }
        true
    }

    #[must_use]
    pub const fn contains_ascii(&self, c: u8) -> bool {
        let mut i = 0;
        while i < N {
            if self.index(i) == 0 {
                return false;
            }
            if self.index(i) == c {
                return true;
            }
            i += 1;
        }
        false
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        let null_position = self.data.iter().position(|&v| v == 0).unwrap_or(N);
        std::str::from_utf8(&self.data[..null_position]).unwrap()
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        let null_position = self.data.iter().position(|&v| v == 0).unwrap_or(N);
        self.data[..null_position].to_vec()
    }
}

impl<const N: usize> AsRef<[u8]> for StrN<N> {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl<const N: usize> From<StrN<N>> for String {
    fn from(v: StrN<N>) -> Self {
        let sz = v.data.iter().position(|&v| v == 0).unwrap_or(N);
        String::from_utf8_lossy(&v.data[..sz]).into()
    }
}

impl<const N: usize> FromStr for StrN<N> {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bytes(s.as_bytes())
    }
}

impl<const N: usize> Index<usize> for StrN<N> {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<'a, const N: usize> Deserialize<'a> for StrN<N> {
    fn deserialize<D: serde::Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        struct StrVisitor<const N: usize>;

        impl<const N: usize> Visitor<'_> for StrVisitor<N> {
            type Value = StrN<N>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("string")
            }

            fn visit_str<E>(self, v: &str) -> Result<StrN<N>, E>
            where
                E: de::Error,
            {
                v.parse().map_err(E::custom)
            }
        }

        d.deserialize_string(StrVisitor)
    }
}

impl<const N: usize> Serialize for StrN<N> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&String::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::s23;

    #[test]
    fn str() {
        assert_eq!("asdf", String::from(s23!("asdf")));
        assert_eq!("asdfa", String::from(s23!("asdfa")));
        assert_eq!("asdfasdf", String::from(s23!("asdfasdf")));
    }

    #[test]
    fn indexing() {
        let s0 = "abcdefgh";
        let s1 = s23!("abcdefgh");
        for i in 0..8 {
            assert_eq!(s0.chars().nth(i).unwrap(), s1.index(i) as char);
        }
    }

    #[test]
    fn lowercase() {
        assert_eq!("asdf", String::from(s23!("ASDF").to_ascii_lowercase()));
        assert_eq!("asdfa", String::from(s23!("ASDFA").to_ascii_lowercase()));
        assert_eq!("asdfasdf", String::from(s23!("ASDFASDF").to_ascii_lowercase()));
    }

    #[test]
    fn uppercase() {
        assert_eq!("ASDF", String::from(s23!("asdf").to_ascii_uppercase()));
        assert_eq!("ASDFA", String::from(s23!("asdfa").to_ascii_uppercase()));
        assert_eq!("ASDFASDF", String::from(s23!("asdfasdf").to_ascii_uppercase()));
    }
}
