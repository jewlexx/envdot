use envdot::dotenv;

#[test]
fn print_test() {
    dotenv!(".test.env");

    println!("{}", ENV_FILE)
}
