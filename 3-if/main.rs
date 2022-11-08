fn main() {
  let age:u8 = 24;

  if age > 21 {
    println!("Adult");
  } else if age >= 18 {
    println!("Young adult");
  } else if age > 12 {
    println!("Teen");
  } else {
    println!("Child");
  }

  let can_drive = if age >= 16 { "Yes" } else { "No" };

  println!("Can you drive? {}", can_drive);
}
