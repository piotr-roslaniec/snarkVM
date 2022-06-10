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

use crate::{errors::SynthesisError, ConstraintSystem, LinearCombination, LookupTable, Variable};
use snarkvm_fields::Field;

use std::marker::PhantomData;

/// This is a "namespaced" constraint system which borrows a constraint system
/// (pushing a namespace context) and, when dropped, pops out of the namespace context.
pub struct Namespace<'a, F: Field, CS: ConstraintSystem<F>>(pub(super) &'a mut CS, pub(super) PhantomData<F>);

impl<F: Field, CS: ConstraintSystem<F>> ConstraintSystem<F> for Namespace<'_, F, CS> {
    type Root = CS::Root;

    #[inline]
    fn one() -> Variable {
        CS::one()
    }

    #[inline]
    fn add_lookup_table(&mut self, lookup_table: LookupTable<F>) -> Result<(), SynthesisError> {
        self.0.add_lookup_table(lookup_table)
    }

    #[inline]
    fn alloc<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable, SynthesisError>
    where
        FN: FnOnce() -> Result<F, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        self.0.alloc(annotation, f)
    }

    #[inline]
    fn alloc_input<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable, SynthesisError>
    where
        FN: FnOnce() -> Result<F, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        self.0.alloc_input(annotation, f)
    }

    #[inline]
    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
    {
        self.0.enforce(annotation, a, b, c)
    }

    #[inline]
    fn lookup(&mut self, val: LinearCombination<F>) -> Result<Variable, SynthesisError> {
        self.0.lookup(val)
    }

    // Downstream users who use `namespace` will never interact with these
    // functions and they will never be invoked because the namespace is
    // never a root constraint system.

    #[inline]
    fn push_namespace<NR, N>(&mut self, _: N)
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR,
    {
        panic!("only the root's push_namespace should be called");
    }

    #[inline]
    fn pop_namespace(&mut self) {
        panic!("only the root's pop_namespace should be called");
    }

    #[inline]
    fn get_root(&mut self) -> &mut Self::Root {
        self.0.get_root()
    }

    #[inline]
    fn num_constraints(&self) -> usize {
        self.0.num_constraints()
    }

    #[inline]
    fn num_public_variables(&self) -> usize {
        self.0.num_public_variables()
    }

    #[inline]
    fn num_private_variables(&self) -> usize {
        self.0.num_private_variables()
    }

    #[inline]
    fn is_in_setup_mode(&self) -> bool {
        self.0.is_in_setup_mode()
    }
}

impl<F: Field, CS: ConstraintSystem<F>> Drop for Namespace<'_, F, CS> {
    #[inline]
    fn drop(&mut self) {
        self.get_root().pop_namespace()
    }
}
