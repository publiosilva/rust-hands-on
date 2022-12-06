struct Holder {
    name: String,
    surname: String,
}

struct Account {
    holder: Holder,
    balance: f32,
}

impl Account {
    fn withdraw(&mut self, value: f32) {
        self.balance -= value;
    }
}

fn main() {
    let holder: Holder = Holder {
        name: String::from("John"),
        surname: String::from("Doe"),
    };
    let mut account: Account = Account {
        holder,
        balance: 100.0,
    };

    account.withdraw(50.0);

    println!(
        "Holder = {} {}, Balance = {}",
        account.holder.name, account.holder.surname, account.balance
    );
}
