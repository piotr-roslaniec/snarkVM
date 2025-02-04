// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

mod from_bits;
mod from_fields;
mod size_in_fields;
mod to_bits;
mod to_fields;

use crate::{FromFields, ToFields, Visibility};
use snarkvm_console_network::Network;
use snarkvm_fields::PrimeField;
use snarkvm_utilities::{FromBits, ToBits};

use anyhow::{bail, Error, Result};
use core::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ciphertext<N: Network>(Vec<N::Field>);

impl<N: Network> Deref for Ciphertext<N> {
    type Target = [N::Field];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
