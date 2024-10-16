mod budget;
mod budgets;
mod command;

// Need to understand how to handle the creating and manipulating 
// the new budget. Find out how to deal with mongodb in Rust
// Create the schemas and define the collections with the budget data

use std::io::stdin;
use std::process;
use crate::budget::Budget;
use crate::budgets::Budgets;
use crate::command::*;

const COMMANDS: [&str; 5] = [
    "new budget",
    "show budgets",
    "get budget",
    "help",
    "exit"
];

fn main() {
    let mut budgets = Budgets::new();
    let mut current_budget: Option<usize> = None;
    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Something wrong with input itself");
        let input = input.trim();

        if input.is_empty() {
            println!("Type something");
            continue;
        }
        
        handle_input_command(input, &mut budgets, &mut current_budget);
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



fn handle_input_command(input: &str, budgets: &mut Budgets, current_budget: &mut Option<usize>) -> () {
    match Command::input_match_command(input) {
        Command::NewBudget => create_new_budget(budgets),
        Command::ShowBudgets => {
            for budget in budgets.budgets.iter() {
                println!("{}", budget.name);
            }
        },
        Command::ShowBallance => show_ballance(budgets, current_budget),
        Command::GetBudget => {
            match get_budget(budgets) {
                Some(index) => {
                    *current_budget = Some(index);
                    println!("Selected budget: {}", budgets.budgets[index].name);
                },
                None => println!("Invalid budget index"),
            }
        },
        Command::Add => {
            // let amount = input.parse::<i32>().expect("Error, parsing the value");
            // Budget::add(&mut self, amount)
        },
        Command::Remove => {

        },
        Command::Edit => {},
        Command::ShotTransactions => {},
        Command::DeleteBudget => {},
        Command::Help => print_help(),
        Command::Exit => exit(),
        Command::Invalid => {
            println!("It's unexistant command");
        }
    }
}

fn create_new_budget(budgets: &mut Budgets) {
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
    budgets.add_budget(budget);
    println!("The budget is created: {}", name);
}

fn get_budget(budgets: &Budgets) -> Option<usize> {
    println!("Choose the budget by entering the number");
    
    let mut input_index = String::new();

    stdin().read_line(&mut input_index).expect("Error with input");

    let index = match input_index.trim().parse::<usize>() {
        Ok(index) => index - 1,
        Err(_) => return None,
    };

    if index < budgets.budgets.len() {
        Some(index)
    } else {
        None
    }
}

fn show_ballance(budgets: &Budgets, current_budget: &mut Option<usize>) -> () {
    let budget = budgets.budgets.get(current_budget.unwrap()).unwrap();
    let ballance = budget.value;
    println!("The current ballance is: {}", ballance);
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