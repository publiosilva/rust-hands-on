fn main() {
  match emit_recoverable_error() {
    Ok(success_message) => println!("Success message: {}", success_message),
    Err(error_code) => println!("Error code: {}", error_code),
  }

  emit_unrecoverable_error();
}

fn emit_recoverable_error() -> Result<String, u16> {
  let emit_error:bool = true;

  if emit_error {
    Err(500)
  } else {
    Ok(String::from("Success"))
  }
}

fn emit_unrecoverable_error() {
  panic!("unrecoverable error")
}
