fn main() {
    let mut s = String::from("Helloo");

    // First "s" is no longer valig as we are moving it to the funtion
    // then it takes back the ownership from the function
    // let s1 = takes_and_gives_ownership(s);

    // borrows_mutable_reference(&mut s);

    // println!("s is {}", s);
    multiple_references();
}

// This function takes the ownership of "some_str" and gives it back
fn takes_and_gives_ownership(some_str: String) -> String {
    some_str
}

fn does_not_take_ownership(str: &String) -> usize {
    str.len()
}

fn borrows_mutable_reference(str: &mut String) {
    str.push_str(" World");
}

fn multiple_references() {
    let mut s = String::from("Hello");

    /*
     *  If we do the other way:
     *
     *      let r1 = &mut s;
     *      let r2 = &s;
     *
     *  We can't do this, because we can't have more than one reference to one value!
     *  As a rule:
     *      - At any given time, you can have either one mutable reference or any number of immutable references.
     *      - References must always be valid.
     *
     */
    let r1 = &s;
    let r2 = &s;

    println!("s is {}, r1 is {}, r2 is {}", s, r1, r2);
}
