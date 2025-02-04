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

use super::*;

impl<N: Network> FromBits for Plaintext<N> {
    /// Initializes a new value from a list of little-endian bits *without* trailing zeros.
    fn from_bits_le(bits_le: &[bool]) -> Result<Self> {
        let mut counter = 0;

        let is_literal = !bits_le[counter];
        counter += 1;

        // Literal
        if is_literal {
            let literal_variant = u8::from_bits_le(&bits_le[counter..counter + 8])?;
            counter += 8;

            let literal_size = u16::from_bits_le(&bits_le[counter..counter + 16])?;
            counter += 16;

            let literal = Literal::from_bits_le(literal_variant, &bits_le[counter..counter + literal_size as usize])?;

            // Store the plaintext bits in the cache.
            let cache = OnceCell::new();
            match cache.set(bits_le.to_vec()) {
                // Return the literal.
                Ok(_) => Ok(Self::Literal(literal, cache)),
                Err(_) => bail!("Failed to store the plaintext bits in the cache."),
            }
        }
        // Composite
        else {
            let num_composites = u8::from_bits_le(&bits_le[counter..counter + 8])?;
            counter += 8;

            let mut composites = Vec::with_capacity(num_composites as usize);
            for _ in 0..num_composites {
                let identifier_size = u8::from_bits_le(&bits_le[counter..counter + 8])?;
                counter += 8;

                let identifier = Identifier::from_bits_le(&bits_le[counter..counter + identifier_size as usize])?;
                counter += identifier_size as usize;

                let composite_size = u16::from_bits_le(&bits_le[counter..counter + 16])?;
                counter += 16;

                let entry = Plaintext::from_bits_le(&bits_le[counter..counter + composite_size as usize])?;
                counter += composite_size as usize;

                composites.push((identifier, entry));
            }

            // Store the plaintext bits in the cache.
            let cache = OnceCell::new();
            match cache.set(bits_le.to_vec()) {
                // Return the composite.
                Ok(_) => Ok(Self::Composite(composites, cache)),
                Err(_) => bail!("Failed to store the plaintext bits in the cache."),
            }
        }
    }

    /// Initializes a new value from a list of big-endian bits *without* trailing zeros.
    fn from_bits_be(bits_be: &[bool]) -> Result<Self> {
        let mut counter = 0;

        let is_literal = !bits_be[counter];
        counter += 1;

        // Literal
        if is_literal {
            let literal_variant = u8::from_bits_be(&bits_be[counter..counter + 8])?;
            counter += 8;

            let literal_size = u16::from_bits_be(&bits_be[counter..counter + 16])?;
            counter += 16;

            let literal = Literal::from_bits_be(literal_variant, &bits_be[counter..counter + literal_size as usize])?;

            // Store the plaintext bits in the cache.
            let cache = OnceCell::new();
            match cache.set(bits_be.to_vec()) {
                // Return the literal.
                Ok(_) => Ok(Self::Literal(literal, cache)),
                Err(_) => bail!("Failed to store the plaintext bits in the cache."),
            }
        }
        // Composite
        else {
            let num_composites = u8::from_bits_be(&bits_be[counter..counter + 8])?;
            counter += 8;

            let mut composites = Vec::with_capacity(num_composites as usize);
            for _ in 0..num_composites {
                let identifier_size = u8::from_bits_be(&bits_be[counter..counter + 8])?;
                counter += 8;

                let identifier = Identifier::from_bits_be(&bits_be[counter..counter + identifier_size as usize])?;
                counter += identifier_size as usize;

                let composite_size = u16::from_bits_be(&bits_be[counter..counter + 16])?;
                counter += 16;

                let entry = Plaintext::from_bits_be(&bits_be[counter..counter + composite_size as usize])?;
                counter += composite_size as usize;

                composites.push((identifier, entry));
            }

            // Store the plaintext bits in the cache.
            let cache = OnceCell::new();
            match cache.set(bits_be.to_vec()) {
                // Return the composite.
                Ok(_) => Ok(Self::Composite(composites, cache)),
                Err(_) => bail!("Failed to store the plaintext bits in the cache."),
            }
        }
    }
}
