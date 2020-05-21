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

use crate::upc::UPC;

use serde::{Deserialize,
            Deserializer,
            Serialize,
            Serializer};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct EAN13
{
    code: u64,
}

impl EAN13
{
    pub fn new(barcode: &str) -> Result<EAN13, EAN13ParseError>
    {
        if barcode.len() != 13
        {
            return Err(EAN13ParseError::Length(barcode.len()));
        }

        let e = EAN13 { code: barcode.parse::<u64>()?, };

        let calc = e.calculate_check_digit();
        let existing = e.check_digit();
        if calc != existing
        {
            return Err(EAN13ParseError::CheckDigit(calc, existing));
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
                                        if i % 2 == 1
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

impl fmt::Display for EAN13
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:013}", self.code)
    }
}

impl From<UPC> for EAN13
{
    fn from(u: UPC) -> EAN13
    {
        EAN13 { code: u.to_string().parse::<u64>().unwrap(), }
    }
}

impl Serialize for EAN13
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}", *self))
    }
}

impl<'de> Deserialize<'de> for EAN13
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;

        // Deserialize the string
        let s = String::deserialize(deserializer)?;

        // Create a new EAN13 from String
        EAN13::new(&s).map_err(|e| D::Error::custom(format!("{:?}", e)))
    }
}

#[derive(Debug)]
pub enum EAN13ParseError
{
    Length(usize),
    Parse(ParseIntError),
    CheckDigit(u8, u8),
}

impl From<ParseIntError> for EAN13ParseError
{
    fn from(err: ParseIntError) -> EAN13ParseError
    {
        EAN13ParseError::Parse(err)
    }
}
