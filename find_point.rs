
use halo2curves::pasta::Fp;
use ff::{Field, PrimeField};

fn main() {
    for i in 0..100 {
        let x = Fp::from(i);
        let y_sq = x * x * x + Fp::from(5);
        let y = y_sq.sqrt();
        if y.is_some().into() {
            println!("x: {}, y: {:?}", i, y.unwrap());
        }
    }
}
