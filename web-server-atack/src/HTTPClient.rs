use std::io::{stdout, Write};
use curl::easy::Easy;

fn main(){
  println!("Inicio del HTTPClient");
  // Write the contents of rust-lang.org to stdout
  let mut easy = Easy::new();
  easy.url("http://localhost:3000/").unwrap();
  easy.write_function(|data| {
    stdout().write_all(data).unwrap();
    Ok(data.len())
  }).unwrap();
  easy.perform().unwrap();
}


