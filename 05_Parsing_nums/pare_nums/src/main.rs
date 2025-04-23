fn parse_nums(input: Vec<Option<&str>>) -> (Vec<i32>, Vec<String>) {
    let mut input_iter = input.iter();

    input_iter
        .map(|opt_str| {
            opt_str
                .ok_or_else(|| "No data".to_string())
                .and_then(|s| s.parse::<i32>().map_err(|_| "Parse Error".to_string()))
        })
        .fold((vec![], vec![]), |(mut ok_vals, mut err_vals), result| {
            match result {
                Ok(num) => ok_vals.push(num),
                Err(e) => err_vals.push(e),
            }
            (ok_vals, err_vals)
        })
}

fn main() {
    let input = vec![Some("42"), None, Some("hello"), Some("-7")];

    let (valids, errors) = parse_nums(input);
    println!("{:?}", valids);
    println!("{:?}", errors);
}