mod board;
mod game_instance;

use game_instance::GameConfig;
use game_instance::GameInstance;

fn main() {
    let warps = board::get_board_warps();

    let config = GameConfig {
        warps,
        can_overshoot_end: false,
    };

    let mut game = GameInstance::create(&config);

    // simulate 1,000 games
    for _ in 1..1000000 {
        game.simulate_player();
    }

    // println!("{:?}", run);
}
