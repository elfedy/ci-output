#[test]
fn test_it_works() {
    println!("stdout");
    eprintln!("stderr");
    println!("ci should output all of this");
    panic!("sorry, it doesn't work");
}
