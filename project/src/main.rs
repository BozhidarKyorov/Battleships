use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect};
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
        // Initialize both boards with empty cells
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
                self.player_board[row][col] = CellState::Hit;
                self.is_player_turn = true;
                break;
            }
        }
    }
}

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.is_player_turn {
            self.computer_turn();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 255)); // Background color

        // Draw the player's board (on the left)
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = col as f32 * CELL_SIZE;
                let y = row as f32 * CELL_SIZE;

                let cell_color = match self.player_board[row][col] {
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
            }
        }

        // Draw the computer's board (on the right)
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = (col as f32 + GRID_SIZE as f32 + 2.0) * CELL_SIZE; // Offset to the right
                let y = row as f32 * CELL_SIZE;

                let cell_color = match self.computer_board[row][col] {
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
            }
        }

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
            let col = (x / CELL_SIZE) as usize;
            let row = (y / CELL_SIZE) as usize;

            if row < GRID_SIZE {
                if col < GRID_SIZE {
                    // Clicked on the player's board
                    if self.is_player_turn {
                        // Example: Perform action on the player's board
                        self.player_board[row][col] = CellState::Hit; // Just a demo
                        self.is_player_turn = false;
                    }
                } else if col >= GRID_SIZE + 2 && col < GRID_SIZE * 2 + 2 {
                    // Clicked on the computer's board
                    let computer_col = col - GRID_SIZE - 2;
                    if self.is_player_turn {
                        // Example: Perform action on the computer's board
                        self.computer_board[row][computer_col] = CellState::Miss; // Just a demo
                        self.is_player_turn = false;
                    }
                }
            }
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("battleship", "Author Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Battleship"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1600.0, 800.0))
        .build()?;

    let game = BattleshipGame::new();
    event::run(ctx, event_loop, game)
}
