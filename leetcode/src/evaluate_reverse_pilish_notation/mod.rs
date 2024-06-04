struct Solution;

impl Solution {
    const OPERATORS: &'static [&'static str] = &["+", "-", "*", "/"];

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        println!("tokens = {:?}", tokens);

        for token in tokens {
            println!("stack = {:?}, token = {}", stack, token);
            if Solution::OPERATORS.contains(&token.as_str()) {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let res = match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!(),
                };
                stack.push(res);
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }

        stack.pop().unwrap()
    }
}

fn main() {
    let tokens = vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string(),
    ];
    let res = Solution::eval_rpn(tokens);
    println!("res = {}", res);
    assert_eq!(res, 9);

    let tokens = vec![
        "4".to_string(),
        "13".to_string(),
        "5".to_string(),
        "/".to_string(),
        "+".to_string(),
    ];
    let res = Solution::eval_rpn(tokens);
    println!("res = {}", res);
    assert_eq!(res, 6);

    let tokens = vec![
        "10".to_string(),
        "6".to_string(),
        "9".to_string(),
        "3".to_string(),
        "+".to_string(),
        "-11".to_string(),
        "*".to_string(),
        "/".to_string(),
        "*".to_string(),
        "17".to_string(),
        "+".to_string(),
        "5".to_string(),
        "+".to_string(),
    ];
    let res = Solution::eval_rpn(tokens);
    println!("res = {}", res);
    assert_eq!(res, 22);
}
