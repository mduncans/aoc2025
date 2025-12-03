use crate::instructions::{Direction, Instruction};

#[derive(Debug, PartialEq, Eq)]
struct Dial {
    position: i32,
    modulus: i32,
}

impl Dial {
    fn new(position: i32, modulus: i32) -> Self {
        Dial { position, modulus }
    }

    fn apply(&mut self, direction: Direction, value: i32) {
        self.position = match direction {
            Direction::Left => (self.position - value).rem_euclid(self.modulus),
            Direction::Right => (self.position + value).rem_euclid(self.modulus),
        };
    }

    fn ticks_to_zero(&self, direction: Direction) -> i32 {
        let ticks = match direction {
            Direction::Left => self.position,
            Direction::Right => (self.modulus - self.position).rem_euclid(self.modulus),
        };

        if ticks == 0 { self.modulus } else { ticks }
    }

    fn num_zero_crossings(&self, direction: Direction, value: i32) -> i32 {
        let ticks_to_zero = self.ticks_to_zero(direction);
        if value >= ticks_to_zero {
            1 + (value - ticks_to_zero) / self.modulus
        } else {
            0
        }
    }
}

pub struct ZeroCounter {
    dial: Dial,
    final_zero_count: i32,
    any_zero_count: i32,
}

impl ZeroCounter {
    pub fn new(position: i32, modulus: i32) -> Self {
        let dial = Dial::new(position, modulus);

        Self {
            dial,
            final_zero_count: 0,
            any_zero_count: 0,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        self.any_zero_count += self
            .dial
            .num_zero_crossings(instruction.direction, instruction.value);
        self.dial.apply(instruction.direction, instruction.value);
        if self.dial.position == 0 {
            self.final_zero_count += 1;
        }
    }

    pub fn final_zero_count(&self) -> i32 {
        self.final_zero_count
    }

    pub fn any_zero_count(&self) -> i32 {
        self.any_zero_count
    }
}

#[cfg(test)]
mod test {
    use crate::dial::{Dial, ZeroCounter};
    use crate::instructions::Instruction;
    use std::str::FromStr;

    #[test]
    fn single_instructions_work() {
        // Starting number, instruction, ending number
        let scenarios = vec![
            (11, "R8", 19),
            (19, "L19", 0),
            (0, "L1", 99),
            (99, "R1", 0),
            (5, "L10", 95),
            (95, "R5", 0),
        ];

        for scenario in scenarios {
            let (start, inst_str, end) = scenario;
            let mut dial = Dial::new(start, 100);
            let instruction = Instruction::from_str(inst_str).unwrap();

            dial.apply(instruction.direction, instruction.value);

            assert_eq!(dial.position, end);
        }
    }

    #[test]
    fn can_count_final_zeros() {
        let instructions = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        let mut counter = ZeroCounter::new(50, 100);

        for inst_str in instructions {
            let int = Instruction::from_str(inst_str).unwrap();
            counter.execute(int);
        }

        assert_eq!(counter.final_zero_count(), 3);
    }

    #[test]
    fn can_count_any_zero() {
        let instructions = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        let mut counter = ZeroCounter::new(50, 100);

        for inst_str in instructions {
            let int = Instruction::from_str(inst_str).unwrap();
            counter.execute(int);
        }

        assert_eq!(counter.any_zero_count(), 6);
    }
}
