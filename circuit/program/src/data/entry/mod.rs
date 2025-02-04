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

mod decrypt;
mod encrypt;
mod num_randomizers;
mod to_bits;

use crate::{Ciphertext, Plaintext, Visibility};
use snarkvm_circuit_network::Aleo;
use snarkvm_circuit_types::{environment::prelude::*, Boolean, Field};

/// An entry stored in program data.
#[derive(Clone)]
pub enum Entry<A: Aleo, Private: Visibility<A>> {
    /// A constant entry.
    Constant(Plaintext<A>),
    /// A publicly-visible entry.
    Public(Plaintext<A>),
    /// A private entry encrypted under the account owner's address.
    Private(Private),
}

#[cfg(console)]
impl<A: Aleo> Eject for Entry<A, Plaintext<A>> {
    type Primitive = console::Entry<A::Network, console::Plaintext<A::Network>>;

    /// Ejects the mode of the entry.
    fn eject_mode(&self) -> Mode {
        match self {
            Entry::Constant(_) => Mode::Constant,
            Entry::Public(_) => Mode::Public,
            Entry::Private(_) => Mode::Private,
        }
    }

    /// Ejects the entry.
    fn eject_value(&self) -> Self::Primitive {
        match self {
            Entry::Constant(plaintext) => console::Entry::Constant(plaintext.eject_value()),
            Entry::Public(plaintext) => console::Entry::Public(plaintext.eject_value()),
            Entry::Private(private) => console::Entry::Private(private.eject_value()),
        }
    }
}

// impl<A: Aleo, Literal: EntryMode<A>> Entry<A, Literal> {
//     // /// Returns the recursive depth of this entry.
//     // /// Note: Once `generic_const_exprs` is stabilized, this can be replaced with `const DEPTH: u8`.
//     // fn depth(&self, counter: usize) -> usize {
//     //     match self {
//     //         Self::Literal(..) => 1,
//     //         Self::Composite(composite) => {
//     //             // Determine the maximum depth of the composite.
//     //             let max_depth = composite.iter().map(|(_, entry)| entry.depth(counter)).fold(0, |a, b| a.max(b));
//     //             // Add `1` to the depth of the member with the largest depth.
//     //             max_depth.saturating_add(1)
//     //         }
//     //     }
//     // }
// }
