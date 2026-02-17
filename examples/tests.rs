//! Tip: Inspect outputs by running `cargo expand --test --example tests`

/// computes the max of the given numbers
fn do_something(a: i32, b: i32) -> i32 {
    if a > b { b } else { a }
}

#[cfg(test)]
mod tests {
    use ai_bindgen::ai;

    #[ai]
    extern "C" {
        #[ai(prompt = "Generate some test case for the magic function above, please")]
        #[test]
        fn test_cases();
    }
}
