pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec![];

    let rows = input.len();
    let cols = input[0].len();

    for y in 0..rows {
        for x in 0..cols {
            if is_saddle_point2(input, y, x) {
                saddle_points.push((y, x))
            }
        }
    }

    return saddle_points;
}

fn is_less_or_equal(candidate: u64) -> impl Fn(u64) -> bool {
    return move |other: u64| candidate <= other;
}

fn is_greater_or_equal(candidate: u64) -> impl Fn(u64) -> bool {
    return move |other: u64| candidate >= other;
}

pub fn row(input: &[Vec<u64>], y: usize) -> Vec<u64> {
    return input[y].to_owned();
}

pub fn column(input: &[Vec<u64>], x: usize) -> Vec<u64> {
    return input.into_iter().map(|col: &Vec<u64>| col[x]).collect();
}

fn is_saddle_point2(input: &[Vec<u64>], y: usize, x: usize) -> bool {
    let r = row(input, y);
    let c = column(input, x);

    let value = input[y][x];

    let max_in_row: bool = r
        .into_iter()
        .map(is_greater_or_equal(value))
        .fold(true, |acc: bool, el: bool| acc && el);

    let min_in_column: bool = c
        .into_iter()
        .map(is_less_or_equal(value))
        .fold(true, |acc: bool, el: bool| acc && el);

    return min_in_column && max_in_row;
}
