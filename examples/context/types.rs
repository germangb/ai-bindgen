use ai_bindgen::ai;
use std::fmt;

#[derive(Debug)]
pub struct Person {
    name: String,
    surname: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[ai]
        extern "C" {
            #[ai(prompt = "format the person type in a nice readable way (a single line please")]
            fn format_person(person: &Person, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }

        format_person(self, f)
    }
}

impl Person {
    pub fn new(name: impl Into<String>, surname: impl Into<String>, age: u32) -> Self {
        Self {
            name: name.into(),
            surname: surname.into(),
            age,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn surname(&self) -> &str {
        &self.surname
    }

    pub fn age(&self) -> u32 {
        self.age
    }
}
