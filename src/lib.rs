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

// fn is_saddle_point(input: &[Vec<u64>], cols: usize, rows: usize, x: usize, y: usize) -> bool {
//     let signed_x = x as i32;
//     let signed_y = y as i32;
//     let signed_cols = cols as i32;
//     let signed_rows = rows as i32;

//     for n in 1.. {
//         if !has_cell_n_above(signed_y, n) {
//             break;
//         }
//         if input[x][y - n as usize] < input[x][y] {
//             return false;
//         }
//     }

//     for n in 1.. {
//         if !has_cell_n_below(signed_y, signed_rows, n) {
//             break;
//         }
//         if input[x][y + n as usize] < input[x][y] {
//             return false;
//         }
//     }

//     for n in 1.. {
//         if !has_cell_n_to_left(signed_x, n) {
//             break;
//         }
//         if input[x - n as usize][y] < input[x][y] {
//             return false;
//         }
//     }

//     for n in 1.. {
//         if !has_cell_n_to_right(signed_x, signed_cols, n) {
//             break;
//         }
//         if input[x + n as usize][y] < input[x][y] {
//             return false;
//         }
//     }

//     return true;
// }

// fn has_cell_n_above(y: i32, n: i32) -> bool {
//     return y - n > 0;
// }

// fn has_cell_n_below(y: i32, rows: i32, n: i32) -> bool {
//     return y + n < rows - 1;
// }

// fn has_cell_n_to_left(x: i32, n: i32) -> bool {
//     return x - n > 0;
// }

// fn has_cell_n_to_right(x: i32, cols: i32, n: i32) -> bool {
//     return x + n < cols - 1;
// }

// pub fn column_coords(input: &[Vec<u64>], x: usize) -> Vec<(usize, usize)> {
//     let v = (0..input[x].len())
//         .into_iter()
//         .map(|y: usize| (x, y))
//         .collect();
//     return v;
// }
