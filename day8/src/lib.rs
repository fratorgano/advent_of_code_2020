// It's really bad, don't bother reading it.
// First I tried doing with strings and slicing them, got in trouble with the borrow checker and couldn't solve it.
// Rewrote most of the code, now it's a real mess

pub fn input_to_instructions(input:&String) -> Vec<(&str, isize)>{
    let lines:Vec<&str> = input.lines().collect();
    lines
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let operation = parts[0];
            let argument = parts[1].parse().unwrap();

            (operation, argument)
        })
        .collect()
}

pub fn run_program(instructions: &Vec<(&str, isize)>) -> Result<isize, isize>{
    let mut visited:Vec<bool> = vec![false; instructions.len()];
    let mut accumulator:isize = 0;
    let mut pointer:usize = 0;

    while pointer<instructions.len() {
        if visited[pointer]{
            return Err(accumulator);
        }
        visited[pointer] = true;

        match instructions[pointer] {
            ("nop", _) => pointer += 1,
            ("acc", argument) => {
                accumulator += argument;
                pointer += 1;
            }
            ("jmp", argument) => pointer = (pointer as isize + argument) as usize,
            (operation, _) => panic!("Unknown operation: {}", operation),
        }
    }
    Ok(accumulator)
}

fn swap_operations(instruction: &mut (&str, isize)) {
    match instruction {
        ("nop", _) => {
            instruction.0 = "jmp";
        }
        ("jmp", _) => {
            instruction.0 = "nop";
        }
        _ => (),
    }
}

pub fn fix_loop(instructions: &mut Vec<(&str, isize)>) -> isize{

    let mut part2 = 0;
    for i in 0..instructions.len() {
        swap_operations(&mut instructions[i]);

        if let Ok(accumulator) = run_program(&instructions) {
            part2 = accumulator;
            break;
        }
        swap_operations(&mut instructions[i]);
    }
    part2
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day08_test() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let res1 = match run_program(&input_to_instructions(&String::from(input))) {
        Ok(accumulator) => accumulator,
        Err(accumulator) => accumulator,
    };
    assert_eq!(5, res1);
    }
    #[test]
    fn day08_test_2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    assert_eq!(8, fix_loop(&mut input_to_instructions(&String::from(input))));
    }
}
