fn help() {
    println!("Usage:
        args <string> - String is the answer
        args <increase/decrease> <integer>
    ");
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    match args.len() {

        1 => {
            println!("No params passed!");
            help();
        },

        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer"),
                _ => println!("This is not the answer")
            }
        },

        3 => {
            let cmd = &args[1];
            let number: i32 = match args[2].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("The arg: '{}' does not seem to be a number", args[2]);
                    return;
                }
            };

            match &cmd[..] {
                "increase" => println!("number: {}", number + 1),
                "decrease" => println!("number: {}",number - 1),
                _ => {
                    println!("invalid command");
                    help();
                }
            };

        }

        _ => {
            help();
        }
    };
}