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
        match method {
            EditInput::Amount(value) => {
                self.transactions[index as usize-1].value = value;
            },
            EditInput::Delete => {
                let deleted = &mut self.transactions.remove(index as usize -1);
                println!("Deleted transaction: {}", deleted.value);
                println!("Deleting happenned successfully");
            }
        }
    }

    pub fn get_ballance(&self) -> i32 {
        // let mut ballance: i32 = self.value;
        // for transaction in &self.transactions {
        //     match transaction.method {
        //         TransactionMethod::Add => {
        //             ballance += transaction.value;
        //         },
        //         TransactionMethod::Remove => {
        //             ballance -= transaction.value;
        //         }
        //     } 
        // }
        // ballance
        self.value
    }
}

pub enum EditInput {
    Delete,
    Amount(i32),
}