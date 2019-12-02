mod program;

use common::input;
use program::Program;

fn main() {
    let mut program: Program = input::read_line().into();

    println!("[PART ONE] value at position 0: {}", program.run(12, 2));

    'outer: for noun in 0..=100 {
        for verb in 0..=100 {
            program.reset();
            if program.run(noun, verb) == 19_690_720 {
                println!("[PART TWO] 100 * noun + verb = {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
