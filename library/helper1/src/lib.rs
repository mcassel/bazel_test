pub struct Greeter {
    greeting: String,
}

impl Greeter {
    pub fn new(greeting: &str) -> Greeter {
        Greeter {
            greeting: greeting.to_string(),
        }
    }

    pub fn greet(&self, name: &str) -> String {
        format!("{} {}!", self.greeting, name)
    }
}

#[cfg(test)]
#[path = "greeter_tests.rs"]
mod greeter_tests;