fn main() {
  let n1 = 0b1011;
  let n2 = 0b0001;
  let and = n1 & n2;
  let or = n1 | n2;
  let xor = n1 ^ n2;

  println!("{:04b} AND {:04b} = {:04b}", n1, n2, and);
  println!("{:04b} OR {:04b} = {:04b}", n1, n2, or);
  println!("{:04b} XOR {:04b} = {:04b}", n1, n2, xor);
  println!("{:04b} >> {} = {:04b}", n1, 2, n1 >> 2);
  println!("{:04b} << {} = {:04b}", n1, 2, n1 << 2);
}
