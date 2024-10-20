mod budget;
mod budgets;
mod command;
mod app_options;

use std::fs::File;
use std::io::{ stdin, Read, Write };
use std::process;
use crate::budget::{ Budget, EditInput };
use crate::budgets::Budgets;
use crate::command::*;
use crate::app_options::AppOptions;


const COMMANDS: [&str; 11] = [
    "new budget",
    "show budgets",
    "set budget <index>",
    "show ballance",
    "add <amount>",
    "remove <amount>",
    "edit transaction",
    "show transactions",
    "delete budget <index>",
    "help",
    "exit"
];

fn main() {

    let mut budgets = Budgets::new();
    let mut current_budget: Option<usize> = None;

    // Read the initial data or create the new YAML file
    let mut yaml: File = match File::open("budget.yaml") {
        Ok(file) => file,
        Err(_) => {
            println!("Can't find the budget file, creating the new one...");
            File::create_new("budget.yaml").expect("Can't create a file")
        },
    };
    
    let mut file_data = String::new();
    yaml.read_to_string(&mut file_data).unwrap();
    match serde_yaml::from_str(&file_data) {
        Ok(data) => {
            budgets = data;
            println!("budget.yaml successfully read");
            println!("Here are the budgets:");
            show_budgets(&budgets);
        },
        Err(_) => {
            println!("There are no budgets yet");
        }
    }

    let mut options = AppOptions {
        budgets: &mut budgets, 
        current_budget: &mut current_budget,
        something_changed: false,
        been_saved: false,
    };

    // MAIN LOOP
    loop {

        let mut input = String::new();

        stdin().read_line(&mut input).expect("Something wrong with input itself");
        let input = input.trim();

        if input.trim().is_empty() {
            println!("Type something");
            continue;
        }

        handle_input_command(input, &mut options);
    }
}


fn handle_input_command(input: &str, options: &mut AppOptions) -> () {
    let mut command: &str = input;
    let mut appendix: &str = "0";

    // Divides input if it has any number. 
    // Because only add ot remove operations need it
    // And to save the ability to write commands divided withh space
    if input.chars().any(|ch| ch.is_numeric()) {
        let index = input.chars().position(|ch| ch.is_numeric()).unwrap();
        appendix = &input[index..];
        command = input[0..index].trim();
    }

    match Command::input_match_command(command) {
        Command::NewBudget => create_new_budget(options),
        Command::ShowBudgets => show_budgets(options.budgets),
        Command::ShowBallance => show_ballance(options),
        Command::SetBudget => set_budget(options, appendix),
        Command::Add => add_to_budget(options, appendix),
        Command::Remove => remove_from_budget(options, appendix),
        Command::ShotTransactions => show_transactions(options.budgets, options.current_budget),
        Command::EditTransaction => edit_transaction(options.budgets, options.current_budget),
        Command::DeleteBudget => delete_budget(options, appendix),
        Command::Help => print_help(),
        Command::Save => save(options),
        Command::Exit => exit(options),
        Command::Invalid => invalid_input(),
    }
}


fn create_new_budget(options: &mut AppOptions) {
    let budgets = &mut options.budgets;
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
    options.changes_true();
    options.been_saved_false();
    println!("The budget is created: {}", name.trim());
}


fn show_budgets(budgets: &Budgets) -> () {
    if check_budgets(budgets) {
        for (i, budget) in budgets.budgets.iter().enumerate() {
            println!("{}. - {}", i + 1, budget.name);
        }
    }
}


fn set_budget(options: &mut AppOptions, input_index: &str) -> () {
    if check_budgets(options.budgets) {
        // println!("Choose the budget by entering the number");
        // show_budgets(options.budgets);
        
        // loop {
    
            // let mut input = String::new();
            // stdin().read_line(&mut input).expect("Error with input");
        
            let index = match input_index.trim().parse::<usize>() {
                Ok(value) if value > 0 => value - 1,
                Ok(_) => {
                    println!("Please, enter the index greater than 0");
                    return;
                    // continue;
                },
                Err(_) => {
                    println!("Enter the number, please");
                    return;
                    // continue;
                },
            };

            if index < options.budgets.budgets.len() {
                *options.current_budget = Some(index);
                println!("Selected budget: {}", options.budgets.budgets[index].name);
                // break;
            } else {
                println!("Invalid budget index");
                // continue;
            }
        // }
    }
}


fn show_ballance(options: &mut AppOptions) -> () {
    if check_current_budget(options.current_budget) {
        let ballance = options.budgets.budgets.get(options.current_budget.unwrap()).unwrap().get_ballance();
        println!("The current ballance is: {}", ballance);
    }
}


fn add_to_budget(options: &mut AppOptions, amount_input: &str) -> () {
    if check_current_budget(options.current_budget) && check_budgets(options.budgets) {
        let amount: i32;

        loop {
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
    
        options.budgets.budgets
            .get_mut(options.current_budget.unwrap())
            .unwrap()
            .add(amount);

        options.changes_true();
        options.been_saved_false();
    }

}


fn remove_from_budget(options: &mut AppOptions, amount_input: &str) -> () {
    if check_current_budget(options.current_budget) && check_budgets(options.budgets) {
        let amount: i32;

        loop {
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
    
        options.budgets.budgets
            .get_mut(options.current_budget.unwrap())
            .unwrap()
            .remove(amount);

        options.changes_true();
        options.been_saved_false();
    }

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
    if check_current_budget(current_budget) && check_budgets(budgets) {
        println!("Enter the number of transaction");
        show_transactions(budgets, current_budget);
    
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
}


fn delete_budget(options: &mut AppOptions, input_index: &str) -> () {
    if check_budgets(options.budgets) {
        match input_index.parse::<usize>() {
            Ok(index) => {
                options.budgets.budgets.remove(index - 1);
                println!("Budget has been successfully deleted");
                options.changes_true();
            },
            Err(_) => {
                println!("Enter the number for index, please");
            }
        }
    }
}


fn save(options: &mut AppOptions) -> () {
    let serialized: String = serde_yaml::to_string(options.budgets).unwrap();
    let serialized = serialized.as_bytes();
    let mut file = File::create("budget.yaml").unwrap();
    file.write(serialized).unwrap();
    options.been_saved_true();
    options.changes_false();
    println!("Budgets successfully saved");
}


fn check_current_budget(current_budget: &mut Option<usize>) -> bool {
    if *current_budget == None {
        println!("Choose the budget to operate first, please\n type \"set budget\"");
        false 
    } else {
        true
    }
}


fn check_budgets(budgets: &Budgets) -> bool {
    if budgets.budgets.len() == 0 {
        println!("You haven't created any budget yet. Please type \"new budget\"");
        false
    } else {
        true
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


fn exit(options: &mut AppOptions) -> ! {
    if !options.something_changed {
        process::exit(1);
    } else {
        println!("You changed something and didn't save it");
        println!("Do you want to save it now? \"y\" or \"n\"");
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Error in input");
            let input = input.trim();
            match input {
                "y" => {
                    save(options);
                    exit(options);
                },
                "n" => {
                    process::exit(1);
                },
                _ => {
                    println!("Type \"y\" or \"n\", please");
                    continue;
                }
            }
        }
    }
}