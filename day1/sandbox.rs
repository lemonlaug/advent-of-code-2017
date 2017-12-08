fn main() {
    let c = String::from("1");
    let c = c.chars();

    for char in c {
        let cs = c.to_string();
        println!("{:?}", char);
    }
}
