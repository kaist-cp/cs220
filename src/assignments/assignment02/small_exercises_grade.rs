#[cfg(test)]
mod test {
    use crate::assignments::assignment02::small_exercises::*;

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
}
