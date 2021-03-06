/*
    Copyright © 2020 Alastair Feille

    Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
    http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
    <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
    option. This file may not be copied, modified, or distributed
    except according to those terms.

    SPDX-License-Identifier: MIT OR Apache-2.0
*/

use std::{fmt,
          num::ParseIntError};

use serde::{Deserialize,
            Deserializer,
            Serialize,
            Serializer};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct UPC
{
    code: u64,
}

impl UPC
{
    pub fn new(barcode: &str) -> Result<UPC, UPCParseError>
    {
        if barcode.len() != 12
        {
            return Err(UPCParseError::Length(barcode.len()));
        }

        let e = UPC { code: barcode.parse::<u64>()?, };

        let calc = e.calculate_check_digit();
        let existing = e.check_digit();
        if calc != existing
        {
            return Err(UPCParseError::CheckDigit(calc, existing));
        }

        Ok(e)
    }

    fn calculate_check_digit(&self) -> u8
    {
        let mut digits: Vec<u8> = self.to_string()
                                      .chars()
                                      .map(|c| c.to_digit(10).unwrap() as u8)
                                      .collect();
        // Remove the existing check digit
        digits.pop();

        let partial_sum: u8 = digits.iter()
                                    .enumerate()
                                    .map(|(i, e)| {
                                        if i % 2 == 0
                                        {
                                            e * 3
                                        }
                                        else
                                        {
                                            e * 1
                                        }
                                    })
                                    .sum();
        let m = partial_sum % 10;
        let check_digit = if m != 0 { 10 - m } else { m };
        check_digit as u8
    }

    pub fn check_digit(&self) -> u8
    {
        (self.code % 10) as u8
    }
}

impl fmt::Display for UPC
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:012}", self.code)
    }
}

impl Serialize for UPC
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}", *self))
    }
}

impl<'de> Deserialize<'de> for UPC
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;

        // Deserialize the string
        let s = String::deserialize(deserializer)?;

        // Create a new UPC from String
        UPC::new(&s).map_err(|e| D::Error::custom(format!("{:?}", e)))
    }
}

#[derive(Debug)]
pub enum UPCParseError
{
    Length(usize),
    Parse(ParseIntError),
    CheckDigit(u8, u8),
}

impl From<ParseIntError> for UPCParseError
{
    fn from(err: ParseIntError) -> UPCParseError
    {
        UPCParseError::Parse(err)
    }
}
