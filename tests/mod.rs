use envdot::dotenv;

#[test]
fn print_test() {
    dotenv!(".test.env");

    println!("{}", file!());

    // println!("{}", ENV_FILE)
}
