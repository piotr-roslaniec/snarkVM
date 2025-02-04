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

impl<G: AffineCurve, const NUM_BITS: u8> CommitUncompressed for Pedersen<G, NUM_BITS> {
    type Input = bool;
    type Output = G;
    type Randomizer = G::ScalarField;

    /// Returns the Pedersen commitment of the given input and randomizer as an affine group element.
    fn commit_uncompressed(&self, input: &[Self::Input], randomizer: &Self::Randomizer) -> Result<Self::Output> {
        let mut output = self.hash_uncompressed(input)?.to_projective();

        // Compute h^r.
        randomizer.to_bits_le().iter().zip_eq(&*self.random_base_window).filter(|(bit, _)| **bit).for_each(
            |(_, base)| {
                output += base;
            },
        );

        Ok(output.to_affine())
    }
}
