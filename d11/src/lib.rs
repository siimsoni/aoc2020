use std::io::BufRead;

#[derive(Debug)]
pub struct SeatMap {
    seats: Box<[bool]>,
    occupied: Box<[bool]>,
    width: usize,
}

pub fn parse<R>(mut reader: R) -> SeatMap
where
    R: BufRead,
{
    let mut seats = Vec::new();
    let mut width = 0;
    let mut buf: [u8; 4096] = [0; 4096];
    while let Ok(len) = reader.read(&mut buf) {
        if len == 0 {
            break;
        }
        seats.reserve(len);
        for c in buf[..len].iter() {
            match &c {
                b'\n' => {
                    if width == 0 {
                        width = seats.len();
                    }
                }
                b'L' => seats.push(true),
                b'.' => seats.push(false),
                _ => (), // ignore
            }
        }
    }
    seats.shrink_to_fit();
    let mut occupied = Vec::new();
    occupied.resize(seats.len(), false);
    SeatMap {
        seats: seats.into_boxed_slice(),
        occupied: occupied.into_boxed_slice(),
        width,
    }
}

pub fn p1_solve(seat_map: &SeatMap) -> Option<u64> {
    let occupied_after = p1_iterate(&seat_map.seats, &seat_map.occupied, &seat_map.width);
    let mut result = 0;
    for c in occupied_after.iter() {
        if *c {
            result += 1;
        }
    }

    Some(result)
}

pub fn p1_iterate(seats: &[bool], occupied: &Box<[bool]>, width: &usize) -> Box<[bool]> {
    let mut col = 0;
    let mut row = 0;
    let mut change_counter = 0;

    let mut tl = -1 - (*width as i32);
    let mut tc = tl + 1;
    let mut tr = tc + 1;
    let mut ml = -1;
    let mut mr = 1;
    let mut bl = (*width as i32) - 1;
    let mut bc = bl + 1;
    let mut br = bc + 1;
    let height = seats.len() / *width;

    let mut result = occupied.clone();

    for (i, s) in seats.iter().enumerate() {
        if *s {
            let mut adjacent_counter = 0;
            let not_top = row > 0;
            let not_left = col > 0;
            let not_bottom = row < height - 1;
            let not_right = col < *width - 1;

            if not_left {
                adjacent_counter += occupied[ml as usize] as usize;
                if not_top {
                    adjacent_counter += occupied[tl as usize] as usize;
                }
                if not_bottom {
                    adjacent_counter += occupied[bl as usize] as usize;
                }
            }
            if not_top {
                adjacent_counter += occupied[tc as usize] as usize;
            }
            if not_bottom {
                adjacent_counter += occupied[bc as usize] as usize;
            }
            if not_right {
                adjacent_counter += occupied[mr as usize] as usize;
                if not_top {
                    adjacent_counter += occupied[tr as usize] as usize;
                }
                if not_bottom {
                    adjacent_counter += occupied[br as usize] as usize;
                }
            }

            if adjacent_counter == 0 && !result[i] {
                change_counter += 1;
                result[i] = true;
            }
            if adjacent_counter >= 4 && result[i] {
                change_counter += 1;
                result[i] = false;
            }
        }

        tl += 1;
        tc += 1;
        tr += 1;
        ml += 1;
        mr += 1;
        bl += 1;
        bc += 1;
        br += 1;

        col += 1;
        if col == *width {
            col = 0;
            row += 1;
        }
    }

    if change_counter > 0 {
        return p1_iterate(seats, &result, width);
    }

    result
}

// fn print_seats(seats: &[bool], occupied: &[bool], width: &usize) {
//     let mut col = 0;
//     for (i, s) in seats.iter().enumerate() {
//         if !s {
//             print!(".");
//         } else {
//             if occupied[i] {
//                 print!("#");
//             } else {
//                 print!("L");
//             }
//         }
//         col += 1;
//         if col == *width {
//             print!("\n");
//             col = 0;
//         }
//     }
// }

pub fn p2_solve(seat_map: &SeatMap) -> Option<u64> {
    let occupied_after = p2_iterate(&seat_map.seats, &seat_map.occupied, &seat_map.width);
    let mut result = 0;
    for c in occupied_after.iter() {
        if *c {
            result += 1;
        }
    }

    Some(result)
}

pub enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

pub fn p2_iterate(seats: &[bool], occupied: &Box<[bool]>, width: &usize) -> Box<[bool]> {
    let mut col = 0;
    let mut row = 0;
    let mut change_counter = 0;
    let height = seats.len() / *width;

    let mut result = occupied.clone();

    for (i, s) in seats.iter().enumerate() {
        if *s {
            let mut adjacent_counter = 0;
            if p2_seek(
                col,
                row,
                width,
                &height,
                Direction::TopLeft,
                seats,
                occupied,
            ) {
                adjacent_counter += 1;
            }
            if p2_seek(col, row, width, &height, Direction::Top, seats, occupied) {
                adjacent_counter += 1;
            }
            if p2_seek(
                col,
                row,
                width,
                &height,
                Direction::TopRight,
                seats,
                occupied,
            ) {
                adjacent_counter += 1;
            }
            if p2_seek(col, row, width, &height, Direction::Left, seats, occupied) {
                adjacent_counter += 1;
            }
            if p2_seek(col, row, width, &height, Direction::Right, seats, occupied) {
                adjacent_counter += 1;
            }
            if p2_seek(
                col,
                row,
                width,
                &height,
                Direction::BottomLeft,
                seats,
                occupied,
            ) {
                adjacent_counter += 1;
            }
            if p2_seek(col, row, width, &height, Direction::Bottom, seats, occupied) {
                adjacent_counter += 1;
            }
            if p2_seek(
                col,
                row,
                width,
                &height,
                Direction::BottomRight,
                seats,
                occupied,
            ) {
                adjacent_counter += 1;
            }
            if adjacent_counter == 0 && !result[i] {
                change_counter += 1;
                result[i] = true;
            }
            if adjacent_counter >= 5 && result[i] {
                change_counter += 1;
                result[i] = false;
            }
        }

        col += 1;
        if col == *width {
            col = 0;
            row += 1;
        }
    }

    if change_counter > 0 {
        return p2_iterate(seats, &result, width);
    }

    result
}

pub fn p2_seek(
    col: usize,
    row: usize,
    width: &usize,
    height: &usize,
    direction: Direction,
    seats: &[bool],
    occupied: &[bool],
) -> bool {
    let mut new_col = col as i16;
    let mut new_row = row as i16;
    match direction {
        Direction::TopLeft => {
            new_col -= 1;
            new_row -= 1;
        }
        Direction::Top => {
            new_row -= 1;
        }
        Direction::TopRight => {
            new_col += 1;
            new_row -= 1;
        }
        Direction::Left => {
            new_col -= 1;
        }
        Direction::Right => {
            new_col += 1;
        }
        Direction::BottomLeft => {
            new_col -= 1;
            new_row += 1;
        }
        Direction::Bottom => {
            new_row += 1;
        }
        Direction::BottomRight => {
            new_col += 1;
            new_row += 1;
        }
    }
    if new_row < 0 || new_row > (*height as i16) - 1 || new_col < 0 || new_col > (*width as i16) - 1
    {
        return false;
    }

    let pos = (new_row as usize) * width + (new_col as usize);
    if !seats[pos] {
        return p2_seek(
            new_col as usize,
            new_row as usize,
            width,
            height,
            direction,
            seats,
            occupied,
        );
    }
    occupied[pos]
}
