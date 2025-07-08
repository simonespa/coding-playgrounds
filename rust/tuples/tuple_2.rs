fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  let x = tup.0;
  let y = tup.1;
  let z = tup.2;

  println!("The values are {x}, {y} and {z}");
  println!("The tuple is: {:?}", tup);

  println!("One element tuple: {:?}", (5,));
  println!("Just an integer: {:?}", (5));
}
