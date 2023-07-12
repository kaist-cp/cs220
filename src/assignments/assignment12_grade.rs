#[cfg(test)]
mod test {
    use super::super::assignment12::*;
    use ntest::timeout;

    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn test_ping_pong() {
        let (tx1, mut rx1) = channel();
        let (mut tx2, rx2) = channel();

        let thread_ping = thread::spawn(move || {
            for i in 0..100 {
                tx1.send(i).unwrap();
                let x = rx2.recv().unwrap();
                assert_eq!(x, i + 1);
            }
        });

        let thread_pong = thread::spawn(move || while pong(&mut rx1, &mut tx2) {});

        thread_ping.join().unwrap();
        thread_pong.join().unwrap();
    }

    #[test]
    fn test_scoped_thread() {
        for i in 0..100 {
            let v = (0..i).collect::<Vec<u32>>();

            thread::scope(|s| {
                let (r1, r2) = use_scoped_thread(
                    s,
                    || v.iter().sum::<u32>(),
                    || v.windows(2).map(|x| x[0] * x[1]).sum::<u32>(),
                );

                assert_eq!(r1, v.iter().sum());
                assert_eq!(r2, v.windows(2).map(|x| x[0] * x[1]).sum());
            });
        }
    }

    #[test]
    #[timeout(5000)]
    fn test_scoped_thread_concurrent() {
        use std::sync::Mutex;

        let m = Mutex::new(0);
        let (r1, r2) = thread::scope(|s| {
            use_scoped_thread(
                s,
                || {
                    for i in 0..100 {
                        loop {
                            let mut a = m.lock().unwrap();
                            if *a == 2 * i {
                                *a += 1;
                                break;
                            }
                        }
                    }
                    thread::current().id()
                },
                || {
                    for i in 0..100 {
                        loop {
                            let mut a = m.lock().unwrap();
                            if *a == 2 * i + 1 {
                                *a += 1;
                                break;
                            }
                        }
                    }
                    thread::current().id()
                },
            )
        });

        assert!(r1 != r2);
    }

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
