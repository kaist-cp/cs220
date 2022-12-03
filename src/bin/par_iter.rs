use std::time::Instant;

use rayon::prelude::*;

const SIZE: usize = 100_000_000;

fn sequential() {
    let _v = (0..SIZE)
        .into_iter()
        .filter_map(|x| if x % 2 == 0 { Some(x * 3) } else { None })
        .collect::<Vec<_>>();
}

fn parallel() {
    let _v = (0..SIZE)
        .into_par_iter()
        .filter_map(|x| if x % 2 == 0 { Some(x * 3) } else { None })
        .collect::<Vec<_>>();
}

fn bench<F>(name: &str, f: F)
where
    F: FnOnce(),
{
    let begin = Instant::now();
    f();
    let elapsed = begin.elapsed();
    println!("{}: {:.2?}", name, elapsed);
}

fn main() {
    bench("sequential", sequential);
    bench("parallel", parallel);
}
