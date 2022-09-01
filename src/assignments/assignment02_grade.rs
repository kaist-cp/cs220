#[cfg(test)]
mod test {
    use super::super::assignment02::*;

    #[test]
    fn test_fahrenheit() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(
            capitalize(String::from("aAbbBcccCddddD❤한글과✓")),
            String::from("AABBBCCCCDDDDD❤한글과✓"),
        );
    }

    #[test]
    fn test_up3() {
        assert_eq!(up3(1), 1);
        assert_eq!(up3(6), 9);
        assert_eq!(up3(9), 9);
        assert_eq!(up3(10), 27);
        assert_eq!(up3(1_000_000), 1_594_323);
    }

    #[test]
    fn test_sum_array() {
        assert_eq!(sum_array(&[1, 2, 3, 4, 5, 100]), 115);
    }

    #[test]
    fn test_chooses() {
        assert_eq!(chooses(0), vec![1]);
        assert_eq!(chooses(1), vec![1, 1]);
        assert_eq!(chooses(5), vec![1, 5, 10, 10, 5, 1]);
        assert_eq!(chooses(6), vec![1, 6, 15, 20, 15, 6, 1]);
    }

    #[test]
    fn test_zip() {
        assert_eq!(zip(vec![1, 2], vec![4, 5]), vec![(1, 4), (2, 5)]);
        assert_eq!(zip(vec![1, 2, 3], vec![4, 5]), vec![(1, 4), (2, 5)]);
        assert_eq!(zip(vec![1, 2], vec![4, 5, 6]), vec![(1, 4), (2, 5)]);
        assert_eq!(zip(vec![], vec![4, 5]), vec![]);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(5), 8);
        assert_eq!(fibonacci(6), 13);
        assert_eq!(fibonacci(7), 21);
        assert_eq!(fibonacci(50), 20365011074);
    }

    #[test]
    fn test_lyrics() {
        assert_eq!(twelve_days_of_christmas_lyrics(), LYRICS)
    }

    const LYRICS: &str = r#"On the first day of Christmas, my true love sent to me
A partridge in a pear tree

On the second day of Christmas, my true love sent to me
Two turtledoves
And a partridge in a pear tree

On the third day of Christmas, my true love sent to me
Three French hens
Two turtledoves
And a partridge in a pear tree

On the fourth day of Christmas, my true love sent to me
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the fifth day of Christmas, my true love sent to me
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the sixth day of Christmas, my true love sent to me
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the seventh day of Christmas, my true love sent to me
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the eighth day of Christmas, my true love sent to me
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the ninth day of Christmas, my true love sent to me
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the tenth day of Christmas, my true love sent to me
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the eleventh day of Christmas, my true love sent to me
I sent eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

On the twelfth day of Christmas, my true love sent to me
Twelve drummers drumming
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five gold rings (five golden rings)
Four calling birds
Three French hens
Two turtledoves
And a partridge in a pear tree

And a partridge in a pear tree
"#;
}
