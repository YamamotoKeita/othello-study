use crate::AiConfig;
use crate::evaluator::game_evaluator::GameEvaluator;
use crate::evaluator::simple_prediction::SimplePrediction;
use crate::model::ai_player::AiPlayer;
use crate::model::board::Board;
use crate::model::player_type::PlayerType;
use crate::model::points::xy_to_point;
use crate::searcher::nega_alpha::NegaAlpha;
use crate::searcher::Searcher;

pub struct GameInfo {
    ai_player1: Option<AiPlayer>,
    ai_player2: Option<AiPlayer>,
    pub board: Board,
    player: PlayerType,
}

impl GameInfo {
    pub fn new(ai_config1: Option<AiConfig>, ai_config2: Option<AiConfig>) -> GameInfo {
        let ai_player1 = GameInfo::make_ai_player(ai_config1);
        let ai_player2 = GameInfo::make_ai_player(ai_config2);

        GameInfo {
            ai_player1,
            ai_player2,
            board: Board::new(),
            player: PlayerType::First,
        }
    }

    fn make_ai_player(ai_config: Option<AiConfig>) -> Option<AiPlayer> {
        let config = match ai_config {
            Some(config)  => config,
            None => return None,
        };

        let game_evaluator = GameEvaluator::new(SimplePrediction::new());
        let searcher = NegaAlpha::new(game_evaluator);
        let searcher_box: Box<dyn Searcher + Send + 'static> = Box::new(searcher);
        Some(AiPlayer::new(searcher_box))
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

    pub fn play_ai(&mut self) -> bool {
        if !self.can_play_ai() {
            return false;
        }


        return true;
    }

    pub fn can_play_manually(&self) -> bool {
        return self.get_ai_player().is_none()
    }

    pub fn can_play_ai(&self) -> bool {
        return self.get_ai_player().is_some();
    }

    pub fn get_ai_player(&self) -> &Option<AiPlayer> {
        match self.player {
            PlayerType::First => &self.ai_player1,
            PlayerType::Second => &self.ai_player2,
            PlayerType::None => &None,
        }
    }
}