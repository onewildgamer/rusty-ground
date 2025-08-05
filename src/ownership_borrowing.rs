fn ownership() {
    println!("Hello, world!");
    let s1 = String::from("Let's understand rust this time");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}")
}

fn borrowing() {
    let mut x = vec![1, 2, 3];
    let last = x.last().unwrap();
    x.push(4);

    println!("{:?}", last);
}
