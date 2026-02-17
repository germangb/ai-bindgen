use ai_bindgen::ai;
use types::Person;

#[path = "context/types.rs"]
mod types;

#[ai]
extern "C" {
    #[ai(
        prompt = "Generate n random people (use real human names, not gibberish, please)",
        context = "context/types.rs"
    )]
    fn generate_random_people(n: usize) -> Vec<Person>;

    #[ai(
        prompt = "Return an iterator of the people who are legally allowed to drink.",
        context = "context/types.rs"
    )]
    fn drink_age(people: &[Person]) -> impl Iterator<Item = &Person>;
}

fn main() {
    let people = generate_random_people(16);

    println!("List of people\n===");

    for person in &people {
        println!("- {person}");
    }

    println!("\nPeople allowed to drink\n===");

    for person in drink_age(&people) {
        println!("- {person}");
    }
}
