use crate::blockchain::Blockchain;
use std::time::{Duration, SystemTime};

mod block;
mod blockchain;
mod hash;

const DIFFICULTY: usize = 3;

fn timeit<F: FnMut() -> T, T>(mut f: F) -> (T, Duration) {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    (result, duration)
}

fn main() -> Result<(), jiff::Error> {
    let mut blockchain = Blockchain::new();

    let (_, duration) = timeit(|| blockchain.mine_block(vec![], DIFFICULTY));
    println!(
        "Mining block #0 took {}s",
        duration.as_millis() as f32 / 1000.
    );
    let (_, duration) = timeit(|| blockchain.mine_block(vec![0, 1, 255], DIFFICULTY));
    println!(
        "Mining block #1 took {}s",
        duration.as_millis() as f32 / 1000.
    );
    let (_, duration) = timeit(|| blockchain.mine_block(vec![], DIFFICULTY));
    println!(
        "Mining block #2 took {}s",
        duration.as_millis() as f32 / 1000.
    );

    println!("{:?}", blockchain);
    println!("{:#?}", blockchain.check_validity(DIFFICULTY));

    Ok(())
}
