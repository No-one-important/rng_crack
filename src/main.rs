use oorandom;

fn main() {
    for i in 0..1_000_000_000 {
        let mut rng = oorandom::Rand32::new(i);
        let a = rng.rand_u32() % 255;
        let b = rng.rand_u32() % 255;
        let c = rng.rand_u32() % 255;

        if a == 42 && b == 42 && c == 42 {
            println!("{}", i);
        }
    }

}
