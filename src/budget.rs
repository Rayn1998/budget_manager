use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Budget {
    pub name: String,
    value: i32,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    pub value: i32,
    pub method: TransactionMethod,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionMethod {
    Add,
    Remove,
}

impl Budget {
    pub fn new(name: String, amount: i32) -> Budget {
        Budget {
            name,
            value: amount,
            transactions: Vec::new(),
        }
    }

    pub fn add(&mut self, amount: i32) -> () {
        let method = TransactionMethod::Add;
        self.transactions.push(Transaction {
            value: amount,
            method,
        });
        self.value = self.value + amount;
    }

    pub fn remove(&mut self, amount: i32) -> () {
        if amount > self.value {
            println!("You can't go the negative ballance");
            return;
        }
        let method = TransactionMethod::Remove;
        self.transactions.push(Transaction {
            value: -amount,
            method,
        });
        self.value = self.value - amount;
    }

    pub fn show_transactions(&self) -> () {
        if self.transactions.len() > 0 {
            for (i, transaction) in self.transactions.iter().enumerate() {
                println!("{}. {:?} {}", i + 1, transaction.method, transaction.value);
            }
        } else {
            println!("{}", "There are no transactions yet...".on_red().black());
        }
    }

    pub fn edit(&mut self, index: i32, method: EditInput) -> () {
        let index = index as usize - 1;
        let old_method = &self.transactions[index].method;

        match method {
            EditInput::Amount(value) => {
                match old_method {
                    TransactionMethod::Add => {
                        // Change + to -
                        let old_value = self.transactions[index].value;

                        if value.is_negative() {
                            let difference = old_value + value.abs();
                            let new_value: i32 = self.value - difference;

                            if new_value < 0 {
                                println!("{}", "Ballance can't be less than zero".red());
                                return;
                            } else {
                                self.transactions[index].method = TransactionMethod::Remove;
                                self.transactions[index].value = value;
                                self.value = self.value - difference;
                            }
                        } else {
                            // Change + to +
                            let difference = old_value - value;

                            if old_value > value {
                                self.transactions[index].value = value;
                                self.value = self.value - difference;
                            } else {
                                self.transactions[index].value = value;
                                self.value = self.value + difference.abs();
                            }
                        }
                    }
                    TransactionMethod::Remove => {
                        // Change - to -
                        let old_value = self.transactions[index].value;

                        if value.is_negative() {
                            let difference = old_value - value;

                            if old_value > value {
                                self.value = self.value - difference.abs();
                                self.transactions[index].value = value;
                            } else {
                                self.value = self.value - difference;
                                self.transactions[index].value = value;
                            }
                        } else {
                            // Change - to +
                            let difference = old_value.abs() + value;
                            self.transactions[index].method = TransactionMethod::Add;
                            self.transactions[index].value = value;
                            self.value = self.value + difference;
                        }
                    }
                }
            }
            EditInput::Delete => {
                match self.transactions[index].method {
                    TransactionMethod::Add => {
                        let deleted = &mut self.transactions.remove(index);
                        self.value -= deleted.value;
                        println!("Deleted transaction: {}", deleted.value);
                    }
                    TransactionMethod::Remove => {
                        let deleted = &mut self.transactions.remove(index);
                        self.value += deleted.value;
                        println!("Deleted transaction: {}", deleted.value);
                    }
                }
                println!("Deleting happenned successfully");
            }
        }
    }

    pub fn get_ballance(&self) -> i32 {
        self.value
    }
}

pub enum EditInput {
    Delete,
    Amount(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_budget() {
        let budget: Budget = Budget::new(String::from("test_budget"), 500);
        assert_eq!(budget.name, "test_budget");
        assert_eq!(budget.value, 500);
        assert_eq!(budget.transactions.len(), 0);
    }

    #[test]
    fn test_add_money_to_budget() {
        let mut budget: Budget = Budget::new(String::from("test_budget"), 100);
        budget.add(50);
        assert_eq!(budget.value, 150);
        assert_eq!(budget.transactions.len(), 1);
        assert_eq!(budget.transactions[0].value, 50);
        assert!(matches!(
            budget.transactions[0].method,
            TransactionMethod::Add
        ));
    }

    #[test]
    fn test_remove_money_to_budget() {
        let mut budget: Budget = Budget::new(String::from("test_budget"), 100);
        budget.remove(20);
        assert_eq!(budget.value, 80);
        assert_eq!(budget.transactions.len(), 1);
        assert_eq!(budget.transactions[0].value, -20);
        assert!(matches!(
            budget.transactions[0].method,
            TransactionMethod::Remove
        ));
    }

    #[test]
    fn test_remove_more_than_budget_has() {
        let mut budget: Budget = Budget::new(String::from("test_budget"), 100);
        budget.remove(200);
        assert_eq!(budget.value, 100);
        assert_eq!(budget.transactions.len(), 0);
    }

    #[test]
    fn test_edit_transaction() {
        let mut budget: Budget = Budget::new(String::from("test_budget"), 100);
        budget.add(100);
        budget.remove(50);
        budget.edit(1, EditInput::Amount((-50)));
        assert_eq!(budget.value, 0);
        assert_eq!(budget.transactions[0].value, -50);
        assert!(matches!(
            budget.transactions[0].method,
            TransactionMethod::Remove
        ));
    }

    #[test]
    fn test_delete_transaction() {
        let mut budget: Budget = Budget::new(String::from("test_budget"), 100);
        budget.add(100);
        budget.edit(1, EditInput::Delete);
        assert_eq!(budget.value, 100);
        assert_eq!(budget.transactions.len(), 0);
    }

    #[test]
    fn test_show_transactions_empty() {
        let budget = Budget::new("My Budget".to_string(), 100);
        budget.show_transactions();
    }

    #[test]
    fn test_get_ballance() {
        let mut budget = Budget::new("My Budget".to_string(), 100);
        budget.add(50);
        budget.remove(30);
        assert_eq!(budget.get_ballance(), 120);
    }
}
