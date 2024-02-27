fn main() {
    println!("Loop with return {}", loop_with_return());

    loop_with_label();

    for_in();
}

fn loop_with_return() -> i32 {

    let mut i = 10;

    
    loop {

        if i == 1 {
            break i;
        }

        i -= 1;
    }
}

fn loop_with_label() {

    let mut i = 10;

    'loop_label: loop {
        println!("I: {i}");

        let mut something = 6;

        loop {
            println!("Something: {something}");

            if something == 0 {
                break;
            }

            if i == 4 {
                break 'loop_label;
            }

            something -= 1;
        }
        
        i -= 1;
    }
}

fn for_in() {

    for num in (2..7).rev() {
        println!("Num plus 3 {}", num + 3);
    }
}