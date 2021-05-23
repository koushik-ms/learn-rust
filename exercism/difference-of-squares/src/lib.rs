pub trait Sq {
    fn sq(self) -> Self;
}

impl Sq for u32 {
    fn sq(self: u32) -> u32 { self*self }
}

pub fn square_of_sum(n: u32) -> u32 {
    (1..=n)
        .sum::<u32>()
        .sq()
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n)
        .map(Sq::sq)
        .sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
