#[cfg(test)] mod data;

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    fn test_0() {
        assert_eq!(1, 1);
    }
}