fn main() {
    let s = String::from("Helloo");

    // First "s" is no longer valig as we are moving it to the funtion
    // then it takes back the ownership from the function
    let s1 = takes_and_gives_ownership(s);

    println!("{s1}");
}

// This function takes the ownership of "some_str" and gives it back
fn takes_and_gives_ownership(some_str: String) -> String {
    some_str
}
