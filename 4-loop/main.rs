fn main() {
  let multiplier:u8 = 5;
  let mut counter:u8 = 0;

  while counter < 10 {
    counter += 1;

    if counter == 5 {
      continue;
    }

    println!("{} x {} = {}", multiplier, counter, multiplier * counter);
  }

  counter = 0;

  loop {
    counter += 1;

    if counter == 5 {
      continue;
    }

    println!("{} x {} = {}", multiplier, counter, multiplier * counter);

    if counter == 10 {
      break;
    }
  }

  for i in 1..=10 {
    if i == 5 {
      continue;
    }

    println!("{} x {} = {}", multiplier, i, multiplier * i);
  }
}
