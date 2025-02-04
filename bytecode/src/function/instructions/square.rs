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
use snarkvm_circuit::{Literal, Parser, ParserResult, Square as SquareCircuit};
use snarkvm_utilities::{FromBytes, ToBytes};

use core::fmt;
use nom::combinator::map;
use std::io::{Read, Result as IoResult, Write};

/// Squares `first`, storing the outcome in `destination`.
pub struct Square<P: Program> {
    operation: UnaryOperation<P>,
}

impl<P: Program> Square<P> {
    /// Returns the operands of the instruction.
    pub fn operands(&self) -> Vec<Operand<P>> {
        self.operation.operands()
    }

    /// Returns the destination register of the instruction.
    pub fn destination(&self) -> &Register<P> {
        self.operation.destination()
    }
}

impl<P: Program> Opcode for Square<P> {
    /// Returns the opcode as a string.
    #[inline]
    fn opcode() -> &'static str {
        "square"
    }
}

impl<P: Program> Operation<P> for Square<P> {
    /// Evaluates the operation.
    #[inline]
    fn evaluate(&self, registers: &Registers<P>) {
        // Load the values for the first operand.
        let first = match registers.load(self.operation.first()) {
            Value::Literal(literal) => literal,
            Value::Definition(name, ..) => P::halt(format!("{name} is not a literal")),
        };

        // Perform the operation.
        let result = match first {
            Literal::Field(a) => Literal::Field(a.square()),
            _ => P::halt(format!("Invalid '{}' instruction", Self::opcode())),
        };

        registers.assign(self.operation.destination(), result);
    }
}

impl<P: Program> Parser for Square<P> {
    type Environment = P::Environment;

    /// Parses a string into a 'square' operation.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Parse the operation from the string.
        map(UnaryOperation::parse, |operation| Self { operation })(string)
    }
}

impl<P: Program> fmt::Display for Square<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.operation)
    }
}

impl<P: Program> FromBytes for Square<P> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        Ok(Self { operation: UnaryOperation::read_le(&mut reader)? })
    }
}

impl<P: Program> ToBytes for Square<P> {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.operation.write_le(&mut writer)
    }
}

#[allow(clippy::from_over_into)]
impl<P: Program> Into<Instruction<P>> for Square<P> {
    /// Converts the operation into an instruction.
    fn into(self) -> Instruction<P> {
        Instruction::Square(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_instruction_halts, test_modes, Identifier, Process};

    #[test]
    fn test_parse() {
        let (_, instruction) = Instruction::<Process>::parse("square r0 into r1;").unwrap();
        assert!(matches!(instruction, Instruction::Square(_)));
    }

    test_modes!(field, Square, "2field", "4field");

    test_instruction_halts!(i8_square_halts, Square, "Invalid 'square' instruction", "1i8.constant");
    test_instruction_halts!(i16_square_halts, Square, "Invalid 'square' instruction", "1i16.constant");
    test_instruction_halts!(i32_square_halts, Square, "Invalid 'square' instruction", "1i32.constant");
    test_instruction_halts!(i64_square_halts, Square, "Invalid 'square' instruction", "1i64.constant");
    test_instruction_halts!(i128_square_halts, Square, "Invalid 'square' instruction", "1i128.constant");
    test_instruction_halts!(u8_square_halts, Square, "Invalid 'square' instruction", "1u8.constant");
    test_instruction_halts!(u16_square_halts, Square, "Invalid 'square' instruction", "1u16.constant");
    test_instruction_halts!(u32_square_halts, Square, "Invalid 'square' instruction", "1u32.constant");
    test_instruction_halts!(u64_square_halts, Square, "Invalid 'square' instruction", "1u64.constant");
    test_instruction_halts!(u128_square_halts, Square, "Invalid 'square' instruction", "1u128.constant");
    test_instruction_halts!(group_square_halts, Square, "Invalid 'square' instruction", "2group.constant");
    test_instruction_halts!(scalar_square_halts, Square, "Invalid 'square' instruction", "1scalar.constant");
    test_instruction_halts!(
        address_square_halts,
        Square,
        "Invalid 'square' instruction",
        "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.constant"
    );
    test_instruction_halts!(boolean_square_halts, Square, "Invalid 'square' instruction", "true.constant");
    test_instruction_halts!(string_square_halts, Square, "Invalid 'square' instruction", "\"hello\".constant");

    #[test]
    #[should_panic(expected = "message is not a literal")]
    fn test_definition_halts() {
        let first = Value::<Process>::Definition(Identifier::from_str("message"), vec![
            Value::from_str("2group.public"),
            Value::from_str("10field.private"),
        ]);

        let registers = Registers::<Process>::default();
        registers.define(&Register::from_str("r0"));
        registers.define(&Register::from_str("r1"));
        registers.assign(&Register::from_str("r0"), first);

        Square::from_str("r0 into r1").evaluate(&registers);
    }
}
