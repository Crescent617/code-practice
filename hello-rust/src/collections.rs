use std::ops::Add;

fn add<T>(a: T, b: T) -> T::Output
where
    T: Add,
{
    a + b
}
