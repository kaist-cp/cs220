//! Test cases for assignment12/demux.rs

#[cfg(test)]
mod test_demux {
    use std::sync::mpsc::channel;
    use std::thread;

    use ntest::timeout;

    use crate::assignments::assignment12::demux::*;

    #[test]
    #[timeout(5000)]
    fn test_demux() {
        let (tx, rx1, rx2) = demux::<u32, _>(|x| x % 2 == 0);

        let thread_tx = thread::spawn(move || {
            for i in 0..100 {
                tx.send(i).unwrap();
            }
        });

        let thread_rx1 = thread::spawn(move || {
            let sum: u32 = rx1.iter().sum();
            assert_eq!(sum, (0..100).filter(|x| x % 2 == 0).sum());
        });

        let thread_rx2 = thread::spawn(move || {
            let sum: u32 = rx2.iter().sum();
            assert_eq!(sum, (0..100).filter(|x| x % 2 != 0).sum());
        });

        thread_tx.join().unwrap();
        thread_rx1.join().unwrap();
        thread_rx2.join().unwrap();
    }
}
