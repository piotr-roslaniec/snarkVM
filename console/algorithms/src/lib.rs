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

#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]

pub mod bhp;
pub use bhp::{BHP, BHP1024, BHP256, BHP512, BHP768};

mod blake2xs;
pub use blake2xs::Blake2Xs;

mod elligator2;
pub use elligator2::Elligator2;

mod nsec5;
pub use nsec5::NSEC5;

mod pedersen;
pub use pedersen::{Pedersen, Pedersen128, Pedersen64};

mod poseidon;
pub use poseidon::{Poseidon, Poseidon2, Poseidon4, Poseidon8};

pub mod traits;
pub use traits::*;
