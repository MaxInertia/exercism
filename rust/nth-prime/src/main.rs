use std::time::{Instant};

mod lib;

fn main() {
    bench()
}

fn bench() {
    print!("warming up...\n");
    lib::nth_naive(10_000);

    {
        print!("running benchmark for 10,000th prime...\n");

        let now = Instant::now();
        lib::nth_naive(10_000);
        let method_1_time = now.elapsed().as_millis();
        
        let now2 = Instant::now();
        lib::nth_with_caching(10_000);
        let method_2_time = now2.elapsed().as_millis();

        print!("method 1: {}\n", method_1_time);
        print!("method 2: {}\n", method_2_time);
    }

    {
        print!("running benchmark for 12,000th prime...\n");

        let now = Instant::now();
        lib::nth_naive(12_000);
        let method_1_time = now.elapsed().as_millis();
        
        let now2 = Instant::now();
        lib::nth_with_caching(12_000);
        let method_2_time = now2.elapsed().as_millis();

        print!("method 1: {}\n", method_1_time);
        print!("method 2: {}\n", method_2_time);
    }
}