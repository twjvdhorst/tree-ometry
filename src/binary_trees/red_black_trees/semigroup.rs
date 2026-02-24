pub trait Semigroup {
    fn op(base: &Self, other: &Self) -> Self;
    
    fn semigroup(&self, other: &Self) -> Self
    where 
        Self: Sized,
    {
        Self::op(self, other)
    }
}
