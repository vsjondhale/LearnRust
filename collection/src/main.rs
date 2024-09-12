fn main() {
    //Veator
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third = &v[2];
    //println!("The third element is {} ", third);

    match v.get(20) {
        Some(third)=> println!("The thrd element is {}", third),
        None => println!("There is no third element")
    }

    let mut itirate = vec![1,2,3,4,5];

    for i in &mut itirate {
        *i += 50;
    }
    
    for i in &mut itirate {
        println!("{}", i);
    }

    //String Collection of UTF8 encoded bytes
    let s1 = String::from("A");
    s1.push_str("B");


    let s2 = String::from("C");
    let s3 = String::from("D");
    let s4 = format!("{}{}", s1, s2);

    //HashMap allows to store the value in key value pair
    let mut score = HashMap::new();

    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Blue"), 20);

    score.entry(String::from("Yellow")).or_insert(30);
    score.entry(String::from("Yellow")).or_insert(40);


}
