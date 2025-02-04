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

impl<E: Environment, I: IntegerType> BitOr<Integer<E, I>> for Integer<E, I> {
    type Output = Integer<E, I>;

    /// Returns `(self OR other)`.
    fn bitor(self, other: Integer<E, I>) -> Self::Output {
        self | &other
    }
}

impl<E: Environment, I: IntegerType> BitOr<Integer<E, I>> for &Integer<E, I> {
    type Output = Integer<E, I>;

    /// Returns `(self OR other)`.
    fn bitor(self, other: Integer<E, I>) -> Self::Output {
        self | &other
    }
}

impl<E: Environment, I: IntegerType> BitOr<&Integer<E, I>> for Integer<E, I> {
    type Output = Integer<E, I>;

    /// Returns `(self OR other)`.
    fn bitor(self, other: &Integer<E, I>) -> Self::Output {
        &self | other
    }
}

impl<E: Environment, I: IntegerType> BitOr<&Integer<E, I>> for &Integer<E, I> {
    type Output = Integer<E, I>;

    /// Returns `(self OR other)`.
    fn bitor(self, other: &Integer<E, I>) -> Self::Output {
        let mut output = self.clone();
        output |= other;
        output
    }
}

impl<E: Environment, I: IntegerType> BitOrAssign<Integer<E, I>> for Integer<E, I> {
    /// Sets `self` as `(self OR other)`.
    fn bitor_assign(&mut self, other: Integer<E, I>) {
        *self |= &other;
    }
}

impl<E: Environment, I: IntegerType> BitOrAssign<&Integer<E, I>> for Integer<E, I> {
    /// Sets `self` as `(self OR other)`.
    fn bitor_assign(&mut self, other: &Integer<E, I>) {
        // Stores the bitwise OR of `self` and `other` in `self`.
        *self = Self {
            bits_le: self.bits_le.iter().zip_eq(other.bits_le.iter()).map(|(a, b)| a | b).collect(),
            phantom: Default::default(),
        }
    }
}

impl<E: Environment, I: IntegerType> Metrics<dyn BitOr<Integer<E, I>, Output = Integer<E, I>>> for Integer<E, I> {
    type Case = (Mode, Mode);

    fn count(case: &Self::Case) -> Count {
        match (case.0, case.1) {
            (Mode::Constant, _) | (_, Mode::Constant) => Count::is(0, 0, 0, 0),
            (_, _) => Count::is(0, 0, I::BITS, I::BITS),
        }
    }
}

impl<E: Environment, I: IntegerType> OutputMode<dyn BitOr<Integer<E, I>, Output = Integer<E, I>>> for Integer<E, I> {
    type Case = (CircuitType<Integer<E, I>>, CircuitType<Integer<E, I>>);

    fn output_mode(case: &Self::Case) -> Mode {
        match ((case.0.mode(), &case.0), (case.1.mode(), &case.1)) {
            ((Mode::Constant, _), (Mode::Constant, _)) => Mode::Constant,
            ((Mode::Constant, case), (mode, _)) | ((mode, _), (Mode::Constant, case)) => match case {
                CircuitType::Constant(constant) => {
                    // Determine if the constant is all ones.
                    let is_all_ones = match I::is_signed() {
                        true => constant.eject_value().into_dual() == I::Dual::MAX, // Cast to unsigned
                        false => constant.eject_value() == I::MAX,
                    };
                    match is_all_ones {
                        true => Mode::Constant,
                        false => mode,
                    }
                }
                _ => E::halt(format!("The constant is required to determine the output mode of Constant OR {mode}")),
            },
            (_, _) => Mode::Private,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_circuit_environment::Circuit;
    use snarkvm_utilities::{test_rng, UniformRand};

    use std::ops::RangeInclusive;

    const ITERATIONS: u64 = 128;

    fn check_or<I: IntegerType + BitOr<Output = I>>(name: &str, first: I, second: I, mode_a: Mode, mode_b: Mode) {
        let a = Integer::<Circuit, I>::new(mode_a, first);
        let b = Integer::new(mode_b, second);
        let expected = first | second;
        Circuit::scope(name, || {
            let candidate = (&a).bitor(&b);
            assert_eq!(expected, candidate.eject_value());
            assert_count!(BitOr(Integer<I>, Integer<I>) => Integer<I>, &(mode_a, mode_b));
            assert_output_mode!(BitOr(Integer<I>, Integer<I>) => Integer<I>, &(CircuitType::from(&a), CircuitType::from(&b)), candidate);
        });
        Circuit::reset();
    }

    fn run_test<I: IntegerType + BitOr<Output = I>>(mode_a: Mode, mode_b: Mode) {
        for i in 0..ITERATIONS {
            let first: I = UniformRand::rand(&mut test_rng());
            let second: I = UniformRand::rand(&mut test_rng());

            let name = format!("BitOr: ({} | {}) {}", mode_a, mode_b, i);
            check_or(&name, first, second, mode_a, mode_b);
            check_or(&name, second, first, mode_a, mode_b); // Commute the operation.

            let name = format!("BitOr Identity: ({} | {}) {}", mode_a, mode_b, i);
            check_or(&name, I::zero(), first, mode_a, mode_b);
            check_or(&name, first, I::zero(), mode_a, mode_b); // Commute the operation.

            let name = format!("BitOr Invariant: ({} | {}) {}", mode_a, mode_b, i);
            let invariant = if I::is_signed() { I::zero() - I::one() } else { I::MAX };
            check_or(&name, invariant, first, mode_a, mode_b);
            check_or(&name, first, invariant, mode_a, mode_b); // Commute the operation.
        }

        // Check cases common to signed and unsigned integers.
        check_or("0 | MAX", I::zero(), I::MAX, mode_a, mode_b);
        check_or("MAX | 0", I::MAX, I::zero(), mode_a, mode_b);
        check_or("0 | MIN", I::zero(), I::MIN, mode_a, mode_b);
        check_or("MIN | 0", I::MIN, I::zero(), mode_a, mode_b);
    }

    fn run_exhaustive_test<I: IntegerType + BitOr<Output = I>>(mode_a: Mode, mode_b: Mode)
    where
        RangeInclusive<I>: Iterator<Item = I>,
    {
        for first in I::MIN..=I::MAX {
            for second in I::MIN..=I::MAX {
                let name = format!("BitOr: ({} | {})", first, second);
                check_or(&name, first, second, mode_a, mode_b);
            }
        }
    }

    test_integer_binary!(run_test, i8, bitor);
    test_integer_binary!(run_test, i16, bitor);
    test_integer_binary!(run_test, i32, bitor);
    test_integer_binary!(run_test, i64, bitor);
    test_integer_binary!(run_test, i128, bitor);

    test_integer_binary!(run_test, u8, bitor);
    test_integer_binary!(run_test, u16, bitor);
    test_integer_binary!(run_test, u32, bitor);
    test_integer_binary!(run_test, u64, bitor);
    test_integer_binary!(run_test, u128, bitor);

    test_integer_binary!(#[ignore], run_exhaustive_test, u8, bitor, exhaustive);
    test_integer_binary!(#[ignore], run_exhaustive_test, i8, bitor, exhaustive);
}
