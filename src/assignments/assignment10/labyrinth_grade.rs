#[cfg(test)]
mod test {

    use rand::seq::SliceRandom;
    use rand::thread_rng;

    use crate::assignments::assignment10::labyrinth::*;

    type Wife = usize;
    type Rooms = Vec<Wife>;

    struct Labyrinth {
        rooms: Rooms,
    }

    impl From<Rooms> for Labyrinth {
        fn from(rooms: Rooms) -> Self {
            Self { rooms }
        }
    }

    impl Labyrinth {
        fn open_the_door(&self, index: usize) -> Wife {
            self.rooms[index]
        }
    }

    fn can_every_husband_rescue_his_wife() -> bool {
        // HINT: https://en.wikipedia.org/wiki/100_prisoners_problem
        const WIVES: usize = 100;

        // One day, wives of 100 husbands were kidnapped by the Minotaur
        // and imprisoned in a labyrinth.... 🏰
        let labyrinth = Labyrinth::from({
            let mut rooms: Vec<_> = (0..WIVES).collect();
            rooms.shuffle(&mut thread_rng());
            rooms
        });

        (0..WIVES).all(|his_wife| {
            // A new husband steps into the labyrinth to rescue his wife...!
            let husband = Box::new(Husband::seeking(his_wife /* 👩 */));
            let strategy = Box::new(husband.has_devised_a_strategy());

            // (Allow for better storytelling.)
            #[allow(clippy::search_is_some)]
            // The Minotaur🐂 will arrive in
            (0..50) // steps...
                .zip(strategy)
                .find(|(_, room)| {
                    // The husband contemplates his next move... 🤔
                    // and finally,
                    let someone/*👤*/ = labyrinth.open_the_door(*room); // 🚪
                    husband.carefully_checks_whos_inside(*room, someone);

                    // Has the husband found his wife...?
                    someone/*👤*/ == his_wife /*👩*/
                })
                .is_some(/* The husband has successfully rescued his wife! 👫*/)
            // or is_none(/* The unfortunate husband has encountered the Minotaur and... 🪓*/)
        })
    }

    #[test]
    fn main() {
        let mut num_success = 0;
        for _ in 0..10000 {
            if can_every_husband_rescue_his_wife() {
                num_success += 1
            }
        }

        assert!(num_success > 3000)
    }
}
