pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let rows = input.to_vec();
    let cols = rows_to_columns(&rows);

    let mut saddle_points = vec![];
    for x in 0..cols.len() {
        for y in 0..rows.len() {
            if is_saddle_point(&cols, &rows, x, y) {
                // my coordinates are (x, y), but they use (y, x)
                saddle_points.push((y, x))
            }
        }
    }

    return saddle_points;
}

// Turns a vector in the following form
//      [[a, b], [c, d]]
// into this
//      [[a, c], [b, d]]
pub fn rows_to_columns(rows: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut cols: Vec<Vec<u64>> = Vec::with_capacity(rows[0].len());

    for col_index in 0..rows[0].len() {
        let n = rows.into_iter().map(move |row: &Vec<u64>| row[col_index]).collect();
        cols.push(n)
    }

    return cols
}

fn is_saddle_point(cols: &Vec<Vec<u64>>, rows: &Vec<Vec<u64>>, x: usize, y: usize) -> bool {
    let point_value = &cols[x][y];
    let row = &rows[y];
    let col = &cols[x];

    let max_in_row: bool = row
        .into_iter()
        .map(|other: &u64| point_value >= other)
        .fold(true, |acc: bool, el: bool| acc && el);

    let min_in_column: bool = col
        .into_iter()
        .map(|other: &u64| point_value <= other)
        .fold(true, |acc: bool, el: bool| acc && el);

    return min_in_column && max_in_row;
}

pub fn row(input: &Vec<Vec<u64>>, y: usize) -> Vec<u64> {
    return input[y].to_owned();
}

pub fn column(input: &Vec<Vec<u64>>, x: usize) -> Vec<u64> {
    return input.into_iter().map(|col: &Vec<u64>| col[x]).collect();
}
