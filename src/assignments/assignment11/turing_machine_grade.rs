//! Test cases for assignment11/turing_machine.rs

#[cfg(test)]
mod test_turing_machine {
    use super::super::turing_machine::*;
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn test_invalid_movement() {
        /// Cell value of the tape
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        enum TMValue {
            /// Zero
            Zero,

            /// One
            One,
        }

        /// State for Turing machine
        /// TODO: Modify this so that users can implement their own state
        #[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
        enum TMState {
            /// Halt
            #[default]
            Halt,

            /// A
            A,

            /// B
            B,
        }

        let tape: Vec<RefCell<TMValue>> = vec![
            TMValue::One,
            TMValue::Zero,
            TMValue::One,
            TMValue::One,
            TMValue::One,
            TMValue::Zero,
            TMValue::One,
        ]
        .into_iter()
        .map(|x| RefCell::new(x))
        .collect();

        let instr = HashMap::from([
            (
                (TMState::A, TMValue::Zero),
                (TMState::B, Move::R, TMValue::One),
            ),
            (
                (TMState::A, TMValue::One),
                (TMState::B, Move::L, TMValue::Zero),
            ),
            (
                (TMState::B, TMValue::Zero),
                (TMState::A, Move::L, TMValue::One),
            ),
            (
                (TMState::B, TMValue::One),
                (TMState::A, Move::R, TMValue::Zero),
            ),
        ]);

        let mut tm = TuringMachine::new(instr, tape);
        let mut cursor = Cursor::new(&tm, TMState::A, 0);

        let result = cursor.run(1000);
        assert_eq!(result, Err(TuringMachineError::InvalidMovement));
    }

    #[test]
    fn test_write_15_fail_move() {
        /// Cell value of the tape
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        enum TMValue {
            /// Zero
            Zero,

            /// One
            One,
        }

        /// State for Turing machine
        /// TODO: Modify this so that users can implement their own state
        #[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
        enum TMState {
            /// Halt
            #[default]
            Halt,

            /// A
            A,

            /// B
            B,
        }

        let tape: Vec<RefCell<TMValue>> = vec![TMValue::Zero; 10]
            .into_iter()
            .map(|x| RefCell::new(x))
            .collect();

        let instr = HashMap::from([
            (
                (TMState::A, TMValue::Zero),
                (TMState::B, Move::R, TMValue::One),
            ),
            (
                (TMState::A, TMValue::One),
                (TMState::B, Move::L, TMValue::One),
            ),
            (
                (TMState::B, TMValue::Zero),
                (TMState::A, Move::L, TMValue::One),
            ),
            (
                (TMState::B, TMValue::One),
                (TMState::Halt, Move::R, TMValue::One),
            ),
        ]);

        let mut tm = TuringMachine::new(instr, tape);
        let mut cursor = Cursor::new(&tm, TMState::A, 0);

        let result = cursor.run(1000);
        assert_eq!(result, Err(TuringMachineError::InvalidMovement));
    }

    #[test]
    fn test_write_15_pass() {
        /// Cell value of the tape
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        enum TMValue {
            /// Zero
            Zero,

            /// One
            One,
        }

        /// State for Turing machine
        /// TODO: Modify this so that users can implement their own state
        #[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
        enum TMState {
            /// Halt
            #[default]
            Halt,

            /// A
            A,

            /// B
            B,
        }

        let tape: Vec<RefCell<TMValue>> = vec![TMValue::Zero; 10]
            .into_iter()
            .map(|x| RefCell::new(x))
            .collect();

        let instr = HashMap::from([
            (
                (TMState::A, TMValue::Zero),
                (TMState::B, Move::R, TMValue::One),
            ),
            (
                (TMState::A, TMValue::One),
                (TMState::B, Move::L, TMValue::One),
            ),
            (
                (TMState::B, TMValue::Zero),
                (TMState::A, Move::L, TMValue::One),
            ),
            (
                (TMState::B, TMValue::One),
                (TMState::Halt, Move::R, TMValue::One),
            ),
        ]);

        let mut tm = TuringMachine::new(instr, tape);
        let mut cursor = Cursor::new(&tm, TMState::A, 2);

        let result = cursor.run(1000);
        for (idx, val) in tm.tape.iter().enumerate() {
            if idx < 4 {
                assert_eq!(*val.borrow(), TMValue::One);
            } else {
                assert_eq!(*val.borrow(), TMValue::Zero);
            }
        }
    }

    #[test]
    fn test_write_zero_ones_fail_step() {
        /// Cell value of the tape
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        enum TMValue {
            /// Empty
            Empty,

            /// Zero
            Zero,

            /// One
            One,
        }

        /// State for Turing machine
        /// TODO: Modify this so that users can implement their own state
        #[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
        enum TMState {
            /// Halt
            #[default]
            Halt,

            /// A
            A,

            /// B
            B,
        }

        let tape: Vec<RefCell<TMValue>> = vec![TMValue::Empty; 100]
            .into_iter()
            .map(|x| RefCell::new(x))
            .collect();

        let instr = HashMap::from([
            (
                (TMState::A, TMValue::Empty),
                (TMState::B, Move::R, TMValue::Zero),
            ),
            (
                (TMState::B, TMValue::Empty),
                (TMState::A, Move::R, TMValue::One),
            ),
        ]);

        let mut tm = TuringMachine::new(instr, tape);
        let mut cursor = Cursor::new(&tm, TMState::A, 0);

        let result = cursor.run(10);
        assert_eq!(result, Err(TuringMachineError::ExceedMaxSteps));
    }

    #[test]
    fn test_write_zero_ones() {
        /// Cell value of the tape
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        enum TMValue {
            /// Empty
            Empty,

            /// Zero
            Zero,

            /// One
            One,
        }

        /// State for Turing machine
        /// TODO: Modify this so that users can implement their own state
        #[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
        enum TMState {
            /// Halt
            #[default]
            Halt,

            /// A
            A,

            /// B
            B,
        }

        let tape: Vec<RefCell<TMValue>> = vec![TMValue::Empty; 100]
            .into_iter()
            .map(|x| RefCell::new(x))
            .collect();

        let instr = HashMap::from([
            (
                (TMState::A, TMValue::Empty),
                (TMState::B, Move::R, TMValue::Zero),
            ),
            (
                (TMState::B, TMValue::Empty),
                (TMState::A, Move::R, TMValue::One),
            ),
        ]);

        let mut tm = TuringMachine::new(instr, tape);
        let mut cursor = Cursor::new(&tm, TMState::A, 0);

        let result = cursor.run(1000);
        assert_eq!(result, Err(TuringMachineError::InvalidMovement));
        for (idx, val) in tm.tape.iter().enumerate() {
            if idx % 2 == 0 {
                assert_eq!(*val.borrow(), TMValue::Zero);
            } else {
                assert_eq!(*val.borrow(), TMValue::One);
            }
        }
    }
}
