pub fn answer(command: &str) -> Option<i32> {

    let command = command.strip_prefix("What is ")?.strip_suffix("?")?.trim();

    if command.is_empty() {
        return None;
    }

    let tokens: Vec<&str> = command.split_whitespace().collect();

    let mut iter = tokens.iter().peekable();
    let mut result: i32;

    let first = iter.next()?;
    result = first.parse::<i32>().ok()?;

    while let Some(&op) = iter.next() {

        let next_op = match op {
            "plus" => "+",
            "minus" => "-",
            "multiplied" => {
                if iter.next()? != &"by" { return None; }
                "*"
            }
            "divided" => {
                if iter.next()? != &"by" { return None; }
                "/"
            }
            _ => return None, 
        };

        let next_num_str = iter.next()?;
        let next_num = next_num_str.parse::<i32>().ok()?;

        result = match next_op {
            "+" => result + next_num,
            "-" => result - next_num,
            "*" => result * next_num,
            "/" => result / next_num,
            _ => unreachable!(),
        };
    }

    Some(result)
}
