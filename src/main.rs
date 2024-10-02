mod budget;

use std::io::stdin;
use std::process;
use crate::budget::Budget;

const COMMANDS: [&str; 4] = [
    "new budget",
    "show budgets",
    "help",
    "exit"
];

fn main() {
    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Something wrong with input itself");
        let input = input.trim();

        if input.is_empty() {
            println!("Type something");
            continue;
        }
        
        handle_input_command(input);
    }
    
    // budget1.add(50);
    // budget1.remove(150);
    // budget1.show_transactions();
    // budget1.edit(1, Some("delete"), None);
    // budget1.edit(1, None, Some(-50));
    // budget1.show_transactions();
    // println!("Left budget is: {}", budget1.get_budget());
    // budget1.delete_self();
}


#[derive(Debug)]
struct Budgets {
    budgets: Vec<Budget>,
}


fn create_new_budget() {
    let mut name = String::new();
    let mut amount = String::new();

    println!("Enter the name of the budget, please");
    stdin().read_line(&mut name).expect("Error with entering the name");
    let name = name.trim();

    if name.is_empty() {
        println!("You need to specify the budget name");
        return; // Change on loop
    }

    println!("Enter the amount of available money for budget");
    stdin().read_line(&mut amount).expect("Error with entering the amount");
    
    let amount = match amount.trim().parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error parsing the amount");
            return; // Change on loop
        }
    };

    let budget = Budget::new(name.to_string(), amount);
    println!("The budget is created: {:?}", budget);
}


fn print_help() -> () {
    println!("Available commands:");
    for command in COMMANDS {
        println!("- {}", command);
    }
}

fn exit() -> ! {
    process::exit(1);
}

enum Command {
    NewBudget,
    Help,
    Exit,
    Invalid,
}

impl Command {
    fn input_match_command(input: &str) -> Command {
        match input {
            "new budget" => Command::NewBudget,
            "help" => Command::Help,
            "exit" => Command::Exit,
            _ => Command::Invalid,
        }
    }
}

fn handle_input_command(input: &str) -> () {
    match Command::input_match_command(input) {
        Command::NewBudget => create_new_budget(),
        Command::Help => print_help(),
        Command::Exit => exit(),
        Command::Invalid => {
            println!("It's unexistant command");
        }
    }
}