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
        assert_eq!(capitalize(String::from("Tschüß")), String::from("TSCHüß"));
    }

    #[test]
    fn test_up3() {
        assert_eq!(up3(0), 1);
        assert_eq!(up3(1), 1);
        assert_eq!(up3(6), 9);
        assert_eq!(up3(9), 9);
        assert_eq!(up3(10), 27);
        assert_eq!(up3(1_000_000), 1_594_323);
        assert_eq!(up3(3u64.pow(39).wrapping_add(1)), 3u64.pow(40));
        assert_eq!(up3(3u64.pow(40)), 3u64.pow(40));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(5, 1), 1);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(2, 6), 2);
        assert_eq!(gcd(24, 18), 6);
        assert_eq!(gcd(20, 63), 1);
        assert_eq!(gcd(0, 33), 33);
    }

    #[test]
    fn test_sum_array() {
        assert_eq!(sum_array(&[]), 0);
        assert_eq!(sum_array(&[1]), 1);
        assert_eq!(sum_array(&[1, 2, 3, 4, 5, 100]), 115);
    }

    #[test]
    fn test_chooses() {
        assert_eq!(chooses(0), vec![1]);
        assert_eq!(chooses(1), vec![1, 1]);
        assert_eq!(chooses(5), vec![1, 5, 10, 10, 5, 1]);
        assert_eq!(chooses(6), vec![1, 6, 15, 20, 15, 6, 1]);
        assert_eq!(
            chooses(67),
            vec![
                1,
                67,
                2211,
                47905,
                766480,
                9657648,
                99795696,
                869648208,
                6522361560,
                42757703560,
                247994680648,
                1285063345176,
                5996962277488,
                25371763481680,
                97862516286480,
                345780890878896,
                1123787895356412,
                3371363686069236,
                9364899127970100,
                24151581961607100,
                57963796707857040,
                129728497393775280,
                271250494550621040,
                530707489338171600,
                972963730453314600,
                1673497616379701112,
                2703342303382594104,
                4105075349580976232,
                5864393356544251760,
                7886597962249166160,
                9989690752182277136,
                11923179284862717872,
                13413576695470557606,
                14226520737620288370,
                14226520737620288370,
                13413576695470557606,
                11923179284862717872,
                9989690752182277136,
                7886597962249166160,
                5864393356544251760,
                4105075349580976232,
                2703342303382594104,
                1673497616379701112,
                972963730453314600,
                530707489338171600,
                271250494550621040,
                129728497393775280,
                57963796707857040,
                24151581961607100,
                9364899127970100,
                3371363686069236,
                1123787895356412,
                345780890878896,
                97862516286480,
                25371763481680,
                5996962277488,
                1285063345176,
                247994680648,
                42757703560,
                6522361560,
                869648208,
                99795696,
                9657648,
                766480,
                47905,
                2211,
                67,
                1
            ]
        );
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
        assert_eq!(fibonacci(92), 12200160415121876738);
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
