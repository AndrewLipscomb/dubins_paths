#![allow(clippy::incompatible_msrv)]

use core::hint::black_box;

use dubins_paths::{DubinsPath, FloatType, PI};
use rand::Rng;

fn main() {
    let runs = 1_000_000;
    let range: FloatType = 10000.0;

    let mut thread_rng = rand::thread_rng();

    for _ in 0..runs {
        let q0 = [
            thread_rng.gen_range(-range..range),
            thread_rng.gen_range(-range..range),
            thread_rng.gen_range((-2. as FloatType * PI)..(2. as FloatType * PI)),
        ]
        .into();
        let q1 = [
            thread_rng.gen_range(-range..range),
            thread_rng.gen_range(-range..range),
            thread_rng.gen_range((-2. as FloatType * PI)..(2. as FloatType * PI)),
        ]
        .into();

        let rho = thread_rng.gen_range((600.0 as FloatType)..(3000. as FloatType));

        let Ok(path) = DubinsPath::shortest_from(black_box(q0), q1, rho) else {
            continue;
        };

        let step_distance = thread_rng.gen_range((5.0 as FloatType)..(rho / (100.0 as FloatType)));
        let _ = path.sample_many(black_box(step_distance));
    }
}
