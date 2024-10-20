use crate::Budget;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Budgets {
    pub budgets: Vec<Budget>,
}

impl Budgets {
    pub fn new() -> Budgets {
        Budgets {
            budgets: Vec::new(),
        }
    }

    pub fn add_budget(&mut self, budget: Budget) {
        self.budgets.push(budget);
    }
}