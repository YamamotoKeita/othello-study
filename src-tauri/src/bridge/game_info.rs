use crate::AiConfig;
use crate::model::board::Board;
use crate::model::player_type::PlayerType;
use crate::model::points::xy_to_point;

#[derive(Clone, Copy)]
pub struct GameInfo {
    ai_config1: Option<AiConfig>,
    ai_config2: Option<AiConfig>,
    pub board: Board,
    player: PlayerType,
}

impl GameInfo {
    pub fn new(ai_config1: Option<AiConfig>, ai_config2: Option<AiConfig>) -> GameInfo {
        GameInfo {
            ai_config1,
            ai_config2,
            board: Board::new(),
            player: PlayerType::First,
        }
    }

    pub fn click(&mut self, x: u32, y: u32) -> bool {
        let point = xy_to_point(x, y);
        if !self.board.can_place(point) {
            return false;
        }
        if !self.can_play_manually() {
            return false;
        }

        let mut new_board = self.board.place_stone(point);
        self.player = self.player.opposite();

        if new_board.placeable_points == 0 {
            new_board = new_board.skip_turn();
            self.player = self.player.opposite();
        }

        self.board = new_board;

        return true;
    }

    fn can_play_manually(&self) -> bool {
        return self.get_ai_config().is_none()
    }

    fn can_play_ai(&self) -> bool {
        return self.get_ai_config().is_some();
    }

    pub fn get_ai_config(&self) -> Option<AiConfig> {
        match self.player {
            PlayerType::First => self.ai_config1,
            PlayerType::Second => self.ai_config2,
            PlayerType::None =>None,
        }
    }
}