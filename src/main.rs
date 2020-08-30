mod board;
mod game_instance;

use std::error::Error;
use game_instance::GameConfig;
use game_instance::GameInstance;

fn main() -> Result<(), Box<dyn Error>> {
    let warps = board::get_board_warps();

    let config = GameConfig {
        warps,
        can_overshoot_end: false,
    };

    let mut game = GameInstance::create(&config);

    // simulate a bunch of games and record their results
    let mut writer = csv::Writer::from_path("./runs.csv").unwrap();
    for _ in 0..1000000 {
        writer.serialize(game.simulate_player())?;
    }
    writer.flush()?;

    println!("Complete.");
    Ok(())
}
