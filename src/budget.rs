#[derive(Debug)]
pub struct Budget {
    pub name: String,
    pub value: i32,
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
        self.value += amount
    }

    pub fn remove(&mut self, amount: i32) -> () {
        let method = TransactionMethod::Remove;
        self.transactions.push(Transaction { value: -amount, method });
        self.value -= amount
    }

    pub fn show_transactions(&self) -> () {
        for (i, transaction) in self.transactions.iter().enumerate() {
            println!("{}. {:?} {}", i+1, transaction.method, transaction.value);
        }
    }

    pub fn edit(&mut self, index: i32, method: Option<&str>, amount: Option<i32>) -> () {
        match method {
            Some("delete") => {
                println!("Deleting the {}'th transaction", index);
                let deleted = &mut self.transactions.remove(index as usize -1);
                println!("Deleted transaction: {}", deleted.value);
                println!("Deleting happenned successfully");
            },
            Some(_) => {
                println!("Unsupported method");
            }
            None => ()
        }

        match amount {
            Some(value) => {
                self.transactions[index as usize-1].value = value;
            },
            None => ()
        }
    }

    pub fn get_budget(&mut self) -> i32 {
        for transaction in &self.transactions {
            self.value += transaction.value;
        }
        self.value
    }

    pub fn delete_self(self) {
        println!("Deleting the budget: {}", self.name);
        println!("The budget was deleted");
    }

}