fn main() {
    let s1: String = String::from("RUST"); //s1 is the owner of the RUST value
    let len = calulate_length(&s1);
    println!("Length of the '{}' string is: '{}'", s1, len);
}

fn calulate_length(s: &String) -> usize {
    s.len()
}
  
