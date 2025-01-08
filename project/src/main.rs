use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, Text, TextFragment};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 40.0; // Each cell is 40x40 pixels

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Empty,
    Hit,
    Miss,
}

struct BattleshipGame {
    player_board: Vec<Vec<CellState>>,  // Player's board
    computer_board: Vec<Vec<CellState>>, // Computer's board
    is_player_turn: bool,               // Tracks whose turn it is
}

impl BattleshipGame {
    fn new() -> Self {
        Self {
            player_board: vec![vec![CellState::Empty; GRID_SIZE]; GRID_SIZE],
            computer_board: vec![vec![CellState::Empty; GRID_SIZE]; GRID_SIZE],
            is_player_turn: true,
        }
    }

    fn computer_turn(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let row = rng.gen_range(0..GRID_SIZE);
            let col = rng.gen_range(0..GRID_SIZE);
            if self.player_board[row][col] == CellState::Empty {
                self.player_board[row][col] = CellState::Miss; // For simplicity, mark as Miss
                break;
            }
        }
    }

    fn calculate_positions(&self) -> (f32, f32, f32) {
        let window_width = 1200.0;
        let window_height = 800.0;
        let board_width = GRID_SIZE as f32 * CELL_SIZE;
        let spacing = 50.0;

        let total_boards_width = board_width * 2.0 + spacing;
        let start_x = (window_width - total_boards_width) / 2.0;
        let player_board_x = start_x;
        let computer_board_x = start_x + board_width + spacing;
        let boards_y = (window_height - board_width) / 2.0;

        (player_board_x, computer_board_x, boards_y)
    }

    fn draw_board(
        &self,
        ctx: &mut Context,
        board: &Vec<Vec<CellState>>,
        x_offset: f32,
        y_offset: f32,
        border_color: Color,
    ) -> GameResult {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = x_offset + col as f32 * CELL_SIZE;
                let y = y_offset + row as f32 * CELL_SIZE;

                let cell_color = match board[row][col] {
                    CellState::Empty => Color::from_rgb(0, 128, 255), // Water
                    CellState::Hit => Color::from_rgb(255, 0, 0),     // Hit
                    CellState::Miss => Color::from_rgb(255, 255, 255), // Miss
                };

                let rectangle = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                    cell_color,
                )?;
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

                let border = Mesh::new_rectangle(
                    ctx,
                    DrawMode::stroke(1.0),
                    Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                    border_color,
                )?;
                graphics::draw(ctx, &border, graphics::DrawParam::default())?;
            }
        }
        Ok(())
    }

    fn draw_labels(&self, ctx: &mut Context, player_board_x: f32, computer_board_x: f32, boards_y: f32) -> GameResult {
        let label_color = Color::from_rgb(255, 255, 255);

        let player_label = Text::new(
            TextFragment::new("Player")
                .color(label_color)
                .scale(32.0),
        );

        let computer_label = Text::new(
            TextFragment::new("Enemy")
                .color(label_color)
                .scale(32.0),
        );

        let player_label_x = player_board_x + (GRID_SIZE as f32 * CELL_SIZE) / 2.0 - player_label.width(ctx) as f32 / 2.0;
        let computer_label_x = computer_board_x + (GRID_SIZE as f32 * CELL_SIZE) / 2.0 - computer_label.width(ctx) as f32 / 2.0;

        graphics::draw(ctx, &player_label, graphics::DrawParam::default().dest([player_label_x, boards_y - 40.0]))?;
        graphics::draw(ctx, &computer_label, graphics::DrawParam::default().dest([computer_label_x, boards_y - 40.0]))?;

        Ok(())
    }
}

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.is_player_turn {
            self.computer_turn();
            self.is_player_turn = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 255)); // Background color
        let border_color = Color::from_rgb(255, 255, 255);

        let (player_board_x, computer_board_x, boards_y) = self.calculate_positions();

        self.draw_board(ctx, &self.player_board, player_board_x, boards_y, border_color)?;
        self.draw_board(ctx, &self.computer_board, computer_board_x, boards_y, border_color)?;

        self.draw_labels(ctx, player_board_x, computer_board_x, boards_y)?;

        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == ggez::input::mouse::MouseButton::Left {
            let (player_board_x, computer_board_x, boards_y) = self.calculate_positions();
            let board_width = GRID_SIZE as f32 * CELL_SIZE;

            if x >= computer_board_x && x < computer_board_x + board_width
                && y >= boards_y && y < boards_y + board_width
            {
                let col = ((x - computer_board_x) / CELL_SIZE).floor() as usize;
                let row = ((y - boards_y) / CELL_SIZE).floor() as usize;

                if self.computer_board[row][col] == CellState::Empty {
                    self.computer_board[row][col] = CellState::Miss; // Mark as miss for simplicity
                }

                self.is_player_turn = false;
            }
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("battleship", "Author Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Battleship"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1200.0, 800.0))
        .build()?;

    let game = BattleshipGame::new();
    event::run(ctx, event_loop, game)
}