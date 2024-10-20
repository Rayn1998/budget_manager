use serde::{ Serialize, Deserialize }; 

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
        self.transactions.push(Transaction { value: amount, method });
        self.value = self.value + amount;
    }

    pub fn remove(&mut self, amount: i32) -> () {
        if amount > self.value {
            println!("You can't go the negative ballance");
            return;
        }
        let method = TransactionMethod::Remove;
        self.transactions.push(Transaction { value: - amount, method });
        self.value = self.value - amount;
    }

    pub fn show_transactions(&self) -> () {
        for (i, transaction) in self.transactions.iter().enumerate() {
            println!("{}. {:?} {}", i+1, transaction.method, transaction.value);
        }
    }

    pub fn edit(&mut self, index: i32, method: EditInput) -> () {
        let old_method = &self.transactions[index as usize-1].method;

        match method {
            EditInput::Amount(value) => {
                match old_method {
                    TransactionMethod::Add => {
                        // Change + to -
                        let old_value = self.transactions[index as usize-1].value;

                        if value.is_negative() {
                            let difference = old_value + value.abs();
                            self.transactions[index as usize-1].method = TransactionMethod::Remove;
                            self.transactions[index as usize-1].value = value;
                            self.value = self.value - difference;
                        } else {
                            // Change + to + 
                            let difference = old_value - value;

                            if old_value > value {
                                self.transactions[index as usize-1].value = value;
                                self.value = self.value - difference;
                            } else {
                                self.transactions[index as usize-1].value = value;
                                self.value = self.value + difference.abs();
                            }
                        }
                    },
                    TransactionMethod::Remove => {
                        // Change - to -
                        let old_value = self.transactions[index as usize-1].value;

                        if value.is_negative() {
                            let difference = old_value - value;

                            if old_value > value {
                                self.value = self.value - difference.abs();
                                self.transactions[index as usize-1].value = value;
                            } else {
                                self.value = self.value - difference;
                                self.transactions[index as usize-1].value = value;
                            }
                        } else {
                            // Change - to +
                            let difference = old_value.abs() + value;
                            self.transactions[index as usize-1].method = TransactionMethod::Add;
                            self.transactions[index as usize-1].value = value;
                            self.value = self.value + difference;
                        }
                    }
                }
            },
            EditInput::Delete => {
                let deleted = &mut self.transactions.remove(index as usize -1);
                println!("Deleted transaction: {}", deleted.value);
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