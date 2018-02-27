use std::str::FromStr;

use nom::digit;

use c::expressions::{Expression,UnaryOperator};

named!(pub parse_expression<&str, Expression>,
    alt!(
        parse_int_literal
        | parse_unary_operation
    )
);

named!(parse_unary_operation<&str, Expression>,
    alt!(
        parse_negation
        | parse_local_negation
        | parse_bitwise
    )
);

named!(parse_negation<&str, Expression>,
    ws!(
        do_parse!(
            char!('-') >>
            expr: parse_expression >>
            (Expression::Unary(UnaryOperator::Negation, Box::from(expr)))
        )
    )
);

named!(parse_local_negation<&str, Expression>,
    ws!(
        do_parse!(
            char!('!') >>
            expr: parse_expression >>
            (Expression::Unary(UnaryOperator::LocalNegation, Box::from(expr)))
        )
    )
);

named!(parse_bitwise<&str, Expression>,
    ws!(
        do_parse!(
            char!('~') >>
            expr: parse_expression >>
            (Expression::Unary(UnaryOperator::Bitwise, Box::from(expr)))
        )
    )
);

named!(parse_int_literal<&str, Expression>, map!(map_res!(digit, i32::from_str), Expression::from));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;
    use c::Expression::*;

    #[test]
    fn test_parse_simple_expression() {
        assert_eq!(parse_expression("42"), Done("", Literal(42)));
    }

    #[test]
    fn test_parse_unary_operator() {
        assert_eq!(parse_expression("!42"),
                   Done("",
                        Unary(UnaryOperator::LocalNegation,
                                          Box::new(Literal(42)))));
        assert_eq!(parse_expression("!!42"),
                   Done("",
                        Unary(UnaryOperator::LocalNegation,
                                          Box::new(Unary(UnaryOperator::LocalNegation,Box::new(Literal(42)))))));
        assert_eq!(parse_expression("~42"),
                   Done("",
                        Unary(UnaryOperator::Bitwise,
                                     Box::new(Literal(42)))));
        assert_eq!(parse_expression("-42"),
                   Done("",
                        Unary(UnaryOperator::Negation,
                                          Box::new(Literal(42)))));
    }

    #[test]
    fn test_parse_int_literal() {
        assert_eq!(parse_int_literal("42"), Done("", Literal(42)));
    }
}
