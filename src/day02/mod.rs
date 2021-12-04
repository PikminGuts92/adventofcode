#[cfg(test)] mod data;

pub enum MoveAction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub fn calculate_new_position(moves: &[MoveAction]) -> (i32, i32) {
    let mut x = 0; // Horizontal pos
    let mut y = 0; // Vertical pos (depth)

    for mv in moves.iter() {
        match mv {
            MoveAction::Forward(d) => x += d,
            MoveAction::Up(d) => y -= d,
            MoveAction::Down(d) => y += d,
        }
    }

    (x, y)
}

pub fn calculate_new_position_with_aim(moves: &[MoveAction]) -> (i32, i32) {
    let mut x = 0; // Horizontal pos
    let mut y = 0; // Vertical pos (depth)
    let mut aim = 0;

    for mv in moves.iter() {
        match mv {
            MoveAction::Forward(d) => {
                x += d;
                y += aim * d;
            },
            MoveAction::Up(d) => aim -= d,
            MoveAction::Down(d) => aim += d,
        }
    }

    (x, y)
}

impl From<(&str, i32)> for MoveAction {
    fn from(mv: (&str, i32)) -> Self {
        let (dir, dis) = mv;

        match dir {
            "forward" => MoveAction::Forward(dis),
            "up" => MoveAction::Up(dis),
            "down" => MoveAction::Down(dis),
            _ => unimplemented!("Direction not supported!")
        }
    }
}

impl From<&(&str, i32)> for MoveAction {
    fn from(mv: &(&str, i32)) -> Self {
        let (dir, dis) = mv;

        match dir {
            &"forward" => MoveAction::Forward(*dis),
            &"up" => MoveAction::Up(*dis),
            &"down" => MoveAction::Down(*dis),
            _ => unimplemented!("Direction not supported!")
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 150)]
    #[case(TEST_DATA_2, 1746616)]
    fn calculate_new_position_test<T: AsRef<[(&'static str, i32)]>>(#[case] moves: T, #[case] expected: i32) {
        let moves = moves
            .as_ref()
            .iter()
            .map(|m| m.into())
            .collect::<Vec<MoveAction>>();

        let (x, y) = calculate_new_position(moves.as_slice());
        let product = x * y;

        assert_eq!(expected, product);
    }

    #[rstest]
    #[case(TEST_DATA_1, 900)]
    #[case(TEST_DATA_2, 1741971043)]
    fn calculate_new_position_with_aim_test<T: AsRef<[(&'static str, i32)]>>(#[case] moves: T, #[case] expected: i32) {
        let moves = moves
            .as_ref()
            .iter()
            .map(|m| m.into())
            .collect::<Vec<MoveAction>>();

        let (x, y) = calculate_new_position_with_aim(moves.as_slice());
        let product = x * y;

        assert_eq!(expected, product);
    }
}