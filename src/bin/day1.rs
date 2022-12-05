use std::fs::read_to_string;
const EXAMPLE_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";
fn main() {
    dbg!(descending_window(4));
    dbg!(zip_metod(3));
}
fn descending_window(win_size: usize) -> usize {
    let depths: Vec<usize> = read_to_string("inputs/day1.txt")
        .unwrap()
        .lines()
        .map(|a| a.parse::<usize>().unwrap())
        .collect();
    depths
        .windows(win_size)
        .filter(|win| 
            win[win_size - 1] > win[0]
        )
        .count()
}
fn zip_metod(skip_amount:usize) -> usize {
    let input = read_to_string("inputs/day1.txt").unwrap();
    let depths = input.lines().map(|a| a.parse::<i32>().unwrap());
    depths
        .clone()
        .zip(depths.skip(skip_amount))
        .filter(|(d1, d2)| d2 > d1)
        .count()
}
#[test]
fn part_two_try2() {
    let input = read_to_string("inputs/day1.txt").unwrap();
    let depths = input.lines().map(|a| a.parse::<i32>().unwrap());
    assert_eq!(depths
        .clone()
        .zip(depths.skip(3))
        .filter(|(d1, d2)| d2 > d1)
        .count(),1158)
}
#[test]
fn part_one_test() {
    let depths: Vec<usize> = EXAMPLE_INPUT.lines().map(|a| a.parse().unwrap()).collect();
    let mut increased = 0;
    for (index, depth) in depths.iter().enumerate() {
        if index != 0 {
            let last_index: usize = index - 1;
            if depth > &depths[last_index] {
                increased += 1
            }
        }
    }
    assert_eq!(increased, 7)
}
#[test]
fn part_one_test2() {
    let depths = EXAMPLE_INPUT.lines().map(|a| a.parse::<i32>().unwrap());
    assert_eq!(
        depths
            .clone()
            .zip(depths.skip(1))
            .filter(|(d1, d2)| d2 > d1)
            .count(),
        7
    );
}
#[test]
fn part_one_test3() {
    let depths: Vec<i32> = EXAMPLE_INPUT
        .lines()
        .map(|a| a.parse::<i32>().unwrap())
        .collect();
    assert_eq!(depths.windows(2).filter(|win| win[1] > win[0]).count(), 7);
}
