use girlfriend::{Girlfriend, User};
use std::io;
use std::io::{stdout, Write};

fn main() {
    let me = User::new("UwUssimo", "Robinson");
    let gf = Girlfriend::new(me.clone());

    let chatter = me.firstname;

    loop {
        print!("{}: ", &chatter);
        stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                gf.talk(input.as_str());
            }
            Err(error) => print!("Error: {}", error),
        }
    }
}
