//! Generators

enum Yielded<T> {
    Value(T),
    Stop,
}

/// Generator
///
/// Reference:
/// - [Python generator](https://python-reference.readthedocs.io/en/latest/docs/generator/)
#[allow(missing_debug_implementations)]
pub struct Generator<T, S> {
    state: S,
    f: fn(&mut S) -> Yielded<T>,
}

impl<T, S> Iterator for Generator<T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Returns a generator that yields fibonacci numbers.
pub fn fib_generator(first: usize, second: usize) -> Generator<usize, (usize, usize)> {
    todo!()
}

/// Returns a generator that yields collatz numbers.
///
/// The generator stops when it reaches to 1.
pub fn collatz_conjecture(start: usize) -> Generator<usize, usize> {
    todo!()
}
