//mutalbe reference
// cerate refrence by "&"
//Immutabel reference

// fn main(){
//     let x: i32 = 5;
//     // let r = x //not good for memory, since ownership is transfred
//     let r: &i32 = &x; // declaration -> referece datatype
//     // this is an immutable reference
//     println!("Value for x: {}", x);
//     println!("Value for r: {}", r);

// }

// mutalbe reference
// fn main(){
//     let mut x: i32 = 5;
//     let r: &mut i32 = & mut x; // declaration -> referece datatype
//     *r = *r+1;
//     *r -= 3;
//     // need to define both the owner and reference as mutable
//     println!("Value for x: {}", x);
//     println!("Value for r: {}", r)
// }

// Sturuct
fn main(){
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 1576.55
    };

    //check the bank account
    account.check_balance();

    //check account withdrawal
    account.withdraw(200.0);

    //check the bank account
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    // in the function withdraw
        // can have immutable function for reading account balance
        // mutable function to withdraw the money from the bank account
    fn withdraw(&mut self, amount: f64){
        println!("withdrawing {} from acccount owned by {}", amount, self.owner);
        self.balance -= amount
    }

    fn check_balance(&self){
        println!("Account balance {} for the account owner {}", self.balance, self.owner);
    }
}
