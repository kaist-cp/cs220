#[cfg(test)]
mod test {
    use super::super::assignment08::*;

    #[test]
    fn test_repeat() {
        for i in 0..10 {
            assert_eq!(42 + 2 * i, repeat(i, |x| x + 2)(42));
        }
    }

    #[test]
    fn test_funny_map() {
        assert_eq!(
            vec![0, 3, 6, 9, 12],
            funny_map(|x| x + 2, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_either2_map() {
        let v1 = Either2::<u32, f32>::Case1 { inner: 42 };
        let u1 = Either2::<u32, f32>::Case1 { inner: 43 };
        assert_eq!(u1, v1.map(|i| i + 1, |f| f + 1.0));

        let v2 = Either2::<u32, f32>::Case2 { inner: 42.0 };
        let u2 = Either2::<u32, f32>::Case2 { inner: 43.0 };
        assert_eq!(u2, v2.map(|i| i + 1, |f| f + 1.0));
    }

    #[test]
    fn test_fsm_1d() {
        #[derive(Debug, Clone, Copy)]
        enum Cmd {
            Inc,
            Dec,
        }

        let f = |s: &mut isize, input: Cmd| match input {
            Cmd::Inc => *s += 1,
            Cmd::Dec => *s -= 1,
        };

        assert_eq!(fsm(f)(5, []), 5);
        assert_eq!(fsm(f)(-2, []), -2);
        assert_eq!(fsm(f)(0, []), 0);

        let inc_inc_dec_inc = [Cmd::Inc, Cmd::Inc, Cmd::Dec, Cmd::Inc];
        let dec_inc_dec = [Cmd::Dec, Cmd::Inc, Cmd::Dec];

        for i in -10..10 {
            for j in 0..10 {
                assert_eq!(fsm(f)(i, inc_inc_dec_inc.repeat(j as usize)), i + 2 * j);
                assert_eq!(fsm(f)(i, dec_inc_dec.repeat(j as usize)), i - j);
            }
        }
    }

    #[test]
    fn test_fsm_2d() {
        #[derive(Debug, Clone, Copy)]
        enum Cmd {
            Up,
            Down,
            Right,
            Left,
        }

        let f = |s: &mut (isize, isize), input: Cmd| match input {
            Cmd::Up => s.1 += 1,
            Cmd::Down => s.1 -= 1,
            Cmd::Right => s.0 += 1,
            Cmd::Left => s.0 -= 1,
        };

        assert_eq!(fsm(f)((0, 0), []), (0, 0));
        assert_eq!(fsm(f)((5, -2), []), (5, -2));

        let up_up_right = [Cmd::Up, Cmd::Up, Cmd::Right];

        assert_eq!(fsm(f)((2, -1), up_up_right), (3, 1));
    }
}
