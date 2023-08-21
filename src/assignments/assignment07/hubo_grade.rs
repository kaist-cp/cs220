#[cfg(test)]
mod test {
    use itertools::Itertools;
    use ntest::assert_about_eq;

    use super::super::hubo::*;

    #[test]
    fn test_hubo_dir4_movement() {
        let mut hubo = Hubo::new(Dir4::Right, 0.0, 0.0);
        let mut controller = HuboController::new(&mut hubo);

        // Test moving forward
        controller.move_hubo_forward(5.0);

        controller.set_hubo_direction(Dir4::Up);
        controller.move_hubo_forward(3.0);

        controller.set_hubo_direction(Dir4::Left);
        controller.move_hubo_forward(2.0);

        assert_eq!(hubo.get_position(), (3.0, 3.0));
    }

    #[test]
    fn test_hubo_tuple_movement() {
        let mut hubo = Hubo::new((1., 0.), 0.0, 0.0);
        let mut controller = HuboController::new(&mut hubo);

        // Test moving forward
        controller.move_hubo_forward(5.0);

        controller.set_hubo_direction((3., 4.));
        controller.move_hubo_forward(5.0);

        controller.set_hubo_direction((-8., -6.));
        controller.move_hubo_forward(15.0);
        assert_eq!(hubo.get_position(), (-4., -5.));
    }
}
