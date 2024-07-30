//! Test cases for assignment12/card.rs

#[cfg(test)]
mod test_card {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;
    use std::time::Duration;

    use crate::assignments::assignment12::card::*;

    const NUM_CARDS: usize = 10000;
    const DURATION: u64 = 20;
    const NUM_ENEMIES: usize = 25;

    #[derive(Clone, Debug)]
    struct Card {
        color: Arc<Mutex<Color>>,
    }

    impl Card {
        fn new() -> Self {
            Card {
                color: Arc::new(Mutex::new(Color::Blue)),
            }
        }

        fn flip(&self, new_color: Color) {
            let mut color = self.color.lock().unwrap();
            *color = new_color;
        }

        fn get_color(&self) -> Color {
            let color = self.color.lock().unwrap();
            *color
        }
    }

    #[derive(Clone, Debug)]
    struct Ground {
        cards: Vec<Card>,
    }

    impl Ground {
        fn new() -> Self {
            let cards: Vec<_> = (0..NUM_CARDS).map(|_| Card::new()).collect();
            Ground { cards }
        }

        fn flip_card(&self, idx: usize, color: Color) {
            self.cards[idx % NUM_CARDS].flip(color);
        }

        fn get_card_color(&self, idx: usize) -> Color {
            self.cards[idx % NUM_CARDS].get_color()
        }
    }

    #[test]
    fn play() {
        let ground = Ground::new();
        let barrier = Arc::new(Barrier::new(
            NUM_ENEMIES + 1 /*Player*/ + 1, /* Referee */
        ));
        let playing = Arc::new(AtomicBool::new(true));

        // Create a thread for the student's strategy
        let mut player = Player::new();
        let player_thread = {
            let ground = ground.clone();
            let barrier = barrier.clone();
            let playing = playing.clone();

            // The player's strategy thread
            thread::spawn(move || {
                // Get, Set, Ready, Go!
                let _ = barrier.wait();

                // As long as the game is still playing...
                while playing.load(Ordering::SeqCst) {
                    let (idx, color) = player.flip_card_strategy();
                    ground.flip_card(idx, color);
                }
            })
        };

        // Create multiple threads for the computer's strategy
        let dist = NUM_CARDS / NUM_ENEMIES;
        let bot_threads: Vec<_> = (0..NUM_ENEMIES)
            .map(|i| {
                let ground = ground.clone();
                let barrier = barrier.clone();
                let playing = playing.clone();

                let init = i * dist;
                let mut cnt = 0;

                thread::spawn(move || {
                    // Get, Set, Ready, Go!
                    let _ = barrier.wait();

                    // As long as the game is still playing...
                    while playing.load(Ordering::SeqCst) {
                        let idx = init + (cnt % dist);
                        match ground.get_card_color(idx) {
                            Color::White => ground.flip_card(idx, Color::Blue),
                            Color::Blue => thread::sleep(Duration::from_micros(10)),
                        };
                        cnt += 1;
                    }
                })
            })
            .collect();

        // Get, Set, Ready, Go!
        let _ = barrier.wait();

        // Wait for a while and stop the game
        thread::sleep(Duration::from_secs(DURATION));
        playing.store(false, Ordering::SeqCst);

        // Wait for all threads to finish
        player_thread.join().unwrap();
        for bot_thread in bot_threads {
            bot_thread.join().unwrap();
        }

        // Count the number of white and blue cards
        let mut white_cnt = 0;
        let mut blue_cnt = 0;
        for card in ground.cards {
            match card.get_color() {
                Color::White => white_cnt += 1,
                Color::Blue => blue_cnt += 1,
            }
        }

        // Print the winner
        println!("[White: {white_cnt}, Blue: {blue_cnt}]");
        assert!(white_cnt > blue_cnt, "You lose...",);
    }
}
