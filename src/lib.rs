use chrono::Datelike;

#[derive(Debug, Clone)]
pub struct User {
    pub firstname: String,
    pub lastname: String,
}

impl User {
    pub fn new(first: &str, last: &str) -> Self {
        User {
            firstname: first.to_string(),
            lastname: last.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.firstname.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Girlfriend {
    firstname: String,
    lastname: String,
    age: i32,
    owner: User,
}

impl Girlfriend {
    pub fn new(owner: User) -> Self {
        let current_date = chrono::Utc::now();

        Girlfriend {
            firstname: String::from("Sara"),
            lastname: String::from("Marchiafava"),
            age: current_date.year() - 2022,
            owner,
        }
    }

    pub fn owner(&self) -> &User {
        &self.owner
    }

    pub fn talk(&self, message: &str) {
        print!("{}: {}", self.firstname, message)
    }
}
