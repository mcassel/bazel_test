use super::*;

#[test]
fn test_greeting() {
    let g = Greeter::new("Hello");
    assert_eq!(g.greet("World"), "Hello World!")
}