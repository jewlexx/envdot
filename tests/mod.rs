use envdot::dotenv;

dotenv!();

#[test]
fn print_test() {
    println!("{}", your_mother())
}
