pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec!();
    
    let cols = input.len();
    let rows = input[0].len();

    for x in 0..cols {
        for y in 0..rows {
            if is_saddle_point(input, cols, rows, x, y) {
                saddle_points.push((x, y))
            }
        }
    }

    return saddle_points;
}

fn is_saddle_point(input: &[Vec<u64>], cols: usize, rows: usize, x: usize, y: usize) -> bool {
    let signed_x = x as i32;
    let signed_y = y as i32;
    let signed_cols = cols as i32;
    let signed_rows = rows as i32;

    for n in 1.. {
        if !has_cell_n_above(signed_y, n) {
            break;
        }
        if input[x][y - n as usize] < input[x][y] {
            return false;
        }
    }

    for n in 1.. {
        if !has_cell_n_below(signed_y, signed_rows, n) {
            break;
        }
        if input[x][y + n as usize] < input[x][y] {
            return false;
        }
    }
   
    for n in 1.. {
        if !has_cell_n_to_left(signed_x, n) {
            break;
        }
        if input[x - n as usize][y] < input[x][y] {
            return false;
        }
    }

    for n in 1.. {
        if !has_cell_n_to_right(signed_x, signed_cols, n) {
            break;
        }
        if input[x + n as usize][y] < input[x][y] {
            return false;
        }
    }

//    // for n in 1.. {
    //     if !has_cell_above(signed_y + 1 - n) {
    //         break;
    //     }
    //     if input[x][y - n as usize] < input[x][y] {
    //         return false;
    //     }
    // }
    // for n in 1.. {
    //     if !has_cell_below(signed_y - 1 + n, signed_rows) {
    //         break;
    //     }
    //     if input[x][y + n as usize] < input[x][y] {
    //         return false;
    //     }
    // }
    // for n in 1.. {
    //     if !has_cell_to_left(signed_x + 1 - shift) {
    //         break;
    //     }
    //     if input[x - shift as usize][y] < input[x][y] {
    //         return false;
    //     }
    // }
    // for shift in 1.. {
    //     if !has_cell_to_right(signed_x - 1 + shift, signed_cols) {
    //         break;
    //     }
    //     if input[x + shift as usize][y] < input[x][y] {
    //         return false;
    //     }
    // }

    return true;
}

fn has_cell_above(y: i32) -> bool {
    return y > 0
}

fn has_cell_below(y: i32, rows: i32) -> bool {
    return y + 1 < rows
}

fn has_cell_to_left(x: i32) -> bool {
    return x > 0
}

fn has_cell_to_right(x: i32, cols: i32) -> bool {
    return x + 1 < cols
}

fn has_cell_n_above(y: i32, n: i32) -> bool {
    return y - n > 0
}

fn has_cell_n_below(y: i32, rows: i32, n: i32) -> bool {
    return y + n < rows - 1
}

fn has_cell_n_to_left(x: i32, n: i32) -> bool {
    return x - n > 0
}

fn has_cell_n_to_right(x: i32, cols: i32, n: i32) -> bool {
    return x + n < cols - 1
}


fn column(input: &[Vec<u64>], x: usize, y: usize) -> Vec<(usize, usize)> {
    // let r = input.to_owned().into_iter().map(move |col: Vec<u64>| (x, col[y] as usize));
    let r = input.into_iter().map(|col: &Vec<u64>| (x, col[y] as usize));
    return r.collect();
}
