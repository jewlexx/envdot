use envdot::dotenv;

dotenv!(".env.local");

#[test]
fn print_test() {
    println!("{}", your_mother())
}
