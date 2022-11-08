fn main() {
  for x in 1..=20 {
    println!("{}: {}", x, match x {
      1 => "One",
      2 | 3 => "Two or three",
      4..=18 => "Between four and eighteen",
      _ if x % 2 != 0 => "Nineteen",
      _ => "Twenty",
    });
  }
}
