fn interpret(cmd: &str) -> Result<i32, String> {
    let tokens: Vec<&str> = cmd.split_whitespace().collect();

    if tokens.len() != 3 {
        return Err("Too few Arguments!".to_string());
    }

    let command = tokens[0];
    let num1 = tokens[1].parse::<i32>()
        .map_err(|_| "Parse error".to_string())?;
    let num2 = tokens[2].parse::<i32>()
        .map_err(|_| "Parse error".to_string())?;

    match command {
        "add" => Ok(num1 + num2),
        "sub" => Ok(num1 - num2),
        "mul" => Ok(num1 * num2),
        "div" => {
            if num2 == 0 {
                Err("Cannot divide by 0!".to_string())
            } else {
                Ok(num1 / num2)
            }
        },
        _ => Err("Unknown command!".to_string())
    }
}

fn main() {
    let test_cases = [
        "add 1 2",
        "sub 10 5",
        "mul 3 4",
        "div 10 2",
        "div 10 0",
        "mod 10 3",
        "add 1",
        "add one 2",
        "add 1 two",
        "",
    ];
    
    for cmd in test_cases {
        match interpret(cmd) {
            Ok(result) => println!("'{}' => {}", cmd, result),
            Err(error) => println!("'{}' => Error: {}", cmd, error),
        }
    }
}