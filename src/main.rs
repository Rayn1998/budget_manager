mod budget;
mod budgets;
mod command;

// Need to understand how to handle the creating and manipulating 
// the new budget. Find out how to deal with mongodb in Rust
// Create the schemas and define the collections with the budget data

use std::io::stdin;
use std::process;
use crate::budget::{ Budget, EditInput };
use crate::budgets::Budgets;
use crate::command::*;

const COMMANDS: [&str; 11] = [
    "new budget",
    "show budgets",
    "get budget",
    "show ballance",
    "add",
    "remove",
    "edit transaction",
    "show transactions",
    "delete budget",
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

        if input.trim().is_empty() {
            println!("Type something");
            continue;
        }
        
        handle_input_command(input, &mut budgets, &mut current_budget);
    }
}


fn handle_input_command(input: &str, budgets: &mut Budgets, current_budget: &mut Option<usize>) -> () {
    match Command::input_match_command(input) {
        Command::NewBudget => create_new_budget(budgets),
        Command::ShowBudgets => show_budgets(budgets),
        Command::ShowBallance => show_ballance(budgets, current_budget),
        Command::GetBudget => get_budget(budgets, current_budget),
        Command::Add => add_to_budget(budgets, current_budget),
        Command::Remove => remove_from_budget(budgets, current_budget),
        Command::ShotTransactions => show_transactions(budgets, current_budget),
        Command::EditTransaction => edit_transaction(budgets, current_budget),
        Command::DeleteBudget => delete_budget(budgets, current_budget),
        Command::Help => print_help(),
        Command::Exit => exit(),
        Command::Invalid => invalid_input(),
    }
}


fn create_new_budget(budgets: &mut Budgets) {
    let mut name = String::new();
    let mut amount_input = String::new();
    let amount: i32;

    loop {
        name.clear();
        println!("Enter the name of the budget, please");
        stdin().read_line(&mut name).expect("Error with entering the name");
    
        if name.trim().is_empty() {
            println!("You need to specify the budget name");
            continue;
        } else {
            break;
        }
    }

    loop {
        amount_input.clear();
        println!("Enter the amount of available money for budget");
        stdin().read_line(&mut amount_input).expect("Error with entering the amount");
        
        match amount_input.trim().parse::<i32>() {
            Ok(value) => {
                amount = value;
                break;
            },
            Err(_) => {
                println!("Enter the amount by numbers, please...");
                continue;
            }
        };
    }

    let budget = Budget::new(name.trim().to_string(), amount);
    budgets.add_budget(budget);
    println!("The budget is created: {}", name.trim());
}


fn show_budgets(budgets: &Budgets) -> () {
    check_budgets(budgets);

    for (i, budget) in budgets.budgets.iter().enumerate() {
        println!("{}. - {}", i + 1, budget.name);
    }
}


fn get_budget(budgets: &Budgets, current_budget: &mut Option<usize>) -> () {
    check_budgets(budgets);

    println!("Choose the budget by entering the number");
    show_budgets(budgets);
    
    loop {

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error with input");
    
        let index = match input.trim().parse::<usize>() {
            Ok(value) if value > 0 => value - 1,
            Ok(_) => {
                println!("Please, enter the index greater than 0");
                continue;
            },
            Err(_) => {
                println!("Enter the number, please");
                continue;
            },
        };
        if index < budgets.budgets.len() {
            *current_budget = Some(index);
            println!("Selected budget: {}", budgets.budgets[index].name);
            break;
        } else {
            println!("Invalid budget index");
            continue;
        }
    }
}


fn show_ballance(budgets: &Budgets, current_budget: &mut Option<usize>) -> () {
    check_current_budget(current_budget);
    
    let ballance = budgets.budgets.get(current_budget.unwrap()).unwrap().get_budget();
    println!("The current ballance is: {}", ballance);
}


fn add_to_budget(budgets: &mut Budgets, current_budget: &mut Option<usize>) -> () {
    check_current_budget(current_budget);
    check_budgets(budgets);

    let amount: i32;
    loop {
        let mut amount_input = String::new();
        stdin().read_line(&mut amount_input).expect("Error reading the amoung");

        match amount_input.trim().parse::<i32>() {
            Ok(value) => {
                amount = value;
                break;
            },
            Err(_) => {
                println!("Enter the number, please");
                continue;
            }
        };
    }

    budgets.budgets
        .get_mut(current_budget.unwrap())
        .unwrap()
        .add(amount);
}


fn remove_from_budget(budgets: &mut Budgets, current_budget: &mut Option<usize>) -> () {
    check_current_budget(current_budget);
    check_budgets(budgets);

    let amount: i32;

    loop {
        let mut amount_input = String::new();
        stdin().read_line(&mut amount_input).expect("Error reading the amount");
        match amount_input.trim().parse::<i32>() {
            Ok(value) => {
                amount = value;
                break;
            },
            Err(_) => {
                println!("Error, parsing the value");
                continue;
            }
        }
    }

    budgets.budgets
        .get_mut(current_budget.unwrap())
        .unwrap()
        .remove(amount);
}


fn show_transactions(budgets: &mut Budgets, current_budget: &mut Option<usize>) -> () {
    check_current_budget(current_budget);
    check_budgets(budgets);

    budgets.budgets
        .get(current_budget.unwrap())
        .unwrap()
        .show_transactions();
}


fn edit_transaction(budgets: &mut Budgets, current_budget: &mut Option<usize>) -> () {
    check_current_budget(current_budget);
    check_budgets(budgets);

    println!("Enter the number of transaction");
        let mut index = String::new();
        stdin().read_line(&mut index).expect("Error reading the index");
        let index = index.trim().parse::<i32>().expect("Error parsing the index");

        println!("Enter the number for edit the transaction or type \"delete\" to delete it");
        let mut method = String::new();
        stdin().read_line(&mut method).expect("Error reading the method");
        match method.trim().parse::<i32>() {
            Ok(value) => {
                budgets.budgets
                    .get_mut(current_budget.unwrap())
                    .unwrap()
                    .edit(index, EditInput::Amount(value));
            }, 
            Err(_) => {
                if method.trim() == "delete" {
                    budgets.budgets
                        .get_mut(current_budget.unwrap())
                        .unwrap()
                        .edit(index, EditInput::Delete);
                } else {
                    println!("Unsupported method");
                }
            }
        }
}


fn delete_budget(budgets: &mut Budgets, current_budget: &mut Option<usize>) {
    check_current_budget(current_budget);
    check_budgets(budgets);
    
    budgets.budgets.remove(current_budget.unwrap());
}


fn check_current_budget(current_budget: &mut Option<usize>) -> () {
    if *current_budget == None {
        println!("Choose the budget to operate first, please\n type \"get budget\"");
        return;
    }
}


fn check_budgets(budgets: &Budgets) -> () {
    if budgets.budgets.len() == 0 {
        println!("You haven't created any budget yet. Please type \"new budget\"");
        return;
    }
}


fn invalid_input() -> () {
    println!("It's unexistant command");
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