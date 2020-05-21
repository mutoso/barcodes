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
pub struct PLU
{
    code: u32,
}

impl PLU
{
    pub fn new(plu: &str) -> Result<PLU, PLUParseError>
    {
        if plu.len() < 4 || plu.len() > 5
        {
            return Err(PLUParseError::Length(plu.len()));
        }

        let p = PLU { code: plu.parse::<u32>()?, };

        Ok(p)
    }
}

impl fmt::Display for PLU
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:04}", self.code)
    }
}

impl Serialize for PLU
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}", *self))
    }
}

impl<'de> Deserialize<'de> for PLU
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;

        // Deserialize the string
        let s = String::deserialize(deserializer)?;

        // Create a new PLU from String
        PLU::new(&s).map_err(|e| D::Error::custom(format!("{:?}", e)))
    }
}

#[derive(Debug)]
pub enum PLUParseError
{
    Length(usize),
    Parse(ParseIntError),
}

impl From<ParseIntError> for PLUParseError
{
    fn from(err: ParseIntError) -> PLUParseError
    {
        PLUParseError::Parse(err)
    }
}
