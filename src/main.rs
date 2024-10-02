fn main() {
    let mut budget1 = Budget::new(String::from("First budget"), 500);

    budget1.add(50);
    budget1.remove(150);
    budget1.show_transactions();
    budget1.edit(1, Some("delete"), None);
    budget1.edit(1, None, Some(-50));
    budget1.show_transactions();
    println!("Left budget is: {}", budget1.get_budget());
    budget1.delete_self();
}

struct Budget {
    name: String,
    value: i32,
    transactions: Vec<Transaction>,
}

#[derive(Debug)]
struct Transaction {
    value: i32,
    method: TransactionMethod,
}

#[derive(Debug)]
enum TransactionMethod {
    Add,
    Remove,
}

impl Budget {

    fn new(name: String, amount: i32) -> Budget {
        Budget {
            name,
            value: amount,
            transactions: Vec::new(),
        }
    }
    
    fn add(&mut self, amount: i32) -> () {
        let method = TransactionMethod::Add;
        self.transactions.push(Transaction { value: amount, method });
        self.value += amount
    }

    fn remove(&mut self, amount: i32) -> () {
        let method = TransactionMethod::Remove;
        self.transactions.push(Transaction { value: -amount, method });
        self.value -= amount
    }

    fn show_transactions(&self) -> () {
        for (i, transaction) in self.transactions.iter().enumerate() {
            println!("{}. {:?} {}", i+1, transaction.method, transaction.value);
        }
    }

    fn edit(&mut self, index: i32, method: Option<&str>, amount: Option<i32>) -> () {
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

    fn get_budget(&mut self) -> i32 {
        for transaction in &self.transactions {
            self.value += transaction.value;
        }
        self.value
    }

    fn delete_self(self) {
        println!("Deleting the budget: {}", self.name);
        println!("The budget was deleted");
    }

}