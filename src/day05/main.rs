use std::fs;
use std::collections::HashSet;

#[macro_use] extern crate maplit;


static ROW_COUNT: usize = 128;
static COLUMN_COUNT: usize = 8;


struct Seat {
    row: usize,
    column: usize,
}


fn partition(input: &[u8], lower: u8, higher: u8) -> usize {
    let mut min = 0 as usize;
    let mut max = 2_usize.pow(input.len() as u32);
    for char in input {
        let midpoint = (max - min) / 2;
        if *char == lower {
            max = min + midpoint;
        } else if *char == higher {
            min = min + midpoint;
        }
    }
    min
}


impl Seat {
    fn decode(input: &str) -> Seat {
        let bytes = input.as_bytes();
        let row = partition(&bytes[..7], 'F' as u8, 'B' as u8);
        let column = partition(&bytes[7..], 'L' as u8, 'R' as u8);
        Seat {row, column}
    }

    fn id(&self) -> usize {
        self.row * COLUMN_COUNT + self.column
    }
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to read input.txt");

    let seats: Vec<Seat> = contents
        .lines()
        .map(|input| Seat::decode(input))
        .collect();

    let highest_id = (&seats).into_iter().map(|seat| seat.id()).max().unwrap();
    println!("{}", highest_id);

    // Build full seating plan
    let mut rows: Vec<Vec<&Seat>> = Vec::with_capacity(ROW_COUNT);
    rows.resize(ROW_COUNT, Vec::new());
    for seat in &seats {
        rows[seat.row].push(seat);
    }
    let rows = rows;

    let my_row = (&rows)
        .iter()
        .skip_while(|row| row.len() < COLUMN_COUNT)
        .skip_while(|row| row.len() == COLUMN_COUNT)
        .next()
        .unwrap()
        ;

    let seat_numbers: HashSet<usize> = hashset!(0, 1, 2, 3, 4, 5, 6, 7);
    let seats_in_my_row: HashSet<usize> = my_row.iter().map(|seat| seat.column).collect();
    for seat_number in seat_numbers.difference(&seats_in_my_row) {
        let seat = Seat { row: my_row[0].row, column: *seat_number };
        println!("{}", seat.id());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat() {
        let input = "FBFBBFFRLR";
        let seat = Seat::decode(input);
        assert_eq!(seat.row, 44);
        assert_eq!(seat.column, 5);
        assert_eq!(seat.id(), 357);
    }
}
