use cast;
use posit::P8E1;
use posit::Posit;
use typenum::{U0, U1, U8, U31};

fn main() {
    for i in 0u16.. {
        if i > u8::MAX as u16 {
            break;
        }

        let p = P8E1::new(i as u8);
        println!("{:3} - {}", i, cast::f64(p));
    }

    let p2 = Posit::<u32, U31, U1>::new(2423);
    println!("{}", p2);

    let p2 = Posit::<u8, U8, U0>::new(123);
    let value = cast::f64(p2);
    println!("{}", value);

    let p3 = posit::P16E1(2.3);
    println!("{:?}", p3);
    println!("{}", cast::f64(p3.unwrap()));
}
