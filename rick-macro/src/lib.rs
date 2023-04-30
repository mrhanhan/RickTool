macro_rules! add {
    ($x:expr, $y:expr) => ({
        println!("{:?}", $x);
        ($x + $y)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add!(2, 2);
        println!("{:?}", result);
    }
}
