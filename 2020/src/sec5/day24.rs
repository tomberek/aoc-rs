use counter::Counter;
use std::collections::HashSet;
use utils::cartesian::Point;
use utils::nums::NumExt;

//COORDS base are +x is e , +y is ne.

pub fn parse_line(input: &str) -> Option<Point<i32>> {
    let mut it = input.chars();
    let mut p = Point::new(0, 0);
    while let Some(c) = it.next() {
        p = match c {
            'e' => p.right(),
            'w' => p.left(),
            'n' => match it.next()? {
                'e' => p.up(),
                'w' => p.up().left(),
                _ => None?,
            },
            's' => match it.next()? {
                'e' => p.down().right(),
                'w' => p.down(),
                _ => None?,
            },
            _ => None?,
        }
    }
    Some(p)
}

pub fn make_floor(input: &str) -> HashSet<Point<i32>> {
    let counts = input
        .lines()
        .map(parse_line)
        .collect::<Option<Counter<Point<i32>>>>()
        .unwrap();
    counts.iter().filter(|x| x.1 % 2 == 1).map(|(&x, _)| x).collect()
}

#[aoc(day24, part1)]
pub fn p1(input: &str) -> usize {
    make_floor(input).len()
}

pub fn step(a: &HashSet<Point<i32>>) -> HashSet<Point<i32>> {
    let cs: Counter<Point<i32>> = a.iter().flat_map(Point::hex_neighbours).collect();
    cs.iter()
        .filter(|(p, &c)| c == 2 || (a.contains(p) && c == 1))
        .map(|(a, _)| *a)
        .collect()
}

#[aoc(day24, part2)]
pub fn p2(input: &str) -> usize {
    100.applications_of_ref(make_floor(input), step).len()
}

#[cfg(test)]
mod regression {
    use super::{p1, p2};
    const ANS: (usize, usize) = (434, 3955);
    const INP: &str = include_str!("../../input/2020/day24.txt");
    #[test]
    pub fn regression() {
        assert_eq!(p1(INP), ANS.0);
        assert_eq!(p2(INP), ANS.1);
    }
}
