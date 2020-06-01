use std::env;

mod list_combos;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  let n = &args[1].parse::<u8>();
  let k = &args[2].parse::<u8>();
  let n = match n {
    Ok(x) => x,
    Err(error) => panic!(
      "Problem parsing &args[1] as u8: {:?}, {:?}",
      &args[1], error
    ),
  };
  let k = match k {
    Ok(x) => x,
    Err(error) => panic!(
      "Problem parsing &args[2] as u8: {:?}, {:?}",
      &args[2], error
    ),
  };
  let combos = list_combos::list_combos(*n, *k);
  println!("{:?}", combos);
}
