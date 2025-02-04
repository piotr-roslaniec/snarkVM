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

use crate::Aleo;
use snarkvm_circuit_algorithms::{
    Commit,
    Hash,
    HashMany,
    HashToScalar,
    Pedersen128,
    Pedersen64,
    Poseidon2,
    Poseidon4,
    Poseidon8,
    BHP1024,
    BHP256,
    BHP512,
    BHP768,
    PRF,
};
use snarkvm_circuit_types::{
    environment::{prelude::*, Circuit},
    Boolean,
    Field,
    Group,
    Scalar,
};
use snarkvm_curves::ProjectiveCurve;
use snarkvm_fields::FieldParameters;

use core::fmt;

type E = Circuit;

thread_local! {
    /// The group bases for the Aleo signature and encryption schemes.
    static GENERATOR_G: Vec<Group<AleoV0>> = Vec::constant(<console::Testnet3 as console::Network>::g_powers().iter().map(|g| g.to_affine()).collect());
    /// The encryption domain as a constant field element.
    static ENCRYPTION_DOMAIN: Field<AleoV0> = Field::constant(<console::Testnet3 as console::Network>::encryption_domain());
    /// The MAC domain as a constant field element.
    static MAC_DOMAIN: Field<AleoV0> = Field::constant(<console::Testnet3 as console::Network>::mac_domain());
    /// The randomizer domain as a constant field element.
    static RANDOMIZER_DOMAIN: Field<AleoV0> = Field::constant(<console::Testnet3 as console::Network>::randomizer_domain());

    /// The BHP gadget, which can take an input of up to 256 bits.
    static BHP_256: BHP256<AleoV0> = BHP256::<AleoV0>::constant(console::BHP_256.with(|bhp| bhp.clone()));
    /// The BHP gadget, which can take an input of up to 512 bits.
    static BHP_512: BHP512<AleoV0> = BHP512::<AleoV0>::constant(console::BHP_512.with(|bhp| bhp.clone()));
    /// The BHP gadget, which can take an input of up to 768 bits.
    static BHP_768: BHP768<AleoV0> = BHP768::<AleoV0>::constant(console::BHP_768.with(|bhp| bhp.clone()));
    /// The BHP gadget, which can take an input of up to 1024 bits.
    static BHP_1024: BHP1024<AleoV0> = BHP1024::<AleoV0>::constant(console::BHP_1024.with(|bhp| bhp.clone()));

    /// The Pedersen gadget, which can take an input of up to 64 bits.
    static PEDERSEN_64: Pedersen64<AleoV0> = Pedersen64::<AleoV0>::constant(console::PEDERSEN_64.with(|pedersen| pedersen.clone()));
    /// The Pedersen gadget, which can take an input of up to 128 bits.
    static PEDERSEN_128: Pedersen128<AleoV0> = Pedersen128::<AleoV0>::constant(console::PEDERSEN_128.with(|pedersen| pedersen.clone()));

    /// The Poseidon hash function, using a rate of 2.
    static POSEIDON_2: Poseidon2<AleoV0> = Poseidon2::<AleoV0>::constant(console::POSEIDON_2.with(|poseidon| poseidon.clone()));
    /// The Poseidon hash function, using a rate of 4.
    static POSEIDON_4: Poseidon4<AleoV0> = Poseidon4::<AleoV0>::constant(console::POSEIDON_4.with(|poseidon| poseidon.clone()));
    /// The Poseidon hash function, using a rate of 8.
    static POSEIDON_8: Poseidon8<AleoV0> = Poseidon8::<AleoV0>::constant(console::POSEIDON_8.with(|poseidon| poseidon.clone()));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AleoV0;

impl Aleo for AleoV0 {
    type Network = console::Testnet3;

    /// The maximum number of bits in data (must not exceed u16::MAX).
    const MAX_DATA_SIZE_IN_FIELDS: u32 = (128 * 1024 * 8) / <Self::BaseField as PrimeField>::Parameters::CAPACITY;

    /// Returns the encryption domain as a constant field element.
    fn encryption_domain() -> Field<Self> {
        ENCRYPTION_DOMAIN.with(|domain| domain.clone())
    }

    /// Returns the MAC domain as a constant field element.
    fn mac_domain() -> Field<Self> {
        MAC_DOMAIN.with(|domain| domain.clone())
    }

    /// Returns the randomizer domain as a constant field element.
    fn randomizer_domain() -> Field<Self> {
        RANDOMIZER_DOMAIN.with(|domain| domain.clone())
    }

    /// Returns the scalar multiplication on the group bases.
    #[inline]
    fn g_scalar_multiply(scalar: &Scalar<Self>) -> Group<Self> {
        GENERATOR_G.with(|bases| {
            bases
                .iter()
                .zip_eq(&scalar.to_bits_le())
                .fold(Group::zero(), |output, (base, bit)| Group::ternary(bit, &(&output + base), &output))
        })
    }

    /// Returns a BHP commitment for the given (up to) 256-bit input and randomizer.
    fn commit_bhp256(input: &[Boolean<Self>], randomizer: &Scalar<Self>) -> Field<Self> {
        BHP_256.with(|bhp| bhp.commit(input, randomizer))
    }

    /// Returns a BHP commitment for the given (up to) 512-bit input and randomizer.
    fn commit_bhp512(input: &[Boolean<Self>], randomizer: &Scalar<Self>) -> Field<Self> {
        BHP_512.with(|bhp| bhp.commit(input, randomizer))
    }

    /// Returns a BHP commitment for the given (up to) 768-bit input and randomizer.
    fn commit_bhp768(input: &[Boolean<Self>], randomizer: &Scalar<Self>) -> Field<Self> {
        BHP_768.with(|bhp| bhp.commit(input, randomizer))
    }

    /// Returns a BHP commitment for the given (up to) 1024-bit input and randomizer.
    fn commit_bhp1024(input: &[Boolean<Self>], randomizer: &Scalar<Self>) -> Field<Self> {
        BHP_1024.with(|bhp| bhp.commit(input, randomizer))
    }

    /// Returns a Pedersen commitment for the given (up to) 64-bit input and randomizer.
    fn commit_ped64(input: &[Boolean<Self>], randomizer: &Scalar<Self>) -> Field<Self> {
        PEDERSEN_64.with(|pedersen| pedersen.commit(input, randomizer))
    }

    /// Returns a Pedersen commitment for the given (up to) 128-bit input and randomizer.
    fn commit_ped128(input: &[Boolean<Self>], randomizer: &Scalar<Self>) -> Field<Self> {
        PEDERSEN_128.with(|pedersen| pedersen.commit(input, randomizer))
    }

    /// Returns the BHP hash for a given (up to) 256-bit input.
    fn hash_bhp256(input: &[Boolean<Self>]) -> Field<Self> {
        BHP_256.with(|bhp| bhp.hash(input))
    }

    /// Returns the BHP hash for a given (up to) 512-bit input.
    fn hash_bhp512(input: &[Boolean<Self>]) -> Field<Self> {
        BHP_512.with(|bhp| bhp.hash(input))
    }

    /// Returns the BHP hash for a given (up to) 768-bit input.
    fn hash_bhp768(input: &[Boolean<Self>]) -> Field<Self> {
        BHP_768.with(|bhp| bhp.hash(input))
    }

    /// Returns the BHP hash for a given (up to) 1024-bit input.
    fn hash_bhp1024(input: &[Boolean<Self>]) -> Field<Self> {
        BHP_1024.with(|bhp| bhp.hash(input))
    }

    /// Returns the Pedersen hash for a given (up to) 64-bit input.
    fn hash_ped64(input: &[Boolean<Self>]) -> Field<Self> {
        PEDERSEN_64.with(|pedersen| pedersen.hash(input))
    }

    /// Returns the Pedersen hash for a given (up to) 128-bit input.
    fn hash_ped128(input: &[Boolean<Self>]) -> Field<Self> {
        PEDERSEN_128.with(|pedersen| pedersen.hash(input))
    }

    /// Returns the Poseidon hash with an input rate of 2.
    fn hash_psd2(input: &[Field<Self>]) -> Field<Self> {
        POSEIDON_2.with(|poseidon| poseidon.hash(input))
    }

    /// Returns the Poseidon hash with an input rate of 4.
    fn hash_psd4(input: &[Field<Self>]) -> Field<Self> {
        POSEIDON_4.with(|poseidon| poseidon.hash(input))
    }

    /// Returns the Poseidon hash with an input rate of 8.
    fn hash_psd8(input: &[Field<Self>]) -> Field<Self> {
        POSEIDON_8.with(|poseidon| poseidon.hash(input))
    }

    /// Returns the extended Poseidon hash with an input rate of 2.
    fn hash_many_psd2(input: &[Field<Self>], num_outputs: u16) -> Vec<Field<Self>> {
        POSEIDON_2.with(|poseidon| poseidon.hash_many(input, num_outputs))
    }

    /// Returns the extended Poseidon hash with an input rate of 4.
    fn hash_many_psd4(input: &[Field<Self>], num_outputs: u16) -> Vec<Field<Self>> {
        POSEIDON_4.with(|poseidon| poseidon.hash_many(input, num_outputs))
    }

    /// Returns the extended Poseidon hash with an input rate of 8.
    fn hash_many_psd8(input: &[Field<Self>], num_outputs: u16) -> Vec<Field<Self>> {
        POSEIDON_8.with(|poseidon| poseidon.hash_many(input, num_outputs))
    }

    /// Returns the Poseidon hash with an input rate of 2 on the scalar field.
    fn hash_to_scalar_psd2(input: &[Field<Self>]) -> Scalar<Self> {
        POSEIDON_2.with(|poseidon| poseidon.hash_to_scalar(input))
    }

    /// Returns the Poseidon hash with an input rate of 4 on the scalar field.
    fn hash_to_scalar_psd4(input: &[Field<Self>]) -> Scalar<Self> {
        POSEIDON_4.with(|poseidon| poseidon.hash_to_scalar(input))
    }

    /// Returns the Poseidon hash with an input rate of 8 on the scalar field.
    fn hash_to_scalar_psd8(input: &[Field<Self>]) -> Scalar<Self> {
        POSEIDON_8.with(|poseidon| poseidon.hash_to_scalar(input))
    }

    /// Returns the Poseidon PRF with an input rate of 2.
    fn prf_psd2(seed: &Field<Self>, input: &[Field<Self>]) -> Field<Self> {
        POSEIDON_2.with(|poseidon| poseidon.prf(seed, input))
    }

    /// Returns the Poseidon PRF with an input rate of 4.
    fn prf_psd4(seed: &Field<Self>, input: &[Field<Self>]) -> Field<Self> {
        POSEIDON_4.with(|poseidon| poseidon.prf(seed, input))
    }

    /// Returns the Poseidon PRF with an input rate of 8.
    fn prf_psd8(seed: &Field<Self>, input: &[Field<Self>]) -> Field<Self> {
        POSEIDON_8.with(|poseidon| poseidon.prf(seed, input))
    }
}

impl Environment for AleoV0 {
    type Affine = <E as Environment>::Affine;
    type AffineParameters = <E as Environment>::AffineParameters;
    type BaseField = <E as Environment>::BaseField;
    type ScalarField = <E as Environment>::ScalarField;

    /// The maximum number of characters allowed in a string.
    const NUM_STRING_BYTES: u32 = E::NUM_STRING_BYTES;

    /// Returns the `zero` constant.
    fn zero() -> LinearCombination<Self::BaseField> {
        E::zero()
    }

    /// Returns the `one` constant.
    fn one() -> LinearCombination<Self::BaseField> {
        E::one()
    }

    /// Returns a new variable of the given mode and value.
    fn new_variable(mode: Mode, value: Self::BaseField) -> Variable<Self::BaseField> {
        E::new_variable(mode, value)
    }

    /// Returns a new witness of the given mode and value.
    fn new_witness<Fn: FnOnce() -> Output::Primitive, Output: Inject>(mode: Mode, logic: Fn) -> Output {
        E::new_witness(mode, logic)
    }

    /// Enters a new scope for the environment.
    fn scope<S: Into<String>, Fn, Output>(name: S, logic: Fn) -> Output
    where
        Fn: FnOnce() -> Output,
    {
        E::scope(name, logic)
    }

    /// Adds one constraint enforcing that `(A * B) == C`.
    fn enforce<Fn, A, B, C>(constraint: Fn)
    where
        Fn: FnOnce() -> (A, B, C),
        A: Into<LinearCombination<Self::BaseField>>,
        B: Into<LinearCombination<Self::BaseField>>,
        C: Into<LinearCombination<Self::BaseField>>,
    {
        E::enforce(constraint)
    }

    /// Returns `true` if all constraints in the environment are satisfied.
    fn is_satisfied() -> bool {
        E::is_satisfied()
    }

    /// Returns `true` if all constraints in the current scope are satisfied.
    fn is_satisfied_in_scope() -> bool {
        E::is_satisfied_in_scope()
    }

    /// Returns the number of constants in the entire circuit.
    fn num_constants() -> u64 {
        E::num_constants()
    }

    /// Returns the number of public variables in the entire circuit.
    fn num_public() -> u64 {
        E::num_public()
    }

    /// Returns the number of private variables in the entire circuit.
    fn num_private() -> u64 {
        E::num_private()
    }

    /// Returns the number of constraints in the entire circuit.
    fn num_constraints() -> u64 {
        E::num_constraints()
    }

    /// Returns the number of gates in the entire circuit.
    fn num_gates() -> u64 {
        E::num_gates()
    }

    /// Returns the number of constants for the current scope.
    fn num_constants_in_scope() -> u64 {
        E::num_constants_in_scope()
    }

    /// Returns the number of public variables for the current scope.
    fn num_public_in_scope() -> u64 {
        E::num_public_in_scope()
    }

    /// Returns the number of private variables for the current scope.
    fn num_private_in_scope() -> u64 {
        E::num_private_in_scope()
    }

    /// Returns the number of constraints for the current scope.
    fn num_constraints_in_scope() -> u64 {
        E::num_constraints_in_scope()
    }

    /// Returns the number of gates for the current scope.
    fn num_gates_in_scope() -> u64 {
        E::num_gates_in_scope()
    }

    /// A helper method to recover the y-coordinate given the x-coordinate for
    /// a twisted Edwards point, returning the affine curve point.
    fn affine_from_x_coordinate(x: Self::BaseField) -> Self::Affine {
        E::affine_from_x_coordinate(x)
    }

    /// Halts the program from further synthesis, evaluation, and execution in the current environment.
    fn halt<S: Into<String>, T>(message: S) -> T {
        E::halt(message)
    }

    /// Clears the circuit and initializes an empty environment.
    fn reset() {
        E::reset()
    }
}

impl fmt::Display for AleoV0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO (howardwu): Find a better way to print the circuit.
        fmt::Display::fmt(&Circuit, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_circuit_types::Field;

    /// Compute 2^EXPONENT - 1, in a purposefully constraint-inefficient manner for testing.
    fn create_example_circuit<E: Environment>() -> Field<E> {
        let one = <E as Environment>::BaseField::one();
        let two = one + one;

        const EXPONENT: u64 = 64;

        // Compute 2^EXPONENT - 1, in a purposefully constraint-inefficient manner for testing.
        let mut candidate = Field::<E>::new(Mode::Public, one);
        let mut accumulator = Field::new(Mode::Private, two);
        for _ in 0..EXPONENT {
            candidate += &accumulator;
            accumulator *= Field::new(Mode::Private, two);
        }

        assert_eq!((accumulator - Field::one()).eject_value(), candidate.eject_value());
        assert_eq!(2, E::num_public());
        assert_eq!(2 * EXPONENT + 1, E::num_private());
        assert_eq!(EXPONENT, E::num_constraints());
        assert!(E::is_satisfied());

        candidate
    }

    #[test]
    fn test_print_circuit() {
        let _candidate = create_example_circuit::<AleoV0>();
        let output = format!("{}", AleoV0);
        println!("{}", output);
    }

    #[test]
    fn test_circuit_scope() {
        AleoV0::scope("test_circuit_scope", || {
            assert_eq!(0, AleoV0::num_constants());
            assert_eq!(1, AleoV0::num_public());
            assert_eq!(0, AleoV0::num_private());
            assert_eq!(0, AleoV0::num_constraints());

            assert_eq!(0, AleoV0::num_constants_in_scope());
            assert_eq!(0, AleoV0::num_public_in_scope());
            assert_eq!(0, AleoV0::num_private_in_scope());
            assert_eq!(0, AleoV0::num_constraints_in_scope());
        })
    }
}
