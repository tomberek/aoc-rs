use crate::utils::cartesian::Point;
use std::cmp::{max, min};
use num::Num;
use std::convert::TryInto;
use std::fmt::Debug;
use std::ops::{RangeInclusive};

#[derive(Clone, Copy, Debug)]
pub struct Aabb<T> {
    pub bottom_left: Point<T>,
    pub top_right: Point<T>,
}

impl<T> Aabb<T>
    where T: Num + Copy + TryInto<usize> + Ord,
          RangeInclusive<T>: std::iter::Iterator<Item = T>,
{
    pub fn new(p: Point<T>) -> Self {
        Self {
            bottom_left: p,
            top_right: p,
        }
    }

    pub fn extend(&self, p: Point<T>) -> Self {
        let mut ans = *self;
        ans.bottom_left.x = min(ans.bottom_left.x, p.x);
        ans.bottom_left.y = min(ans.bottom_left.y, p.y);
        ans.top_right.x = max(ans.top_right.x, p.x);
        ans.top_right.y = max(ans.top_right.y, p.y);
        ans
    }
    pub fn contains(&self, p: Point<T>) -> bool {
        self.bottom_left.x <= p.x
            && self.bottom_left.y <= p.y
            && self.top_right.x >= p.x
            && self.top_right.y >= p.y
    }
    pub fn extend_box(&self, b: Self) -> Self {
        self.extend(b.bottom_left).extend(b.top_right)
    }
    pub fn intersect(&self, b: Self) -> Self {
        Self {
            bottom_left: Point::new(
                max(self.bottom_left.x, b.bottom_left.x),
                max(self.bottom_left.y, b.bottom_left.y),
            ),
            top_right: Point::new(
                min(self.top_right.x, b.top_right.x),
                min(self.top_right.y, b.top_right.y),
            ),
        }
    }
    pub fn all_points(&self) -> impl Iterator<Item=Point<T>> + '_ {
        (self.bottom_left.y..=self.top_right.y)
            .flat_map(move |y|
                (self.bottom_left.x..=self.top_right.x)
                    .map(move |x| Point::new(x, y)))
    }
    pub fn vec_with<TO: Default + Clone>(&self, ft: impl Fn(Point<T>) -> TO) -> Vec<Vec<TO>> {
        let offset = self.bottom_left;
        let mut v = vec![vec![Default::default(); self.width()]; self.height()];
        for p in self.all_points() {
            let rel = p - offset;
            let x: usize = Self::to_usize(rel.x);
            let y: usize = Self::to_usize(rel.y);
            v[y][x] = ft(p);
        }
        v
    }
    fn to_usize(t: T) -> usize {
        match t.try_into() {
            Ok(x) => x,
            Err(_) => panic!("Can't convert to usize"),
        }
    }
    pub fn width(&self) -> usize {
        Self::to_usize(T::one() + self.top_right.x - self.bottom_left.x)
    }
    pub fn height(&self) -> usize {
        Self::to_usize(T::one() + self.top_right.y - self.bottom_left.y)
    }
}
