//! TV Room Simulator.
//!
//! People can come to the TV room and watch TV. There are two types of TV watchers, manager and
//! guest.
//!
//! The rule of the TV room is as follows:
//!
//! - Closed TV room can be opened by the manager.
//! - Guests can enter the TV room by the manager.
//! - Manager can leave the TV room earlier than guests.
//! - The TV room closes when the last person left the TV room.
//!
//! Both `Manager` and `Guest` have `Rc<Watcher>` as a field, and its reference count indicates the
//! number of people in the TV room. When the 'Manager' and 'Guest' object is dropped, it means that
//! the person leaves the TV room.
//!
//! Consult the following documentations:
//! - <https://doc.rust-lang.org/book/ch15-04-rc.html#rct-the-reference-counted-smart-pointer>
//! - <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt>
//!
//! Refer `tv_room_grade.rs` for test cases.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
enum TVRoomState {
    Opened,
    Closed,
}

/// TV Room
#[derive(Debug)]
pub struct TVRoom {
    /// Indicates whether the TV room is state.
    state: RefCell<TVRoomState>,
}

impl Default for TVRoom {
    fn default() -> Self {
        Self::new()
    }
}

impl TVRoom {
    /// Creates a new TV room.
    ///
    /// Initial state of the TV room is closed.
    pub fn new() -> Self {
        Self {
            state: RefCell::new(TVRoomState::Closed),
        }
    }

    /// Opens the TV room and returns the manager.
    ///
    /// Returns `None` if the TV room is already opened.
    pub fn open(&self) -> Option<Manager<'_>> {
        todo!()
    }

    /// Returns whether the TV room is opened or not.
    pub fn is_opened(&self) -> bool {
        todo!()
    }
}

/// TV Room Manager.
///
/// - The manager is special TV's watcher that has privileges to add other guests.
/// - If all watchers including the manager drop (~= leave the TV room), the TV must be turned off.
/// - Note that the manager can be dropped while other watchers are watching TV.
#[derive(Debug)]
pub struct Manager<'a> {
    inner: Rc<Watcher<'a>>,
}

impl<'a> Manager<'a> {
    fn new(tvstate: &'a RefCell<TVRoomState>) -> Self {
        Self {
            inner: Rc::new(Watcher::new(tvstate)),
        }
    }

    /// Adds new guest to the TV room.
    pub fn new_guest(&self) -> Guest<'a> {
        todo!()
    }
}

/// TV Room Guest.
#[derive(Debug)]
pub struct Guest<'a> {
    inner: Rc<Watcher<'a>>,
}

#[derive(Debug)]
struct Watcher<'a> {
    tvstate: &'a RefCell<TVRoomState>,
}

impl<'a> Watcher<'a> {
    fn new(tvstate: &'a RefCell<TVRoomState>) -> Self {
        Self { tvstate }
    }
}

impl Drop for Watcher<'_> {
    fn drop(&mut self) {
        // When the last person leaves the TV room, the TV room should be closed.
        todo!()
    }
}
