use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, Text, TextFragment};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::mouse::MouseButton;
use rand::Rng;
use ggez::mint;

const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 40.0;

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Empty,
    Hit,
    Miss,
}

#[derive(PartialEq)]
enum GameState {
    StartScreen,
    ShipPlacement,
    Playing,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum ShipType {
    Destroyer, // Size 2
    Submarine, // Size 3
    Cruiser,   // Size 3
    Battleship, // Size 4
    Carrier,   // Size 5
}


impl ShipType {
    fn size(&self) -> usize {
        match self {
            ShipType::Destroyer => 2,
            ShipType::Submarine => 3,
            ShipType::Cruiser => 3,
            ShipType::Battleship => 4,
            ShipType::Carrier => 5,
        }
    }
}

struct BattleshipGame {
    player_board: Vec<Vec<CellState>>,
    computer_board: Vec<Vec<CellState>>,
    is_player_turn: bool,
    game_state: GameState,
    ships_to_place: Vec<ShipType>, // Assuming you have a `ShipType` enum or struct
    selected_ship: Option<ShipType>, // This tracks the selected ship
}

impl BattleshipGame {
    fn new() -> Self {
        Self {
            player_board: vec![vec![CellState::Empty; GRID_SIZE]; GRID_SIZE],
            computer_board: vec![vec![CellState::Empty; GRID_SIZE]; GRID_SIZE],
            is_player_turn: true,
            game_state: GameState::StartScreen,
            ships_to_place: vec![
                ShipType::Carrier,
                ShipType::Battleship,
                ShipType::Cruiser,
                ShipType::Submarine,
                ShipType::Destroyer,
            ],
            selected_ship: None, // Initialize selected_ship to None
        }
    }

    fn computer_turn(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let row = rng.gen_range(0..GRID_SIZE);
            let col = rng.gen_range(0..GRID_SIZE);
            if self.player_board[row][col] == CellState::Empty {
                self.player_board[row][col] = CellState::Miss; // Mark as miss for simplicity
                break;
            }
        }
    }

    fn draw_start_screen(&self, ctx: &mut Context) -> GameResult {
        //graphics::clear(ctx, Color::from_rgb(0, 0, 180)); // Black background

        let title_text = Text::new(
            TextFragment::new("Battleship")
                .color(Color::from_rgb(255, 255, 255))
                .scale(64.0),
        );

        let start_text = Text::new(
            TextFragment::new("Start")
                .color(Color::from_rgb(0, 255, 0))
                .scale(48.0),
        );

        let exit_text = Text::new(
            TextFragment::new("Exit")
                .color(Color::from_rgb(255, 0, 0))
                .scale(48.0),
        );

        let (window_width, window_height) = (1200.0, 800.0);
        let title_x = (window_width - title_text.width(ctx) as f32) / 2.0;
        let title_y = 100.0;

        let start_x = (window_width - start_text.width(ctx) as f32) / 2.0;
        let start_y = 300.0;

        let exit_x = (window_width - exit_text.width(ctx) as f32) / 2.0;
        let exit_y = 400.0;

        graphics::draw(ctx, &title_text, graphics::DrawParam::default().dest([title_x, title_y]))?;
        graphics::draw(ctx, &start_text, graphics::DrawParam::default().dest([start_x, start_y]))?;
        graphics::draw(ctx, &exit_text, graphics::DrawParam::default().dest([exit_x, exit_y]))?;

        Ok(())
    }

    fn draw_game_screen (&self, ctx: &mut Context) -> GameResult {
        //graphics::clear(ctx, Color::from_rgb(0, 0, 180)); // Background color
        let border_color = Color::from_rgb(255, 255, 255);

        let (player_board_x, computer_board_x, boards_y) = self.calculate_positions();

        self.draw_board(ctx, &self.player_board, player_board_x, boards_y, border_color)?;
        self.draw_board(ctx, &self.computer_board, computer_board_x, boards_y, border_color)?;

        self.draw_labels(ctx, player_board_x, computer_board_x, boards_y)?;

        Ok(())
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
            TextFragment::new("Computer")
                .color(label_color)
                .scale(32.0),
        );

