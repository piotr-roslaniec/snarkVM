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

use crate::{
    function::{parsers::*, Instruction, Opcode, Operation, Register, Registers},
    Program,
    Value,
};
use snarkvm_circuit::{Compare, Literal, Parser, ParserResult};
use snarkvm_utilities::{FromBytes, ToBytes};

use core::fmt;
use nom::combinator::map;
use std::io::{Read, Result as IoResult, Write};

/// Checks if `first` is less than `second`, storing the outcome in `destination`.
pub struct LessThan<P: Program> {
    operation: BinaryOperation<P>,
}

impl<P: Program> LessThan<P> {
    /// Returns the operands of the instruction.
    pub fn operands(&self) -> Vec<Operand<P>> {
        self.operation.operands()
    }

    /// Returns the destination register of the instruction.
    pub fn destination(&self) -> &Register<P> {
        self.operation.destination()
    }
}

impl<P: Program> Opcode for LessThan<P> {
    /// Returns the opcode as a string.
    #[inline]
    fn opcode() -> &'static str {
        "lt"
    }
}

impl<P: Program> Operation<P> for LessThan<P> {
    /// Evaluates the operation.
    #[inline]
    fn evaluate(&self, registers: &Registers<P>) {
        // Load the values for the first and second operands.
        let first = match registers.load(self.operation.first()) {
            Value::Literal(literal) => literal,
            Value::Definition(name, ..) => P::halt(format!("{name} is not a literal")),
        };
        let second = match registers.load(self.operation.second()) {
            Value::Literal(literal) => literal,
            Value::Definition(name, ..) => P::halt(format!("{name} is not a literal")),
        };

        // Perform the operation.
        let result = match (first, second) {
            (Literal::Field(a), Literal::Field(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::I8(a), Literal::I8(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::I16(a), Literal::I16(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::I32(a), Literal::I32(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::I64(a), Literal::I64(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::I128(a), Literal::I128(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::Scalar(a), Literal::Scalar(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::U8(a), Literal::U8(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::U16(a), Literal::U16(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::U32(a), Literal::U32(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::U64(a), Literal::U64(b)) => Literal::Boolean(a.is_less_than(&b)),
            (Literal::U128(a), Literal::U128(b)) => Literal::Boolean(a.is_less_than(&b)),
            _ => P::halt(format!("Invalid '{}' instruction", Self::opcode())),
        };

        registers.assign(self.operation.destination(), result);
    }
}

impl<P: Program> Parser for LessThan<P> {
    type Environment = P::Environment;

    /// Parses a string into an 'lt' operation.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Parse the operation from the string.
        let (string, operation) = map(BinaryOperation::parse, |operation| Self { operation })(string)?;
        // Return the operation.
        Ok((string, operation))
    }
}

impl<P: Program> fmt::Display for LessThan<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.operation)
    }
}

impl<P: Program> FromBytes for LessThan<P> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        Ok(Self { operation: BinaryOperation::read_le(&mut reader)? })
    }
}

impl<P: Program> ToBytes for LessThan<P> {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.operation.write_le(&mut writer)
    }
}

#[allow(clippy::from_over_into)]
impl<P: Program> Into<Instruction<P>> for LessThan<P> {
    /// Converts the operation into an instruction.
    fn into(self) -> Instruction<P> {
        Instruction::LessThan(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{binary_instruction_test, test_instruction_halts, test_modes, Identifier, Process};

    #[test]
    fn test_parse() {
        let (_, instruction) = Instruction::<Process>::parse("lt r0 r1 into r2;").unwrap();
        assert!(matches!(instruction, Instruction::LessThan(_)));
    }

    test_modes!(field, LessThan, "1field", "2field", "true");
    binary_instruction_test!(field_gt, LessThan, "2field.public", "1field.public", "false.private");

    test_modes!(i8, LessThan, "-1i8", "1i8", "true");
    binary_instruction_test!(i8_gt, LessThan, "1i8.public", "-1i8.public", "false.private");

    test_modes!(i16, LessThan, "-1i16", "1i16", "true");
    binary_instruction_test!(i16_gt, LessThan, "1i16.public", "-1i16.public", "false.private");

    test_modes!(i32, LessThan, "-1i32", "1i32", "true");
    binary_instruction_test!(i32_gt, LessThan, "1i32.public", "-1i32.public", "false.private");

    test_modes!(i64, LessThan, "-1i64", "1i64", "true");
    binary_instruction_test!(i64_gt, LessThan, "1i64.public", "-1i64.public", "false.private");

    test_modes!(i128, LessThan, "-1i128", "1i128", "true");
    binary_instruction_test!(i128_gt, LessThan, "1i128.public", "-1i128.public", "false.private");

    test_modes!(scalar, LessThan, "1scalar", "2scalar", "true");
    binary_instruction_test!(scalar_gt, LessThan, "2scalar.public", "1scalar.public", "false.private");

    test_modes!(u8, LessThan, "0u8", "1u8", "true");
    binary_instruction_test!(u8_gt, LessThan, "1u8.public", "0u8.public", "false.private");

    test_modes!(u16, LessThan, "0u16", "1u16", "true");
    binary_instruction_test!(u16_gt, LessThan, "1u16.public", "0u16.public", "false.private");

    test_modes!(u32, LessThan, "0u32", "1u32", "true");
    binary_instruction_test!(u32_gt, LessThan, "1u32.public", "0u32.public", "false.private");

    test_modes!(u64, LessThan, "0u64", "1u64", "true");
    binary_instruction_test!(u64_gt, LessThan, "1u64.public", "0u64.public", "false.private");

    test_modes!(u128, LessThan, "0u128", "1u128", "true");
    binary_instruction_test!(u128_gt, LessThan, "1u128.public", "0u128.public", "false.private");

    test_instruction_halts!(
        address_halts,
        LessThan,
        "Invalid 'lt' instruction",
        "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.constant",
        "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.constant"
    );
    test_instruction_halts!(boolean_halts, LessThan, "Invalid 'lt' instruction", "true.constant", "true.constant");
    test_instruction_halts!(group_halts, LessThan, "Invalid 'lt' instruction", "0group", "2group");
    test_instruction_halts!(string_halts, LessThan, "Invalid 'lt' instruction", "\"hello\"", "\"hello\"");

    #[test]
    #[should_panic(expected = "message is not a literal")]
    fn test_definition_halts() {
        let first = Value::<Process>::Definition(Identifier::from_str("message"), vec![
            Value::from_str("2group.public"),
            Value::from_str("10field.private"),
        ]);
        let second = first.clone();

        let registers = Registers::<Process>::default();
        registers.define(&Register::from_str("r0"));
        registers.define(&Register::from_str("r1"));
        registers.define(&Register::from_str("r2"));
        registers.assign(&Register::from_str("r0"), first);
        registers.assign(&Register::from_str("r1"), second);

        LessThan::from_str("r0 r1 into r2").evaluate(&registers);
    }
}
