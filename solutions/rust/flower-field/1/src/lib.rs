pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut result = vec![];
    let rows = garden.len();
    if rows != 0 {
        let columns = garden[0].len();
        for y in 0..rows {
            let mut string = vec![];
            for x in 0..columns {
                if garden[y].as_bytes()[x] == b' ' {
                    let new = check_flower(garden, x, y, rows, columns);
                    if new != '0' {
                        string.push(new);
                    } else {
                        string.push(' ');
                    }
                }
                if garden[y].as_bytes()[x] == b'*' {
                    string.push('*');
                }
            }
            let string = string.into_iter().collect::<String>();
            result.push(string);
        }
    }
    result
}

fn check_flower(garden: &[&str], x: usize, y: usize, rows: usize, columns: usize) -> char {
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    let mut new_directions = vec![];

    for direction in directions {
        let (dx, dy) = direction;
        let nx = x as isize - dx;
        let ny = y as isize - dy;
        if nx < columns as isize && nx >= 0 && ny < rows as isize && ny >= 0 {
            new_directions.push((nx as usize, ny as usize))
        }
    }
    for (nx, ny) in new_directions {
        if garden[ny].as_bytes()[nx] == b'*' {
            count += 1;
        }
    }
    char::from_digit(count, 10).unwrap()
}
