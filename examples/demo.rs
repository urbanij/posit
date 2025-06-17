use posit::Posit;
use typenum::{U7, U8};

fn main() {
    let p = Posit::<u8, U8, U7>::new(24);
    println!("{}", p);
    println!("{:?}", p);

    let p2 = posit::P16E1(2.3);
    // println!("{}", p2);
    println!("{:?}", p2.unwrap());
}
