use envdot::dotenv;

#[test]
fn print_test() {
    dotenv!(".env.local");

    println!("{}", ENV_FILE)
}
