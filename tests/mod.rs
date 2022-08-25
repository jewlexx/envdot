use std::env;

use envdot::init_dotenv;

#[test]
fn print_test() {
    init_dotenv!(".test.env");

    assert_eq!(env::var("PROJECT"), Ok("envdot".to_string()));
}
