#[derive(Copy,Clone,Debug)]
pub enum Row {
    Front,
    Back,
}

#[derive(Copy,Clone)]
pub enum Column {
    Left,
    Right,
}

pub struct Seat {
    row: u32,
    col: u32,
}

pub struct Rs([Row; 7]);

pub struct Cs([Column; 3]);

pub struct BoardingPass {
    rs: Rs,
    cs: Cs,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Range {
    lower: u32,
    upper: u32,
}

impl Range{
    fn new_row() -> Range {
        Range {
            lower: 0, upper: 127
        }
    }
    fn new_column() -> Range {
        Range {
            lower: 0, upper: 7
        }
    }
    fn take_lower(self) -> Range {
        Range {
            lower: self.lower, upper: self.lower + ((self.upper - self.lower) / 2)
        }
    }
    fn take_upper(self) -> Range {
        Range {
            lower: self.lower + (((self.upper - self.lower) / 2) + 1), upper: self.upper
        }
    }
    fn result(self) -> u32 {
        if self.lower == self.upper {
            self.lower
        } else {
            panic!{"Non-terminated range"}
        }
    }
}

// #[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

// Can’t get this annotation to work
// #[aoc(day5, part1)]
pub fn solve_part1(input: &[&str]) -> u32 {
    input.iter()
        .map(|s| -> u32 {
            let (r, c) = calculate_from_string_input(s);
            seat_id(r,c)
        })
        .max().unwrap()
}

fn seat_id(row: u32, col: u32) -> u32 {
    row * 8 + col
}

fn calculate_from_string_input(input: &str) -> (u32, u32) {
    let sr = input.get(0..7).unwrap();
    let r = from_string_to_row(sr);
    let rnum = calculate_from_whole_row(&r);

    let cr = input.get(7..10).unwrap();
    let c = from_string_to_column(cr);
    let cnum = calculate_from_whole_column(&c);

    (rnum, cnum)
}

fn from_string_to_row(input: &str) -> [Row; 7] {
    let mut r = [Row::Front; 7];
    let mut i = 0;
    for ch in input.chars() {
        r[i] = to_row(ch);
        i += 1;
    }
    r
}

fn from_string_to_column(input: &str) -> [Column; 3] {
    let mut c = [Column::Left; 3];
    let mut i = 0;
    for ch in input.chars() {
        c[i] = to_column(ch);
        i += 1;
    }
    c
}

fn to_row(c: char) -> Row {
    match c {
        'F' => Row::Front,
        'B' => Row::Back,
        _ => panic!{"to_row: nonsense"}
    }
}

fn to_column(ch: char) -> Column {
    match ch {
        'L' => Column::Left,
        'R' => Column::Right,
        _ => panic!{"to column: nonsense"}
    }
}

fn calculate_from_row(ra: Range, r: Row) -> Range {
    match r {
        Row::Front => ra.take_lower(),
        Row::Back => ra.take_upper(),
    }
}

fn calculate_from_whole_row(rs: &[Row; 7]) -> u32 {
    calculate_helper(rs, Range::new_row(), calculate_from_row)
}

fn calculate_from_column(ra: Range, c: Column) -> Range {
    match c {
        Column::Left => ra.take_lower(),
        Column::Right => ra.take_upper(),
    }
}

fn calculate_from_whole_column(cs: &[Column; 3]) -> u32 {
    calculate_helper(cs, Range::new_column(), calculate_from_column)
}

fn calculate_helper<T: Copy, F>(container: &[T], initial: Range, mut calc: F) -> u32
where F: FnMut(Range, T) -> Range {
    container.iter().fold(initial,
                   |acc, &elem| calc(acc, elem)).result()
}

#[test]
fn calculate_from_row_fbfbbfb() {
    let first = calculate_from_row(Range::new_row(), Row::Front);
    assert_eq!(Range {lower: 0, upper: 63}, first);
    let second = calculate_from_row(first, Row::Back);
    assert_eq!(Range {lower: 32, upper: 63}, second);
    let third = calculate_from_row(second, Row::Front);
    assert_eq!(Range {lower: 32, upper: 47}, third);
    let fourth = calculate_from_row(third, Row::Back);
    assert_eq!(Range {lower: 40, upper: 47}, fourth);
    let fifth = calculate_from_row(fourth, Row::Back);
    assert_eq!(Range {lower: 44, upper: 47}, fifth);
    let sixth = calculate_from_row(fifth, Row::Front);
    assert_eq!(Range {lower: 44, upper: 45}, sixth);
    let seventh = calculate_from_row(sixth, Row::Front);
    assert_eq!(Range {lower: 44, upper: 44}, seventh);
}
//
#[test]
fn calculcate_from_column_rlr() {
    let first = calculate_from_column(Range::new_column(), Column::Right);
    assert_eq!(Range {lower: 4, upper: 7}, first);
    let second = calculate_from_column(first, Column::Left);
    assert_eq!(Range {lower: 4, upper: 5}, second);
    let third = calculate_from_column(second, Column::Right);
    assert_eq!(Range {lower: 5, upper: 5}, third);
}
//
#[test]
fn calculate_from_whole_row_fbfbbff() {
    use self::Row::*;
    let row = [Front, Back, Front, Back, Back, Front, Front];
    assert_eq!(44, calculate_from_whole_row(&row))
}
//
#[test]
fn calculate_from_whole_column_rlr() {
    use self::Column::*;
    let column = [Right, Left, Right];
    assert_eq!(5, calculate_from_whole_column(&column))
}
//
#[test]
fn calculate_from_string_input_fbfbffrlr_and_seat_id() {
    let s = "FBFBBFFRLR";
    let (row, col) = calculate_from_string_input(&s);
    assert_eq!((44, 5), (row, col));
    assert_eq!(357, seat_id(row, col))
}
