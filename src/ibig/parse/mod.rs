//! Parsing numbers.

use crate::ibig::{
    error::ParseError,
    ibig::IBig,
    radix::{self, Digit},
    sign::Sign::*,
    ubig::UBig,
};
use core::str::FromStr;
use std::time::Instant;

pub mod non_power_two;
pub mod power_two;
pub mod parsebytes;

impl FromStr for UBig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<UBig, ParseError> {
        UBig::from_str_radix(s, 10)
    }
}

impl FromStr for IBig {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<IBig, ParseError> {
        IBig::from_str_radix(s, 10)
    }
}

impl UBig {
    /// Convert a string in a given base to [UBig].
    ///
    /// `src` may contain an optional `+` prefix.
    /// Digits 10-35 are represented by `a-z` or `A-Z`.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    /// ```
    /// # use ibig::{error::ParseError, ubig, UBig};
    /// assert_eq!(UBig::from_str_radix("+7ab", 32)?, ubig!(7499));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_str_radix(src: &str, radix: u32) -> Result<UBig, ParseError> {
        radix::check_radix_valid(radix);
        let src = src.strip_prefix('+').unwrap_or(src);
        UBig::from_str_radix_no_sign(src, radix)
    }

    /// Convert a string with an optional radix prefix to [UBig].
    ///
    /// `src` may contain an optional `+` after the radix prefix.
    ///
    /// Allowed prefixes: `0b` for binary, `0o` for octal, `0x` for hexadecimal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{error::ParseError, ubig, UBig};
    /// assert_eq!(UBig::from_str_with_radix_prefix("+0o17")?, ubig!(0o17));
    /// assert_eq!(UBig::from_str_with_radix_prefix("0x1f")?, ubig!(0x1f));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_str_with_radix_prefix(src: &str) -> Result<UBig, ParseError> {
        let src = src.strip_prefix('+').unwrap_or(src);
        UBig::from_str_with_radix_prefix_no_sign(src)
    }

    /// Convert an unsigned string with an optional radix prefix to [UBig].
    fn from_str_with_radix_prefix_no_sign(src: &str) -> Result<UBig, ParseError> {
        if let Some(bin) = src.strip_prefix("0b") {
            UBig::from_str_radix_no_sign(bin, 2)
        } else if let Some(oct) = src.strip_prefix("0o") {
            UBig::from_str_radix_no_sign(oct, 8)
        } else if let Some(hex) = src.strip_prefix("0x") {
            UBig::from_str_radix_no_sign(hex, 16)
        } else {
            UBig::from_str_radix_no_sign(src, 10)
        }
    }

    /// Convert an unsigned string to [UBig].
    fn from_str_radix_no_sign(mut src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(radix::is_radix_valid(radix));
        if src.is_empty() {
            return Err(ParseError::NoDigits);
        }

        while let Some(src2) = src.strip_prefix('0') {
            src = src2;
        }

        if radix.is_power_of_two() {
            power_two::parse(src, radix)
        } else {
            non_power_two::parse(src, radix) // it took 1 - 2 micro second
        } // it took 1 - 3 micro sec;
    }

    fn from_bytes_radix_no_sign(mut strbytes : &[u8], mut numindex : usize, radix : Digit) -> Result<UBig, ParseError> {
        if strbytes.is_empty() {
            return Err(ParseError::NoDigits);
        }

        while  strbytes[numindex] == b'0' {
            numindex += 1;
        }

        if radix.is_power_of_two() {
            power_two::parse2(&strbytes[numindex..], radix)
            // non_power_two::parse2(&strbytes[numindex..], radix)
        } else {
            non_power_two::parse2(&strbytes[numindex..], radix) // it took 1 - 2 micro second
        } // it took 1 - 3 micro sec;
    }
}

impl IBig {
    /// Convert a string in a given base to [IBig].
    ///
    /// The string may contain a `+` or `-` prefix.
    /// Digits 10-35 are represented by `a-z` or `A-Z`.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    /// ```
    /// # use ibig::{error::ParseError, ibig, IBig};
    /// assert_eq!(IBig::from_str_radix("-7ab", 32)?, ibig!(-7499));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_str_radix1(mut src: &str, radix: u32) -> Result<IBig, ParseError> {
        radix::check_radix_valid(radix); // took 11 - 12 micro secs
        let sign;
        match src.strip_prefix('-') {
            Some(s) => {
                sign = Negative;
                src = s;
            }
            None => {
                sign = Positive;
                src = src.strip_prefix('+').unwrap_or(src);
            }
        } // till here takes 40 - 50 micro second
        let mag = UBig::from_str_radix_no_sign(src, radix)?; 
        Ok(IBig::from_sign_magnitude(sign, mag) /* takes < 0.5 micro seconds */)
    }

    pub fn from_str_radix(src: &str, radix: u32) -> Result<IBig, ParseError> {
        radix::check_radix_valid(radix); // took 11 - 12 micro secs
        let sign;
        let strbytes = src.as_bytes(); // bytes of str
        // index after sign byte. Like for -123, it will 1 and 123 it will 0
        let numindex = match strbytes[0] {
            b'-' => {
                sign = Negative;
                1
            },
            b'+' => {
                sign = Positive;
                1
            },
            _ => {
                sign = Positive;
                0
            }
        }; // till here takes 40 - 50 micro second
        let mag = UBig::from_bytes_radix_no_sign(strbytes, numindex, radix)?; 
        Ok(IBig::from_sign_magnitude(sign, mag) /* takes < 0.5 micro seconds */)
    }

    /// Convert a string with an optional radix prefix to [IBig].
    ///
    /// `src` may contain an '+' or `-` prefix after the radix prefix.
    ///
    /// Allowed prefixes: `0b` for binary, `0o` for octal, `0x` for hexadecimal.
    ///
    /// # Examples
    /// ```
    /// # use ibig::{error::ParseError, ibig, IBig};
    /// assert_eq!(IBig::from_str_with_radix_prefix("+0o17")?, ibig!(0o17));
    /// assert_eq!(IBig::from_str_with_radix_prefix("-0x1f")?, ibig!(-0x1f));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_str_with_radix_prefix(mut src: &str) -> Result<IBig, ParseError> {
        let sign;
        match src.strip_prefix('-') {
            Some(s) => {
                sign = Negative;
                src = s;
            }
            None => {
                sign = Positive;
                src = src.strip_prefix('+').unwrap_or(src);
            }
        }
        let mag = UBig::from_str_with_radix_prefix_no_sign(src)?;
        Ok(IBig::from_sign_magnitude(sign, mag))
    }
}
