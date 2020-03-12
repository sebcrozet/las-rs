use num::Zero;
use std::str;
use {Error, Result};
use std::borrow::Cow;

pub trait AsLasStr {
    fn as_las_str(&self) -> Result<Cow<str>>;
}

pub trait FromLasStr {
    fn from_las_str(&mut self, s: &str) -> Result<()>;
}

pub fn some_or_none_if_zero<T: Zero>(n: T) -> Option<T> {
    if n.is_zero() {
        None
    } else {
        Some(n)
    }
}

impl<'a> AsLasStr for &'a [u8] {
    fn as_las_str(&self) -> Result<Cow<str>> {
        if let Some(position) = self.iter().position(|c| *c == 0) {
            if self[position..].iter().any(|c| *c != 0) {
                return Err(Error::NotZeroFilled(self.to_vec()));
            } else {
                Ok(String::from_utf8_lossy(&self[0..position]))
            }
        } else {
            Ok(String::from_utf8_lossy(self))
        }
    }
}

impl<'a> FromLasStr for &'a mut [u8] {
    fn from_las_str(&mut self, s: &str) -> Result<()> {
        if self.len() < s.bytes().count() {
            return Err(Error::StringTooLong(s.to_string(), self.len()));
        }
        for (a, b) in self.iter_mut().zip(s.bytes()) {
            *a = b;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_good_las_str() {
        let bytes = b"Beer!";
        assert_eq!("Beer!", bytes.as_ref().as_las_str().unwrap());
    }

    #[test]
    fn to_just_a_zero() {
        let bytes = [0];
        assert_eq!("", bytes.as_ref().as_las_str().unwrap());
    }

    #[test]
    fn to_not_nul_filled() {
        let bytes = [60, 0, 60];
        assert!(bytes.as_ref().as_las_str().is_err());
    }

    #[test]
    fn to_not_ascii() {
        let bytes = [0xf0, 0x9f, 0x8d, 0xba];
        assert!(bytes.as_ref().as_las_str().is_err());
    }

    #[test]
    fn from_good_las_str() {
        let mut bytes = [0; 5];
        bytes.as_mut().from_las_str("Beer!").unwrap();
        assert_eq!(b"Beer!", &bytes);
    }

    #[test]
    fn from_too_long() {
        let mut bytes = [0; 5];
        assert!(bytes.as_mut().from_las_str("Beer!!").is_err());
    }
}
