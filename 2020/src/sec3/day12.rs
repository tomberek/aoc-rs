use crate::utils::cartesian::{Dir, Point};

pub fn solve<F>(input: &str,waypoint : Point<isize>, move_fn : F) -> isize 
    where F : Fn(Point<isize>,Point<isize>,Point<isize>) -> (Point<isize>,Point<isize>)
{
    input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..].parse().unwrap()))
        .fold((Point::new(0, 0), waypoint), |(ship, way), (c, d)| match c {
            'F' => (ship + way * d, way),
            'L' => (ship, (0..d / 90).fold(way, |d, _| d.rotate_left_about_origin())),
            'R' => (ship, (0..d / 90).fold(way, |d, _| d.rotate_right_about_origin())),
            c => move_fn(ship,way,Dir::from_x("NSWE", c) * d)
        })
        .0
        .manhattan()
}
#[aoc(day12, part1)]
pub fn p1(input: &str) -> isize {
    solve(input,Point::new(1,0), |ship,way,step| (ship+step,way))
}
#[aoc(day12, part2)]
pub fn p2(input: &str) -> isize {
    solve(input,Point::new(10,1), |ship,way,step| (ship,way+step))
}
