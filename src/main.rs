use cpc::eval;
use cpc::units::Unit;

/// cpc CLI interface
fn main() {
  use std::env;
  let args: Vec<String> = env::args().collect();
  let mut debug = false;
  let mut verbose = false;
  if args.iter().any(|i| i=="--debug") {
    debug = true;
  }
  if args.iter().any(|i| i=="--verbose") {
    verbose = true;
  }
  if args.len() >= 2 {
    match eval(&args[1], true, Unit::Celcius, debug) {
      Ok(answer) => {
        if !debug {
          print!("{} ", answer.value);
          if verbose {
            println!("{:?}", answer.unit)
          } else {
            println!("{}", answer.unit)
          }
        }
      },
      Err(e) => {
        println!("{}", e)
      },
    }
  } else {
    println!("No argument supplied");
  }
}
