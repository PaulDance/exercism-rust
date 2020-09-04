use std::cmp::PartialOrd;
use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Add<Output = T> + PartialOrd + PartialEq + From<u8> + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|x| *x <= 0.into())
            || sides[0] + sides[1] < sides[2]
            || sides[0] + sides[2] < sides[1]
            || sides[1] + sides[2] < sides[0]
        {
            None
        } else {
            Some(Self { sides })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let first = self.sides[0];
        self.sides.iter().all(|&side| side == first)
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }
}
