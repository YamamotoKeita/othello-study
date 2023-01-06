use crate::model::board::Board;
use crate::model::player_type::PlayerType;
use crate::model::points::Points;

pub trait Player {
    fn play(&self, board: &Board) -> Points;
    fn message_before_play(&self, board: &Board) -> Option<String>;
}

pub trait OthelloView {
    fn wait_next_move(&self, board: &Board);
    fn send_message(&self, text: String);
    fn place_stone(&self, point: Points, before: &Board, after: &Board);
    fn skipped(&self, player: PlayerType);
    fn game_end(&self, board: &Board);
}

pub struct GameManager {
    first_player: Box<dyn Player + Send + 'static>,
    second_player: Box<dyn Player + Send + 'static>,
    view: Box<dyn OthelloView + Send + 'static>,
}

impl GameManager {
    pub fn new(first_player: Box<dyn Player + Send + 'static>, second_player: Box<dyn Player + Send + 'static>, view: Box<dyn OthelloView + Send + 'static>) -> GameManager {
        GameManager {
            first_player,
            second_player,
            view,
        }
    }

    pub fn start_game(&mut self) {
        let mut board = Board::new();

        loop {
            self.view.wait_next_move(&board);

            // Place a stone
            let player = self.get_player(board.player);

            let message = player.message_before_play(&board);
            if let Some(message) = message {
                self.view.send_message(message);
            }

            let point = player.play(&board);
            let mut new_board = board.place_stone(point);
            self.view.place_stone(point, &board, &new_board);

            if new_board.placeable_points == 0 {
                new_board = new_board.skip_turn();

                if new_board.is_game_end() {
                    break;
                } else {
                    self.view.skipped(board.player.opposite());
                }
            }

            board = new_board;
        }

        self.view.game_end(&board);
    }

    fn get_player(&self, player: PlayerType) -> &dyn Player {
        match player {
            PlayerType::First => self.first_player.as_ref(),
            PlayerType::Second => self.second_player.as_ref(),
            PlayerType::None => panic!("Use a player when there is no player."),
        }
    }
}
