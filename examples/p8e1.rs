use cast::f64;
use posit::Posit;
use posit::P8E1;
use typenum::{U1, U31};

fn main() {
    for i in 0u16.. {
        if i > u8::MAX as u16 {
            break;
        }

        let p = P8E1::new(i as u8);
        println!("{:3} - {}", i, f64(p));
    }

    let p2 = Posit::<u32, U31, U1>::new(2423);
    println!("{}", p2);
}
