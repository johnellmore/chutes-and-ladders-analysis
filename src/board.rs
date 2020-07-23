#[derive(Copy, Clone)]
pub enum TransportType {
    Chute(u8),
    Ladder(u8),
    None,
}

pub type BoardWarps = [TransportType; 100];

pub fn get_board_warps() -> BoardWarps {
    // define a map of each space on the game board to where it should "warp"
    // the player to (if anywhere)
    let mut warps : BoardWarps = [TransportType::None; 100];

    // define each chute and ladder, as a (from_space, to_space) tuple
    let chutes_and_ladders = [
        // ladders
        (1, 38),
        (4, 14),
        (9, 31),
        (21, 42),
        (28, 84),
        (36, 44),
        (51, 67),
        (71, 91),
        (80, 100),

        // chutes
        (16, 6),
        (48, 26),
        (49, 11),
        (56, 53),
        (62, 19),
        (64, 60),
        (87, 24),
        (93, 73),
        (95, 75),
        (98, 78)
    ];

    // update the game board space map with our known transitions
    for transition in chutes_and_ladders.iter() {
        let (from_space, to_space) = transition;
        if from_space < to_space {
            warps[*from_space as usize] = TransportType::Ladder(*to_space)
        } else {
            warps[*from_space as usize] = TransportType::Chute(*to_space)
        }
    }

    warps
}