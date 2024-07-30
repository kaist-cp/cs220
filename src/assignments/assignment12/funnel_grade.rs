//! Test cases for assignment12/funnel.rs

#[cfg(test)]
mod test_funnel {
    use std::sync::mpsc::channel;
    use std::thread;

    use ntest::timeout;

    use crate::assignments::assignment12::funnel::*;

    #[test]
    #[timeout(5000)]
    fn test_funnel_concurrent() {
        let (txs, rxs): (Vec<_>, Vec<_>) = (0..10).map(|_| channel::<u32>()).unzip();
        let (tx, rx) = channel::<u32>();
        let filter = |x: &u32| x % 2 == 0;

        let thread_txs_rx = thread::spawn(move || {
            for i in 0..100 {
                let idx = (i * 7) % 13 * 17 % 10;
                txs[idx].send(i as u32).unwrap();
                if i % 2 == 0 {
                    let x = rx.recv().unwrap();
                    assert_eq!(x, i as u32);
                }
            }
        });
        let thread_funnel = spawn_funnel(rxs, tx, filter);

        thread_txs_rx.join().unwrap();
        thread_funnel.join().unwrap();
    }
}
