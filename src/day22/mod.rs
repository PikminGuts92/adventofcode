#[cfg(test)] mod data;

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    //#[case]
    pub fn test() {
        assert_eq!(true, true);
    }
}