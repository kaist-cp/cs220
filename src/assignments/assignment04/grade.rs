#[cfg(test)]
mod test {
    use crate::assignments::assignment04::syntax::*;
    use crate::assignments::assignment04::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parser::parse_command("$1 = (132 + 77) * 3 ^ 8").unwrap(),
            Command {
                variable: Some("$1".into()),
                expression: Expression::BinOp {
                    op: BinOp::Multiply,
                    lhs: Expression::BinOp {
                        op: BinOp::Add,
                        lhs: Expression::Num(132.0).into(),
                        rhs: Expression::Num(77.0).into(),
                    }
                    .into(),
                    rhs: Expression::BinOp {
                        op: BinOp::Power,
                        lhs: Expression::Num(3.0).into(),
                        rhs: Expression::Num(8.0).into(),
                    }
                    .into(),
                }
            }
        );

        assert_eq!(
            parser::parse_command("132 + 77").unwrap(),
            Command {
                variable: None,
                expression: Expression::BinOp {
                    op: BinOp::Add,
                    lhs: Expression::Num(132.0).into(),
                    rhs: Expression::Num(77.0).into(),
                }
            }
        );

        assert_eq!(
            parser::parse_command("v2 = 132 + 77").unwrap(),
            Command {
                variable: Some("v2".into()),
                expression: Expression::BinOp {
                    op: BinOp::Add,
                    lhs: Expression::Num(132.0).into(),
                    rhs: Expression::Num(77.0).into(),
                }
            }
        );

        assert!(parser::parse_command("132 +!s 77").is_err());

        assert_eq!(
            parser::parse_command("12 - 34 + 23 ^ 4").unwrap(),
            Command {
                variable: None,
                expression: Expression::BinOp {
                    op: BinOp::Add,
                    lhs: Expression::BinOp {
                        op: BinOp::Subtract,
                        lhs: Expression::Num(12.0).into(),
                        rhs: Expression::Num(34.0).into(),
                    }
                    .into(),
                    rhs: Expression::BinOp {
                        op: BinOp::Power,
                        lhs: Expression::Num(23.0).into(),
                        rhs: Expression::Num(4.0).into(),
                    }
                    .into(),
                },
            }
        );
    }

    #[test]
    fn test_context_calc_expression() {
        let ctx = context::Context::new();

        // "(132 + 77) * 3 ^ 8"
        assert_eq!(
            ctx.calc_expression(&Expression::BinOp {
                op: BinOp::Multiply,
                lhs: Expression::BinOp {
                    op: BinOp::Add,
                    lhs: Expression::Num(132.0).into(),
                    rhs: Expression::Num(77.0).into(),
                }
                .into(),
                rhs: Expression::BinOp {
                    op: BinOp::Power,
                    lhs: Expression::Num(3.0).into(),
                    rhs: Expression::Num(8.0).into(),
                }
                .into(),
            })
            .expect("calculate expression is failed"),
            1371249.0
        );

        // "132 + 77"
        assert_eq!(
            ctx.calc_expression(&Expression::BinOp {
                op: BinOp::Add,
                lhs: Expression::Num(132.0).into(),
                rhs: Expression::Num(77.0).into(),
            })
            .expect("calculate expression is failed"),
            209.0
        );

        // "v + 77"
        assert!(ctx
            .calc_expression(&Expression::BinOp {
                op: BinOp::Add,
                lhs: Expression::Variable("v".into()).into(),
                rhs: Expression::Num(77.0).into(),
            })
            .is_err());

        // "3 / (3 * 4 - 12)"
        assert!(ctx
            .calc_expression(&Expression::BinOp {
                op: BinOp::Divide,
                lhs: Expression::Num(3.0).into(),
                rhs: Expression::BinOp {
                    op: BinOp::Subtract,
                    lhs: Expression::BinOp {
                        op: BinOp::Multiply,
                        lhs: Expression::Num(3.0).into(),
                        rhs: Expression::Num(4.0).into(),
                    }
                    .into(),
                    rhs: Expression::Num(12.0).into(),
                }
                .into(),
            })
            .is_err());

        // "12 - 34 + 23 ^ 4"
        assert_eq!(
            ctx.calc_expression(&Expression::BinOp {
                op: BinOp::Add,
                lhs: Expression::BinOp {
                    op: BinOp::Subtract,
                    lhs: Expression::Num(12.0).into(),
                    rhs: Expression::Num(34.0).into(),
                }
                .into(),
                rhs: Expression::BinOp {
                    op: BinOp::Power,
                    lhs: Expression::Num(23.0).into(),
                    rhs: Expression::Num(4.0).into(),
                }
                .into(),
            },)
                .expect("calculate expression is failed"),
            279819.0
        );
    }

    #[test]
    fn test_context_calc_command() {
        let mut ctx = context::Context::new();

        // "v1 = 132 + 77"
        assert_eq!(
            ctx.calc_command(&Command {
                variable: Some("v1".into()),
                expression: Expression::BinOp {
                    op: BinOp::Add,
                    lhs: Expression::Num(132.0).into(),
                    rhs: Expression::Num(77.0).into(),
                }
            })
            .unwrap(),
            ("v1".into(), 209.0)
        );

        // "1 - 3 + 2 ^ 4"
        assert_eq!(
            ctx.calc_command(&Command {
                variable: None,
                expression: Expression::BinOp {
                    op: BinOp::Add,
                    lhs: Expression::BinOp {
                        op: BinOp::Subtract,
                        lhs: Expression::Num(1.0).into(),
                        rhs: Expression::Num(3.0).into(),
                    }
                    .into(),
                    rhs: Expression::BinOp {
                        op: BinOp::Power,
                        lhs: Expression::Num(2.0).into(),
                        rhs: Expression::Num(4.0).into(),
                    }
                    .into(),
                },
            })
            .unwrap(),
            ("$0".into(), 14.0)
        );

        // "v2 = v1 * 3 + $0"
        assert_eq!(
            ctx.calc_command(&Command {
                variable: Some("v2".into()),
                expression: Expression::BinOp {
                    op: BinOp::Add,
                    lhs: Expression::BinOp {
                        op: BinOp::Multiply,
                        lhs: Expression::Variable("v1".into()).into(),
                        rhs: Expression::Num(3.0).into(),
                    }
                    .into(),
                    rhs: Expression::Variable("$0".into()).into()
                }
            })
            .unwrap(),
            ("v2".into(), 641.0)
        );

        // "5 / 2"
        assert_eq!(
            ctx.calc_command(&Command {
                variable: None,
                expression: Expression::BinOp {
                    op: BinOp::Divide,
                    lhs: Expression::Num(5.0).into(),
                    rhs: Expression::Num(2.0).into(),
                },
            })
            .unwrap(),
            ("$1".into(), 2.5)
        );

        // "v2 = v2 ^ 2"
        assert_eq!(
            ctx.calc_command(&Command {
                variable: Some("v2".into()),
                expression: Expression::BinOp {
                    op: BinOp::Power,
                    lhs: Expression::Variable("v2".into()).into(),
                    rhs: Expression::Num(2.0).into(),
                }
            })
            .unwrap(),
            ("v2".into(), 410881.0)
        );

        // "v2 = v2 - $1"
        assert_eq!(
            ctx.calc_command(&Command {
                variable: Some("v2".into()),
                expression: Expression::BinOp {
                    op: BinOp::Subtract,
                    lhs: Expression::Variable("v2".into()).into(),
                    rhs: Expression::Variable("$1".into()).into(),
                }
            })
            .unwrap(),
            ("v2".into(), 410878.5)
        );

        // "v2 = v2 / $3"
        assert!(ctx
            .calc_command(&Command {
                variable: Some("v2".into()),
                expression: Expression::BinOp {
                    op: BinOp::Divide,
                    lhs: Expression::Variable("v2".into()).into(),
                    rhs: Expression::Variable("$3".into()).into(),
                }
            })
            .is_err());

        // "v3 = 3 / (3 * 4 - 12)"
        assert!(ctx
            .calc_command(&Command {
                variable: Some("v3".into()),
                expression: Expression::BinOp {
                    op: BinOp::Divide,
                    lhs: Expression::Num(3.0).into(),
                    rhs: Expression::BinOp {
                        op: BinOp::Subtract,
                        lhs: Expression::BinOp {
                            op: BinOp::Multiply,
                            lhs: Expression::Num(3.0).into(),
                            rhs: Expression::Num(4.0).into(),
                        }
                        .into(),
                        rhs: Expression::Num(12.0).into(),
                    }
                    .into(),
                }
            })
            .is_err());

        // v3 = v3 - v2
        assert!(ctx
            .calc_command(&Command {
                variable: Some("v3".into()),
                expression: Expression::BinOp {
                    op: BinOp::Subtract,
                    lhs: Expression::Variable("v3".into()).into(),
                    rhs: Expression::Variable("v2".into()).into(),
                }
            })
            .is_err());
    }
}
