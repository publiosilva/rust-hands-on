fn main() {
  /*
   * This variable will be stored in heap memory. This variable type can have only one owner.
   */
  let str = String::from("Hello world");

  /*
   * By doing this you are borrowing the variable's value to the steal_str function
   * This is an immutable reference. To create a mutable reference you must write &mut.
   */
  steal_str(&str);

  /*
   * if you pass a String and not a String to the steal_str function,
   * you will not be able to execute the line below because the steal_str function would steal ownership of the variable.
   */
  println!("{}", str);
}

fn steal_str(str: &String) {
  println!("{}", str);
}
