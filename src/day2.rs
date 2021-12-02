enum SubmarineCommand {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_pairs(input: Vec<(String, u32)>) -> Result<Vec<SubmarineCommand>, String> {
    let mut vec: Vec<SubmarineCommand> = Vec::new();

    for (command, count) in input {
        match command.as_ref() {
            "forward" => vec.push(SubmarineCommand::Forward(count)),
            "down" => vec.push(SubmarineCommand::Down(count)),
            "up" => vec.push(SubmarineCommand::Up(count)),
            x => return Err(format!("'{}' is not a valid command", x)),
        }
    }

    return Ok(vec);
}

fn parse_input(input: Vec<String>) -> Result<Vec<(String, u32)>, String> {
    let mut output: Vec<(String, u32)> = Vec::new();
    for line in input {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 2 {
            return Err(format!("Too many items on line '{}'", line));
        }
        let command = parts[0];
        let count = parts[1].parse::<u32>().unwrap();
        output.push((String::from(command), count));
    }
    return Ok(output);
}

pub fn p1(input: Vec<String>) -> u32 {
    let pairs = parse_input(input);
    let commands = parse_pairs(pairs.unwrap());

    let mut pos: (u32, u32) = (0, 0);
    for command in commands.unwrap() {
        match command {
            SubmarineCommand::Forward(x) => { pos = (pos.0 + x, pos.1); },
            SubmarineCommand::Down(x) => { pos = (pos.0, pos.1 + x); },
            SubmarineCommand::Up(x) => { pos = (pos.0, pos.1 - x); },
        }
    }
    return pos.0 * pos.1;
}

pub fn p2(input: Vec<String>) -> u32 {
    let pairs = parse_input(input);
    let commands = parse_pairs(pairs.unwrap());

    let mut pos: (u32, u32) = (0, 0);
    let mut aim: u32 = 0;
    for command in commands.unwrap() {
        match command {
            SubmarineCommand::Forward(x) => { pos = (pos.0 + x, pos.1 + aim * x); },
            SubmarineCommand::Down(x) => { aim += x; },
            SubmarineCommand::Up(x) => { aim -= x; },
        }
    }
    return pos.0 * pos.1;
}