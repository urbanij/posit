use cast;
use posit::Posit;
use posit::P8E1;
use typenum::{U0, U1, U10, U16, U31, U7, U8};

fn main() {
  let p = Posit::<u8, U8, U7>::new(24);
  println!("{p}");
}
