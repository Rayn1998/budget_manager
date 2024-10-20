pub enum Command {
    NewBudget,
    ShowBudgets,
    ShowBallance,
    Add,
    Remove,
    EditTransaction,
    ShotTransactions,
    GetBudget,
    DeleteBudget,
    Help,
    Save,
    Exit,
    Invalid,
}

impl Command {
    pub fn input_match_command(input: &str) -> Command {
        match input {
            "new budget" => Command::NewBudget,
            "show budgets" => Command::ShowBudgets,
            "set budget" => Command::GetBudget,
            "show ballance" => Command::ShowBallance,
            "add" => Command::Add,
            "remove" => Command::Remove,
            "edit transaction" => Command::EditTransaction,
            "show transactions" => Command::ShotTransactions,
            "delete budget" => Command::DeleteBudget,
            "help" => Command::Help,
            "save" => Command::Save,
            "exit" => Command::Exit,
            _ => Command::Invalid,
        }
    }
}