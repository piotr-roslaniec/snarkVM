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

use crate::{FromFields, Identifier, Literal, ToFields, Visibility};
use snarkvm_console_network::Network;
use snarkvm_fields::PrimeField;
use snarkvm_utilities::{FromBits, ToBits};

use anyhow::{bail, Error, Result};
use once_cell::sync::OnceCell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Plaintext<N: Network> {
    /// A literal.
    Literal(Literal<N>, OnceCell<Vec<bool>>),
    /// A composite.
    Composite(Vec<(Identifier<N>, Plaintext<N>)>, OnceCell<Vec<bool>>),
}

impl<N: Network> From<Literal<N>> for Plaintext<N> {
    /// Returns a new `Plaintext` from a `Literal`.
    fn from(literal: Literal<N>) -> Self {
        Self::Literal(literal, OnceCell::new())
    }
}

impl<N: Network> From<&Literal<N>> for Plaintext<N> {
    /// Returns a new `Plaintext` from a `Literal`.
    fn from(literal: &Literal<N>) -> Self {
        Self::Literal(literal.clone(), OnceCell::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::Testnet3;
    use snarkvm_utilities::{test_rng, UniformRand};

    use core::str::FromStr;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_plaintext() -> Result<()> {
        let value = Plaintext::<CurrentNetwork>::Literal(Literal::Boolean(true), OnceCell::new());
        assert_eq!(value.to_bits_le(), Plaintext::<CurrentNetwork>::from_bits_le(&value.to_bits_le())?.to_bits_le());

        let value =
            Plaintext::<CurrentNetwork>::Literal(Literal::Field(UniformRand::rand(&mut test_rng())), OnceCell::new());
        assert_eq!(value.to_bits_le(), Plaintext::<CurrentNetwork>::from_bits_le(&value.to_bits_le())?.to_bits_le());

        let value = Plaintext::<CurrentNetwork>::Composite(
            vec![
                (
                    Identifier::from_str("a")?,
                    Plaintext::<CurrentNetwork>::Literal(Literal::Boolean(true), OnceCell::new()),
                ),
                (
                    Identifier::from_str("b")?,
                    Plaintext::<CurrentNetwork>::Literal(
                        Literal::Field(UniformRand::rand(&mut test_rng())),
                        OnceCell::new(),
                    ),
                ),
            ],
            OnceCell::new(),
        );
        assert_eq!(value.to_bits_le(), Plaintext::<CurrentNetwork>::from_bits_le(&value.to_bits_le())?.to_bits_le());

        let value = Plaintext::<CurrentNetwork>::Composite(
            vec![
                (
                    Identifier::from_str("a")?,
                    Plaintext::<CurrentNetwork>::Literal(Literal::Boolean(true), OnceCell::new()),
                ),
                (
                    Identifier::from_str("b")?,
                    Plaintext::<CurrentNetwork>::Composite(
                        vec![
                            (
                                Identifier::from_str("c")?,
                                Plaintext::<CurrentNetwork>::Literal(Literal::Boolean(true), OnceCell::new()),
                            ),
                            (
                                Identifier::from_str("d")?,
                                Plaintext::<CurrentNetwork>::Composite(
                                    vec![
                                        (
                                            Identifier::from_str("e")?,
                                            Plaintext::<CurrentNetwork>::Literal(
                                                Literal::Boolean(true),
                                                OnceCell::new(),
                                            ),
                                        ),
                                        (
                                            Identifier::from_str("f")?,
                                            Plaintext::<CurrentNetwork>::Literal(
                                                Literal::Field(UniformRand::rand(&mut test_rng())),
                                                OnceCell::new(),
                                            ),
                                        ),
                                    ],
                                    OnceCell::new(),
                                ),
                            ),
                            (
                                Identifier::from_str("g")?,
                                Plaintext::<CurrentNetwork>::Literal(
                                    Literal::Field(UniformRand::rand(&mut test_rng())),
                                    OnceCell::new(),
                                ),
                            ),
                        ],
                        OnceCell::new(),
                    ),
                ),
                (
                    Identifier::from_str("h")?,
                    Plaintext::<CurrentNetwork>::Literal(
                        Literal::Field(UniformRand::rand(&mut test_rng())),
                        OnceCell::new(),
                    ),
                ),
            ],
            OnceCell::new(),
        );
        assert_eq!(value.to_bits_le(), Plaintext::<CurrentNetwork>::from_bits_le(&value.to_bits_le())?.to_bits_le());
        Ok(())
    }
}
