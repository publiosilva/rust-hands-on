const PI:f32 = 3.14;

static EULER_NUMBER:f32 = 2.71;

fn main() {
  let age:i32 = 32;
  println!("Age: {}, size: {} byte(s)", age, std::mem::size_of_val(&age));

  let height:f32 = 1.80;
  println!("Height: {}", height);

  let has_job:bool = false;
  println!("Has job: {}, size: {} byte(s)", has_job, std::mem::size_of_val(&has_job));
  let has_job:bool = true;
  println!("Has job: {}, size: {} byte(s)", has_job, std::mem::size_of_val(&has_job));

  let sex:char = 'M';
  println!("Sex: {}, size: {} byte(s)", sex, std::mem::size_of_val(&sex));

  println!("PI: {}", PI);

  println!("Euler number: {}", EULER_NUMBER);
}
