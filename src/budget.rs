#[derive(Debug)]
pub struct Budget {
    pub name: String,
    value: i32,
    transactions: Vec<Transaction>,
}

#[derive(Debug)]
struct Transaction {
    pub value: i32,
    pub method: TransactionMethod,
}

#[derive(Debug)]
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
    }

    pub fn remove(&mut self, amount: i32) -> () {
        let method = TransactionMethod::Remove;
        self.transactions.push(Transaction { value: amount, method });
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

    pub fn get_budget(&self) -> i32 {
        let mut ballance: i32 = self.value;
        for transaction in &self.transactions {
            match transaction.method {
                TransactionMethod::Add => {
                    ballance += transaction.value;
                },
                TransactionMethod::Remove => {
                    ballance -= transaction.value;
                }
            } 
        }
        ballance
    }

    // pub fn delete_self(self) {
    //     println!("Deleting the budget: {}", self.name);
    //     println!("The budget was deleted");
    // }

}

pub enum EditInput {
    Delete,
    Amount(i32),
}