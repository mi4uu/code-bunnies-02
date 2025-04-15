pub mod project;
pub mod providers;
pub mod tools;

#[cfg(test)]
mod tests {

    #[test]
    fn test_example_from_prompt() {
        use super::providers::providers::Providers;
        let providers = Providers::load();
        println!("Providers: {:?}", providers);
    }
}
