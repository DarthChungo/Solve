pub fn find_closing_parenthesis(expr: &str) -> Option<usize> {
    let mut par_level: isize = 0;

    for (i, c) in expr.chars().enumerate() {
        par_level += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if par_level == 0 {
            return Some(i);
        }
    }

    None
}

pub fn is_function_call(expr: &str) -> Option<(&str, &str, usize)> {
    for (i, c) in expr.chars().enumerate() {
        if c == '(' && i != 0 {
            if let Some(j) = find_closing_parenthesis(&expr[i..]) {
                return if i + j == expr.len() - 1 {
                    Some((&expr[..i], &expr[i + 1..expr.len() - 1], i))
                } else {
                    None
                };
            }
        } else if !c.is_alphabetic() {
            return None;
        }
    }

    None
}

pub fn find_nth_comma(expr: &str, n: usize) -> Option<usize> {
    let mut i: usize = 0;
    let mut c: usize = 0;

    if expr.is_empty() {
        return None;
    }

    loop {
        match expr.chars().nth(i).unwrap() {
            ',' => {
                c += 1;
            }
            '(' => {
                if let Some(j) = find_closing_parenthesis(&expr[i..]) {
                    i += j;
                }
            }
            _ => {}
        };

        if n == c {
            return Some(i);
        }

        if i == expr.len() - 1 {
            break;
        } else {
            i += 1;
        }
    }

    None
}

pub fn count_args(expr: &str) -> usize {
    let mut i: usize = 0;
    let mut c: usize = 0;

    if expr.is_empty() {
        return 0;
    }

    loop {
        match expr.chars().nth(i).unwrap() {
            ',' => {
                c += 1;
            }
            '(' => {
                if let Some(j) = find_closing_parenthesis(&expr[i..]) {
                    i += j;
                }
            }
            _ => {}
        };

        if i == expr.len() - 1 {
            break;
        } else {
            i += 1;
        }
    }

    c + 1
}

#[macro_export]
macro_rules! assert_parse_result_float {
    ($x:expr, $y:expr) => {
        assert!((parse($x, $x, 0).unwrap() - $y).abs() < f64::EPSILON);
    };
}

#[macro_export]
macro_rules! assert_parse_result_is {
    ($x:expr, $y:ident) => {
        assert!((parse($x, $x, 0).unwrap().$y()));
    };
}

#[macro_export]
macro_rules! assert_parse_error {
    ($x:expr, $y:expr) => {
        assert_eq!(format!("{}", parse($x, $x, 0).expect_err("")), $y);
    };
}

#[cfg(test)]
mod tests {
    use super::{count_args, find_closing_parenthesis, find_nth_comma, is_function_call};

    #[test]
    fn find_closing_parenthesis_simple() {
        assert_eq!(find_closing_parenthesis("(test)").unwrap(), 5);
        assert_eq!(find_closing_parenthesis("()").unwrap(), 1);
        assert!(find_closing_parenthesis("(test").is_none());
        assert!(find_closing_parenthesis("(").is_none());
    }

    #[test]
    fn is_function_call_simple() {
        assert!(is_function_call("test(test)").is_some());
        assert!(is_function_call("test()").is_some());
        assert!(is_function_call("test((),())").is_some());
        assert!(is_function_call("test").is_none());
        assert!(is_function_call("test(test)a").is_none());
        assert!(is_function_call("test()a").is_none());
    }

    #[test]
    fn find_nth_comma_simple() {
        assert_eq!(find_nth_comma("1,2,3", 1).unwrap(), 1);
        assert_eq!(find_nth_comma("1,2,3", 2).unwrap(), 3);
        assert!(find_nth_comma("1,2,3", 3).is_none());
        assert!(find_nth_comma("", 1).is_none());
        assert_eq!(find_nth_comma("1,(1,2),3", 1).unwrap(), 1);
        assert_eq!(find_nth_comma("1,(1,2),3", 2).unwrap(), 7);
    }

    #[test]
    fn count_args_simple() {
        assert_eq!(count_args("1,2,3"), 3);
        assert_eq!(count_args("1,2"), 2);
        assert_eq!(count_args("1"), 1);
        assert_eq!(count_args(""), 0);
        assert_eq!(count_args("1,(1,2),3"), 3);
        assert_eq!(count_args("(1,2)"), 1);
    }
}
