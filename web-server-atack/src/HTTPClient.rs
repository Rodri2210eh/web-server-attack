use std::io::{stdout, Write};
use curl::easy::Easy;
use std::env;
#[path = "./prethread-webserver.rs"] mod prethreadwebserver;

pub fn main(){
  env::set_var("RUST_BACKTRACE", "1");
  let args: Vec<String> = env::args().collect();
  // Write the contents of rust-lang.org to stdout
  let mut easy = Easy::new();
  let mut i = 1;
  println!("{}", args[i]);
  if args[i] == "-h"{
     println!("Bienvenido al HTTPClient, con este programa se conectara al servidor usando la libreria curl \n\n");
     i = i + 1;
  }
  if args[i] == "prethread-webserver"{
     prethreadwebserver::main();
     easy.url("http://localhost:3000/").unwrap();
     easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
     }).unwrap();
     easy.perform().unwrap();
  }
}


