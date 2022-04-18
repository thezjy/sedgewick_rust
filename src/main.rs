fn main() {
    println!("yes");
    let x = vec!["yes".to_string(); 100_0000];
    println!("{:?}", &x[0..100]);
}
