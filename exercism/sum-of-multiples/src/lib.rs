trait Multiple {
    fn is_multiple_of_any(&self, factors: &[u32]) -> bool;
}

impl Multiple for u32 {
    fn is_multiple_of_any(&self, factors: &[u32]) -> bool { 
        factors
            .iter()
            .filter(|&&z| z != 0)
            .any(|&f| self%f == 0)
    }
}


pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| x.is_multiple_of_any(factors))
        .sum()
}
