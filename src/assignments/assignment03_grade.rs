#[cfg(test)]
mod test {
    use super::super::assignment03::*;

    #[test]
    fn test_next_weekday() {
        assert_eq!(next_weekday(DayOfWeek::Sun), DayOfWeek::Mon);
        assert_eq!(next_weekday(DayOfWeek::Mon), DayOfWeek::Tue);
        assert_eq!(next_weekday(DayOfWeek::Tue), DayOfWeek::Wed);
        assert_eq!(next_weekday(DayOfWeek::Wed), DayOfWeek::Thu);
        assert_eq!(next_weekday(DayOfWeek::Thu), DayOfWeek::Fri);
        assert_eq!(next_weekday(DayOfWeek::Fri), DayOfWeek::Mon);
        assert_eq!(next_weekday(DayOfWeek::Sat), DayOfWeek::Mon);
    }

    #[test]
    fn test_my_map() {
        use MyOption::*;

        fn len(s: &str) -> usize {
            s.len()
        }

        fn plus_one(x: isize) -> isize {
            x + 1
        }

        fn is_positive(x: f64) -> bool {
            x > 0.0f64
        }

        assert_eq!(my_map(MySome("Hello, World!"), len), MySome(13));
        assert_eq!(my_map(MyNone, len), MyNone);

        assert_eq!(my_map(MySome(1), plus_one), MySome(2));
        assert_eq!(my_map(MyNone, plus_one), MyNone);

        assert_eq!(my_map(MySome(5.0f64), is_positive), MySome(true));
        assert_eq!(my_map(MySome(-3.0f64), is_positive), MySome(false));
        assert_eq!(my_map(MyNone::<f64>, is_positive), MyNone);
    }

    #[test]
    fn test_my_and_then() {
        use MyOption::*;

        fn plus_one(x: isize) -> MyOption<isize> {
            MySome(x + 1)
        }

        fn none(_: isize) -> MyOption<isize> {
            MyNone
        }

        assert_eq!(my_and_then(MySome(1), plus_one), MySome(2));
        assert_eq!(my_and_then(MySome(1), none), MyNone);

        assert_eq!(my_and_then(MyNone, plus_one), MyNone);
        assert_eq!(my_and_then(MyNone, none), MyNone);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(vec![]), None);
        assert_eq!(median(vec![1]), Some(1));
        assert_eq!(median(vec![1, 2]), Some(2));
        assert_eq!(median(vec![2, 4, 5, 1, 3]), Some(3));
        assert_eq!(median(vec![2, 3, 5, 7, 11, 13]), Some(7));
        assert_eq!(median(vec![1, 3, 3, 6, 7, 8, 9]), Some(6));
        assert_eq!(median(vec![6, 7, 3, 1, 9, 3, 8]), Some(6));
        assert_eq!(median(vec![1, 2, 3, 4, 5, 6, 8, 9]), Some(5));
        assert_eq!(median(vec![3, 4, 8, 9, 1, 6, 5, 2]), Some(5));
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(vec![]), None);
        assert_eq!(mode(vec![3]), Some(3));
        assert_eq!(mode(vec![2, 1, 2, 3]), Some(2));
        assert_eq!(mode(vec![2, 3, 1, 2, 2, 3, 3]), Some(2));
        assert_eq!(mode(vec![1, 1, 2, 2, 3, 3]), Some(1));
    }

    #[test]
    fn test_piglatin() {
        assert_eq!(piglatin("pig".to_string()), "igpay".to_string());
        assert_eq!(piglatin("latin".to_string()), "atinlay".to_string());
        assert_eq!(piglatin("banana".to_string()), "ananabay".to_string());
        assert_eq!(piglatin("will".to_string()), "illway".to_string());
        assert_eq!(piglatin("butler".to_string()), "utlerbay".to_string());
        assert_eq!(piglatin("happy".to_string()), "appyhay".to_string());
        assert_eq!(piglatin("duck".to_string()), "uckday".to_string());
        assert_eq!(piglatin("me".to_string()), "emay".to_string());
        assert_eq!(piglatin("bagel".to_string()), "agelbay".to_string());
        assert_eq!(piglatin("history".to_string()), "istoryhay".to_string());

        assert_eq!(piglatin("smile".to_string()), "ilesmay".to_string());
        assert_eq!(piglatin("string".to_string()), "ingstray".to_string());
        assert_eq!(piglatin("stupid".to_string()), "upidstay".to_string());
        assert_eq!(piglatin("glove".to_string()), "oveglay".to_string());
        assert_eq!(piglatin("trash".to_string()), "ashtray".to_string());
        assert_eq!(piglatin("floor".to_string()), "oorflay".to_string());
        assert_eq!(piglatin("store".to_string()), "orestay".to_string());

        assert_eq!(piglatin("eat".to_string()), "eathay".to_string());
        assert_eq!(piglatin("omelet".to_string()), "omelethay".to_string());
        assert_eq!(piglatin("are".to_string()), "arehay".to_string());
        assert_eq!(piglatin("egg".to_string()), "egghay".to_string());
        assert_eq!(piglatin("explain".to_string()), "explainhay".to_string());
        assert_eq!(piglatin("ends".to_string()), "endshay".to_string());
        assert_eq!(piglatin("amulet".to_string()), "amulethay".to_string());
    }

    #[test]
    fn test_organize() {
        assert_eq!(
            organize(vec![
                "Add Amir to Engineering".to_string(),
                "Add Sally to Sales".to_string(),
                "Remove Jeehoon from Sales".to_string(),
                "Move Amir from Engineering to Sales".to_string(),
            ]),
            [(
                "Sales".to_string(),
                ["Amir".to_string(), "Sally".to_string()].into()
            )]
            .into()
        );

        assert_eq!(
            organize(vec![
                "Add Jeehoon to Mathematics".to_string(),
                "Add Minseong to Mathematics".to_string(),
                "Add Seungmin to Computer-Science".to_string(),
                "Move Jeehoon from Mathematics to Computer-Science".to_string(),
                "Remove Minseong from Mathematics".to_string(),
                "Add Minseong to Computer-Science".to_string(),
            ]),
            [(
                "Computer-Science".to_string(),
                [
                    "Seungmin".to_string(),
                    "Jeehoon".to_string(),
                    "Minseong".to_string()
                ]
                .into()
            )]
            .into()
        );

        assert_eq!(
            organize(vec![
                "Move P1 from D1 to D2".to_string(),
                "Remove P2 from D2".to_string(),
                "Add P3 to D3".to_string(),
                "Add P4 to D1".to_string(),
                "Add P3 to D4".to_string(),
                "Move P3 from D4 to D2".to_string(),
            ]),
            [
                ("D1".to_string(), ["P4".to_string()].into()),
                ("D2".to_string(), ["P3".to_string()].into()),
                ("D3".to_string(), ["P3".to_string()].into())
            ]
            .into()
        );
    }
}
