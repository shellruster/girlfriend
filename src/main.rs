use girlfriend::{Girlfriend, User};

fn main() {
    let me = User::new("UwUssimo", "Robinson");
    let gf = Girlfriend::new(me);

    gf.talk("Hello!");
}
