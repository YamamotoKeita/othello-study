use crate::bridge::ai_config::AiConfig;
use crate::bridge::point::{Point, to_point};
use crate::evaluator::simple_prediction::SimplePrediction;
use crate::GameResponse;
use crate::model::ai_player::AiPlayer;
use crate::model::board::Board;
use crate::model::player::Player;
use crate::model::player_type::PlayerType;
use crate::model::points::xy_to_point;
use crate::searcher::nega_alpha::NegaAlpha;
use crate::searcher::Searcher;

pub struct GameInfo {
    ai_player1: Option<AiPlayer>,
    ai_player2: Option<AiPlayer>,
    pub board: Board,
    pub player: PlayerType,
}

impl GameInfo {
    pub fn new() -> GameInfo {
        GameInfo {
            ai_player1: None,
            ai_player2: None,
            board: Board::new(),
            player: PlayerType::First,
        }
    }

    pub fn init(&mut self, ai_config1: AiConfig, ai_config2: AiConfig) {
        self.ai_player1 = GameInfo::make_ai_player(ai_config1);
        self.ai_player2 = GameInfo::make_ai_player(ai_config2);
        self.board = Board::new();
        self.player = PlayerType::First;
    }

    fn make_ai_player(ai_config: AiConfig) -> Option<AiPlayer> {
        if !ai_config.is_ai {
            return None;
        }

        let evaluator = SimplePrediction::new();
        let searcher = NegaAlpha::new(evaluator);
        let searcher_box: Box<dyn Searcher + Send + 'static> = Box::new(searcher);
        Some(AiPlayer::new(searcher_box))
    }

    pub fn get_player_move(&mut self, x: u32, y: u32) -> Option<Point> {
        let point = xy_to_point(x, y);
        if !self.board.can_place(point) {
            return None;
        }
        if !self.can_play_manually() {
            return None;
        }

        return Some(Point{x, y});
    }

    pub fn get_ai_move(&self) -> Option<Point> {
        if let Some(ai) = self.get_ai_player() {
            let point = to_point(ai.play(&self.board));
            return Some(point);
        }

        return None;
    }

    pub fn play(&mut self, point: Point) -> GameResponse {
        let points = xy_to_point(point.x, point.y);
        let board = self.board;
        let player = self.player;

        let mut new_board = self.board.place_stone(points);
        self.player = self.player.opposite();

        if new_board.placeable_points == 0 {
            new_board = new_board.skip_turn();
            self.player = self.player.opposite();
        }

        self.board = new_board;

        GameResponse::new_by_move(
            new_board,
            board,
            player,
            point,
        )
    }

    pub fn can_play_manually(&self) -> bool {
        return self.get_ai_player().is_none()
    }

    pub fn get_ai_player(&self) -> &Option<AiPlayer> {
        match self.player {
            PlayerType::First => &self.ai_player1,
            PlayerType::Second => &self.ai_player2,
            PlayerType::None => &None,
        }
    }
}