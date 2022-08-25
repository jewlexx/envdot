use std::env;

use envdot::dotenv;

#[test]
fn print_test() {
    dotenv!(".test.env");

    assert_eq!(env::var("PROJECT"), Ok("envdot".to_string()));
}
