fn parse_input_top_level(input: &str) -> u64 {
    let mut iter = input.chars();
    evaluate_operands(&mut iter)
}

// input: lines of expressions
pub fn sum_lines(input: &str) -> u64 {
    input.lines().map(parse_input_top_level).sum()
}

fn evaluate_operands(mut iter: &mut std::str::Chars<'_>) -> u64 {
    let mut last_expr: u64 = 0;
    while let Some(c) = iter.next() {
        if c.is_digit(10) {
            last_expr = c.to_digit(10).unwrap().into();
        } else if c == '(' {
            last_expr = evaluate_paren_expression(&mut iter)
        } else if c == ')' || c == ' ' {
            continue;
        } else if c == '+' {
            last_expr += evaluate(last_expr, &mut iter);
        } else if c == '*' {
            last_expr *= evaluate(last_expr, &mut iter);
        } else {
            panic!("invalid expression, or bug in the parser (prob. bug in parser): c: {}", c)
        }
    }
    last_expr
}


fn evaluate(last: u64, mut iter: &mut std::str::Chars<'_>) -> u64 {
    let mut last_expr: u64 = last;
    while let Some(c) = iter.next() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap().into();
        } else if c == '(' {
            last_expr = evaluate_paren_expression(&mut iter)
        } else if c == ')' {
            break;
        } else if c == ' ' {
            continue;
        } else if c == '+' {
            last_expr += evaluate(last_expr, &mut iter);
        } else if c == '*' {
            last_expr *= evaluate(last_expr, &mut iter);
        } else {
            panic!("invalid expression, or bug in the parser (prob. bug in parser): c: {}", c);
        }
    }
    last_expr
}

fn evaluate_paren_expression(mut iter: &mut std::str::Chars<'_>) -> u64 {
    evaluate(0, &mut iter)
}

#[test]
fn just_two_operands() {
    let input = "1 + 2";
    assert_eq!(3, parse_input_top_level(&input));
}
#[test]
fn evaluate_first_example() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(71, parse_input_top_level(&input))
}
#[test]
fn evaluate_second_example() {
    let input = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(51, parse_input_top_level(&input));
}
#[test]
fn evaluate_third_example() {
    let input = "2 * 3 + (4 * 5)";
    assert_eq!(26, parse_input_top_level(&input));
}
#[test]
fn evaluate_fourth_example() {
    let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(437, parse_input_top_level(&input));
}
#[test]
fn evaluate_fifth_example() {
    let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(12240, parse_input_top_level(&input));
}
#[test]
fn evaluate_sixth_example() {
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(13632, parse_input_top_level(&input));
}
#[test]
fn sum_lines_three_lines() {
    let input = "1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(514, sum_lines(&input));
}
