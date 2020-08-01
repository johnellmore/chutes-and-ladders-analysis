use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;
use serde::Serialize;
use crate::board::TransportType;
use crate::board::BoardWarps;

pub struct GameConfig {
    pub warps: BoardWarps,
    pub can_overshoot_end: bool,
}

impl fmt::Debug for GameConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameConfig")
         .field("can_overshoot_end", &self.can_overshoot_end)
         .finish()
    }
}


#[derive(Debug)]
pub struct GameInstance<'a> {
    config: &'a GameConfig,
    rng: ThreadRng,
    turns: u32,
    spaces_traversed: u32,
    ladders_taken: u32,
    chutes_taken: u32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PlayerRun {
    pub turns: u32,
    pub spaces_traversed: u32,
    pub ladders_taken: u32,
    pub chutes_taken: u32,
}

impl<'a> GameInstance<'a> {
    pub fn create<'b>(config: &'b GameConfig) -> GameInstance<'b> {
        GameInstance {
            config,
            rng: rand::thread_rng(),
            turns: 0,
            spaces_traversed: 0,
            ladders_taken: 0,
            chutes_taken: 0,
        }
    }

    pub fn simulate_player(&mut self) -> PlayerRun {
        self.turns = 0;
        self.spaces_traversed = 0;
        self.ladders_taken = 0;
        self.chutes_taken = 0;

        let mut pos = 0;
        while pos != 100 {
            self.turns += 1;
            pos = self.move_for_turn(pos);
        }

        PlayerRun {
            turns: self.turns,
            spaces_traversed: self.spaces_traversed,
            ladders_taken: self.ladders_taken,
            chutes_taken: self.chutes_taken,
        }
    }

    fn roll_dice(&mut self) -> u8 {
        self.rng.gen_range(1, 7)
    }

    fn move_for_turn(&mut self, current_pos: u8) -> u8 {
        let dice_roll = self.roll_dice();
        let mut new_pos = current_pos + dice_roll;

        // players are not allowed to overshoot to final space
        if new_pos > 100 {
            return current_pos;
        }

        self.spaces_traversed += dice_roll as u32;

        // if the player got to space 100, they've won
        if new_pos == 100 {
            return new_pos
        }

        // handle a warp
        let warp = self.config.warps[new_pos as usize];
        match warp {
            TransportType::Ladder(dest) => {
                self.ladders_taken += 1;
                new_pos = dest;
            },
            TransportType::Chute(dest) => {
                self.chutes_taken += 1;
                new_pos = dest;
            },
            TransportType::None => {}
        };

        new_pos
    }
}
