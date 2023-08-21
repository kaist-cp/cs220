//! Test cases for assignment11/tv_room.rs

#[cfg(test)]
mod test_tv_room {
    use crate::assignments::assignment11::tv_room::*;

    #[test]
    fn test_tv_room() {
        let tv_room = TVRoom::new();
        assert!(!tv_room.is_opened());

        // Turn on and add new guests.
        let manager = tv_room.open().unwrap();
        assert!(tv_room.is_opened());
        let guest1 = manager.new_guest();
        let guest2 = manager.new_guest();
        drop(manager);
        drop(guest1);
        assert!(tv_room.open().is_none());
        drop(guest2);
        assert!(!tv_room.is_opened());

        // Turn on and add new guests.
        let manager = tv_room.open().unwrap();
        assert!(tv_room.is_opened());
        let guest3 = manager.new_guest();
        drop(guest3);
        assert!(tv_room.is_opened());
        drop(manager);
        assert!(!tv_room.is_opened());
    }
}
