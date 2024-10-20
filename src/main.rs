mod budget;
mod budgets;
mod command;

use std::fs::File;
use std::io::{ stdin, Read, Write };
use std::process;
use crate::budget::{ Budget, EditInput };
use crate::budgets::Budgets;
use crate::command::*;

// use eframe::{run_simple_native, App};

const COMMANDS: [&str; 11] = [
    "new budget",
    "show budgets",
    "set budget",
    "show ballance",
    "add",
    "remove",
    "edit transaction",
    "show transactions",
    "delete budget",
    "help",
    "exit"
];

struct AppOptions<'app> {
    budgets: &'app mut Budgets,
    current_budget: &'app mut Option<usize>,
    something_changed: bool,
    been_saved: bool,
}

impl<'app> AppOptions<'app> {
    fn changes_true(&mut self) {
        self.something_changed = true;
    }

    fn changes_false(&mut self) {
        self.something_changed = false;
    }

    fn been_saved_true(&mut self) {
        self.been_saved = true;
    }

    fn been_saved_false(&mut self) {
        self.been_saved = false;
    }
}

fn main() {

    let mut been_saved: bool = false;
    let mut something_changed: bool = false;

    let mut budgets = Budgets::new();
    let mut current_budget: Option<usize> = None;

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
        something_changed,
        been_saved,
    };

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
    match Command::input_match_command(input) {
        Command::NewBudget => create_new_budget(options),
        Command::ShowBudgets => show_budgets(options.budgets),
        Command::ShowBallance => show_ballance(options.budgets, options.current_budget),
        Command::GetBudget => set_budget(options.budgets, options.current_budget),
        Command::Add => add_to_budget(options.budgets, options.current_budget),
        Command::Remove => remove_from_budget(options.budgets, options.current_budget),
        Command::ShotTransactions => show_transactions(options.budgets, options.current_budget),
        Command::EditTransaction => edit_transaction(options.budgets, options.current_budget),
        Command::DeleteBudget => delete_budget(options.budgets, options.current_budget),
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
    // let budgets
    if check_budgets(budgets) {
        for (i, budget) in budgets.budgets.iter().enumerate() {
            println!("{}. - {}", i + 1, budget.name);
        }
    }
}


fn set_budget(budgets: &Budgets, current_budget: &mut Option<usize>) -> () {
    if check_budgets(budgets) {
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
}


fn show_ballance(budgets: &Budgets, current_budget: &mut Option<usize>) -> () {
    if check_current_budget(current_budget) {
        let ballance = budgets.budgets.get(current_budget.unwrap()).unwrap().get_ballance();
        println!("The current ballance is: {}", ballance);
    }
}


fn add_to_budget(budgets: &mut Budgets, current_budget: &mut Option<usize>) -> () {
    if check_current_budget(current_budget) && check_budgets(budgets) {
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
    println!("Budget has been successfully deleted");
}


fn save(options: &mut AppOptions) -> () {
    let budgets = &options.budgets;
    let serialized: String = serde_yaml::to_string(budgets).unwrap();
    let serialized = serialized.as_bytes();
    let mut file = File::create("budget.yaml").unwrap();
    file.write(serialized).unwrap();
    options.been_saved_true();
    options.changes_false();
    println!("Budgets successfully saved");
}


fn check_current_budget(current_budget: &mut Option<usize>) -> bool {
    if *current_budget == None {
        println!("Choose the budget to operate first, please\n type \"get budget\"");
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