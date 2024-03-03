
fn main() {
    
  println!("Is odd, {}", is_odd(2));
  println!("Is odd, {}", is_odd(1));

}


fn is_odd(number: i32) -> bool {

  if number % 2 == 0 {
    true
  } else {
    false
  }
}