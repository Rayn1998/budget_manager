pub enum Command {
    NewBudget,
    ShowBudgets,
    ShowBallance,
    Add,
    Remove,
    Edit,
    ShotTransactions,
    GetBudget,
    DeleteBudget,
    Help,
    Exit,
    Invalid,
}

impl Command {
    pub fn input_match_command(input: &str) -> Command {
        match input {
            "new budget" => Command::NewBudget,
            "show budgets" => Command::ShowBudgets,
            "get budget" => Command::GetBudget,
            "show ballance" => Command::ShowBallance,
            "help" => Command::Help,
            "exit" => Command::Exit,
            _ => Command::Invalid,
        }
    }
}