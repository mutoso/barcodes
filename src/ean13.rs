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

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct EAN13
{
    code: u64,
}

impl EAN13
{
    pub fn new(barcode: &str) -> Result<EAN13, ParseIntError>
    {
        Ok(EAN13 { code: barcode.parse::<u64>()?, })
    }
}

impl fmt::Display for EAN13
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:013}", self.code)
    }
}
