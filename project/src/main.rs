use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

use ggez::graphics::{Text, TextFragment, Font};
use ggez::mint;

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

        

        let border_color = Color::from_rgb(255, 255, 255); // White border color
        let label_color = Color::from_rgb(255, 255, 255);  // Label color

        let window_width = 1200.0; // Adjust based on your window dimensions
        let window_height = 800.0;
        let board_width = GRID_SIZE as f32 * CELL_SIZE;
        let spacing = 50.0; // Space between the two boards

        // Calculate positions to center the boards
        let total_boards_width = board_width * 2.0 + spacing;
        let start_x = (window_width - total_boards_width) / 2.0;
        let player_board_x = start_x;
        let computer_board_x = start_x + board_width + spacing;
        let boards_y = (window_height - board_width) / 2.0;

        // Draw player board
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = player_board_x + col as f32 * CELL_SIZE;
                let y = boards_y + row as f32 * CELL_SIZE;

                let cell_color = match self.player_board[row][col] {
                    CellState::Empty => Color::from_rgb(0, 128, 255), // Water
                    CellState::Hit => Color::from_rgb(255, 0, 0),     // Hit
                    CellState::Miss => Color::from_rgb(255, 255, 255), // Miss
                };

                // Draw filled cell
                let rectangle = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                    cell_color,
                )?;
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

                // Draw cell border
                let border = Mesh::new_rectangle(
                    ctx,
                    DrawMode::stroke(1.0),
                    Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                    border_color,
                )?;
                graphics::draw(ctx, &border, graphics::DrawParam::default())?;
            }
        }

        // Draw computer board
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = computer_board_x + col as f32 * CELL_SIZE;
                let y = boards_y + row as f32 * CELL_SIZE;

                let cell_color = match self.computer_board[row][col] {
                    CellState::Empty => Color::from_rgb(0, 128, 255), // Water
                    CellState::Hit => Color::from_rgb(255, 0, 0),     // Hit
                    CellState::Miss => Color::from_rgb(255, 255, 255), // Miss
                };

                // Draw filled cell
                let rectangle = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                    cell_color,
                )?;
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

                // Draw cell border
                let border = Mesh::new_rectangle(
                    ctx,
                    DrawMode::stroke(1.0),
                    Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                    border_color,
                )?;
                graphics::draw(ctx, &border, graphics::DrawParam::default())?;
            }
        }

        // Draw labels
        let text_scale = 32.0;

        let player_label = Text::new(
            TextFragment::new("Player")
                .color(Color::from_rgb(255, 255, 255)) // Optional: set text color
                .font(Font::default())
                .scale(text_scale),
        );
        
        // Create the "Computer" label
        let computer_label = Text::new(
            TextFragment::new("Computer")
                .color(Color::from_rgb(255, 255, 255)) // Optional: set text color
                .font(Font::default())
                .scale(text_scale),
        );


        // let player_label = Text::new(("Player", text_scale, graphics::Font::default()));
        // let computer_label = Text::new(("Computer", text_scale, graphics::Font::default()));

        let player_label_x = player_board_x + board_width / 2.0 - player_label.width(ctx) as f32 / 2.0;
        let computer_label_x = computer_board_x + board_width / 2.0 - computer_label.width(ctx) as f32 / 2.0;
        
        let boards = [(&self.player_board, player_board_x), (&self.computer_board, computer_board_x)];
        for (board, board_x) in boards.iter() {
            for row in 0..GRID_SIZE {
                for col in 0..GRID_SIZE {
                    let x = *board_x + col as f32 * CELL_SIZE;
                    let y = boards_y + row as f32 * CELL_SIZE;

                    let cell_color = match board[row][col] {
                        CellState::Empty => Color::from_rgb(0, 128, 255), // Water
                        CellState::Hit => Color::from_rgb(255, 0, 0),     // Hit
                        CellState::Miss => Color::from_rgb(255, 255, 255), // Miss
                    };

                    // Draw filled cell
                    let rectangle = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                        cell_color,
                    )?;
                    graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

                    // Draw cell border
                    let border = Mesh::new_rectangle(
                        ctx,
                        DrawMode::stroke(1.0), // Thin border
                        Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                        border_color,
                    )?;
                    graphics::draw(ctx, &border, graphics::DrawParam::default())?;
                }
            }
        }

        graphics::draw(
            ctx,
            &player_label,
            (mint::Point2 { x: player_label_x, y: boards_y - text_scale * 1.5 }, label_color),
        )?;
        graphics::draw(
            ctx,
            &computer_label,
            (mint::Point2 { x: computer_label_x, y: boards_y - text_scale * 1.5 }, label_color),
        )?;

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
            // Board dimensions
            let board_width = GRID_SIZE as f32 * CELL_SIZE;
            let spacing = 50.0; // Space between boards
            let window_width = 1200.0; // Match window size
            let window_height = 800.0;
    
            // Calculate the starting positions of the boards
            let total_boards_width = board_width * 2.0 + spacing;
            let start_x = (window_width - total_boards_width) / 2.0;
            let player_board_x = start_x;
            let computer_board_x = start_x + board_width + spacing;
            let boards_y = (window_height - board_width) / 2.0;
    
            // Check if the click is on the computer's board
            if x >= computer_board_x && x < computer_board_x + board_width
                && y >= boards_y && y < boards_y + board_width
            {
                let col = ((x - computer_board_x) / CELL_SIZE).floor() as usize;
                let row = ((y - boards_y) / CELL_SIZE).floor() as usize;
    
                // Update only if the cell is Empty
                if self.computer_board[row][col] == CellState::Empty {
                    self.computer_board[row][col] = CellState::Miss; // Mark as a miss
                } else {
                    println!("Cell already clicked!");
                }
    
                println!("Clicked on Computer's Board: Row {}, Col {}", row, col);
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