        let player_label_x = player_board_x + (GRID_SIZE as f32 * CELL_SIZE) / 2.0 - player_label.width(ctx) as f32 / 2.0;
        let computer_label_x = computer_board_x + (GRID_SIZE as f32 * CELL_SIZE) / 2.0 - computer_label.width(ctx) as f32 / 2.0;

        graphics::draw(ctx, &player_label, graphics::DrawParam::default().dest([player_label_x, boards_y - 40.0]))?;
        graphics::draw(ctx, &computer_label, graphics::DrawParam::default().dest([computer_label_x, boards_y - 40.0]))?;

        Ok(())
    }

    fn draw_ship_placement_screen(
        &self,
        ctx: &mut Context,
        selected_ship: &mut Option<ShipType>, // Pass selected ship as a mutable reference
    ) -> GameResult {
        let instruction = graphics::Text::new(("Place your ships", graphics::Font::default(), 32.0));
        graphics::draw(ctx, &instruction, (ggez::mint::Point2 { x: 20.0, y: 20.0 }, Color::WHITE))?;
    
        // Draw the player's board
        let player_board_x = 100.0;
        let player_board_y = 100.0;
        self.draw_board(ctx, &self.player_board, player_board_x, player_board_y, Color::WHITE)?;
    
        // Draw ship buttons and models
        let ships_x = player_board_x + GRID_SIZE as f32 * CELL_SIZE + 50.0;
        let mut ships_y = player_board_y;
    
        for ship in &self.ships_to_place {
            let ship_text = Text::new(
                TextFragment::new(format!("{:?} (Size: {})", ship, ship.size()))
                    .color(Color::from_rgb(255, 255, 255))
                    .scale(24.0),
            );
    
            graphics::draw(ctx, &ship_text, graphics::DrawParam::default().dest([ships_x, ships_y]))?;
    
            // Draw ship model below the text
            let model_x = ships_x;
            let model_y = ships_y + 30.0;
            let model_width = CELL_SIZE * ship.size() as f32;
            let model_height = CELL_SIZE / 2.0;
    
            let ship_model = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(model_x, model_y, model_width, model_height),
                if Some(*ship) == *selected_ship {
                    Color::from_rgb(0, 255, 0) // Highlight selected ship
                } else {
                    Color::from_rgb(100, 100, 100)
                },
            )?;
    
            graphics::draw(ctx, &ship_model, graphics::DrawParam::default())?;
    
            // Cross out placed ships
            if !self.ships_to_place.contains(ship) {
                let cross_line = Mesh::new_line(
                    ctx,
                    &[
                        ggez::mint::Point2 { x: model_x, y: model_y },
                        ggez::mint::Point2 { x: model_x + model_width, y: model_y + model_height },
                    ],
                    2.0,
                    Color::from_rgb(255, 0, 0),
                )?;
                graphics::draw(ctx, &cross_line, graphics::DrawParam::default())?;
            }
    
            ships_y += 80.0;
        }
    
        // Draw "Continue" button
        if self.ships_to_place.is_empty() {
            let button_text = Text::new(
                TextFragment::new("Continue")
                    .color(Color::from_rgb(0, 255, 0))
                    .scale(48.0),
            );
    
            let button_x = 500.0;
            let button_y = 700.0;
    
            graphics::draw(ctx, &button_text, graphics::DrawParam::default().dest([button_x, button_y]))?;
        }
    
        Ok(())
    }
    
    
    
    // fn draw_ships_to_place(&self, ctx: &mut Context, x_offset: f32, y_offset: f32) -> GameResult {
    //     let mut y_position = y_offset;

    //     for &ship_size in &self.ships_to_place {
    //         for i in 0..ship_size {
    //             let x = x_offset + i as f32 * CELL_SIZE;
    //             let y = y_position;

    //             let rectangle = Mesh::new_rectangle(
    //                 ctx,
    //                 DrawMode::fill(),
    //                 Rect::new(x, y, CELL_SIZE, CELL_SIZE),
    //                 Color::from_rgb(0, 255, 0), // Green for unplaced ships
    //             )?;
    //             graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
    //         }

    //         // Add spacing between ships
    //         y_position += CELL_SIZE * 1.5;
    //     }

    //     Ok(())
    // }

}

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.game_state == GameState::Playing && !self.is_player_turn {
            self.computer_turn();
            self.is_player_turn = true;
        }
        Ok(())
    }

    // fn draw(&mut self, ctx: &mut Context) -> GameResult {
    //     match self.game_state {
    //         GameState::StartScreen => self.draw_start_screen(ctx),
    //         GameState::Playing => self.draw_game_screen(ctx),
    //     }
    // }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen only once per frame
        graphics::clear(ctx, Color::from_rgb(0, 0, 255)); // Background color
    
        let mut selected_ship: Option<ShipType> = None;
        // Based on the game state, draw the correct screen
        match self.game_state {
            GameState::StartScreen => {
                self.draw_start_screen(ctx)?;  // Draw the Start Screen
            }
            GameState::ShipPlacement => {
                self.draw_ship_placement_screen(ctx, &mut selected_ship)?;  // Draw Ship Placement Screen
            }
            GameState::Playing => {
                self.draw_game_screen(ctx)?;  // Draw the Playing screen (the game itself)
            }
        }
    
        // Present the drawn content to the screen
        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self, 
        _ctx: &mut Context, 
        button: MouseButton, 
        x: f32, 
        y: f32
    ) {
        if self.game_state == GameState::StartScreen && button == MouseButton::Left {
            let (window_width, _window_height) = (1200.0, 800.0);
            let start_x = (window_width - 200.0) / 2.0; // Approximate button width
            let start_y = 300.0;
            let exit_y = 400.0;
    
            if x >= start_x && x <= start_x + 200.0 && y >= start_y && y <= start_y + 50.0 {
                self.game_state = GameState::ShipPlacement; // Start game
            } else if x >= start_x && x <= start_x + 200.0 && y >= exit_y && y <= exit_y + 50.0 {
                std::process::exit(0); // Exit game
            }
        }
    
        if self.game_state == GameState::Playing && button == MouseButton::Left {
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
    
        if self.game_state == GameState::ShipPlacement && button == MouseButton::Left {
            let player_board_x = 100.0;
            let player_board_y = 100.0;
            let board_width = GRID_SIZE as f32 * CELL_SIZE;
    
            let ships_x = player_board_x + board_width + 50.0;
            let mut ships_y = player_board_y;
    
            // Check if a ship is selected
            for ship in self.ships_to_place.iter() {
                let model_x = ships_x;
                let model_y = ships_y + 30.0;
                let model_width = CELL_SIZE * ship.size() as f32;
                let model_height = CELL_SIZE / 2.0;
    
                if x >= model_x && x <= model_x + model_width
                    && y >= model_y && y <= model_y + model_height
                {
                    self.selected_ship = Some(*ship); // Set selected_ship here
                    return;
                }
    
                ships_y += 80.0;
            }
    
            // Place ship on the board
            if let Some(selected_ship) = &self.selected_ship {
                let col = ((x - player_board_x) / CELL_SIZE).floor() as usize;
                let row = ((y - player_board_y) / CELL_SIZE).floor() as usize;
    
                if row + selected_ship.size() <= GRID_SIZE && col < GRID_SIZE {
                    let can_place = (0..selected_ship.size()).all(|i| self.player_board[row + i][col] == CellState::Empty);
    
                    if can_place {
                        for i in 0..selected_ship.size() {
                            self.player_board[row + i][col] = CellState::Hit;
                        }
                        self.ships_to_place.retain(|&s| s != *selected_ship);
                    }
                }
    
                // Reset the selected ship after placement
                self.selected_ship = None;
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
