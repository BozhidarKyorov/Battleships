use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, Text, TextFragment};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::mouse::MouseButton;
use rand::Rng;
use ggez::mint;

const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 40.0;
const WIN_CONDITION: usize = 17;

#[derive(PartialEq)]
enum GameState {
    StartScreen,
    ShipPlacement,
    Playing,
}

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Empty,
    Occupied, // New variant for placed ships
    Hit,
    Miss,
    Hovered,
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
    ships_to_place: Vec<ShipType>,
    selected_ship: Option<ShipType>,
    mouse_x: f32,
    mouse_y: f32,
    is_ship_horizontal: bool, // New field to track ship orientation
    total_player_hits: usize,
    total_computer_hits: usize,
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
            selected_ship: None,
            mouse_x: 0.0,
            mouse_y: 0.0,
            is_ship_horizontal: true, // Default to horizontal orientation
            total_player_hits: 0,
            total_computer_hits: 0,
        }
        
    }

    fn place_computer_ships(&mut self) {

        //println!("IN\n");
        let ships = vec![5, 4, 3, 3, 2,];
        //let mut not_placed = true;
        let mut gen = rand::thread_rng();
        


        for selected_ship in &ships {
            //not_placed = true;
            let mut can_place = false;
            let mut col = 0;
            let mut row = 0;
            let mut is_ship_horizontal = true;

            while !can_place {
                
                col = gen.gen_range(0..=GRID_SIZE);
                row = gen.gen_range(0..=GRID_SIZE);
                is_ship_horizontal = gen.gen_bool(0.5);

                //println!("{col} {row}'n");
                can_place = if is_ship_horizontal {
                    if col + selected_ship >= GRID_SIZE {
                        false
                    } else {
                        (0..*selected_ship).all(|i| (col + i) < 10 && row < 10 && self.computer_board[row][col + i] == CellState::Empty)
                    }
                } else {
                    if row + selected_ship >= GRID_SIZE {
                        false
                    } else {
                        (0..*selected_ship).all(|i| (row + i) < 10 && row < 10 && self.computer_board[row + i][col] == CellState::Empty)
                    }
                };
            }
            
            
            if is_ship_horizontal {
                for i in 0..*selected_ship {
                    self.computer_board[row][col + i] = CellState::Occupied; // Mark as occupied
                }
            } else {
                for i in 0..*selected_ship {
                    self.computer_board[row + i][col] = CellState::Occupied; // Mark as occupied
                }
            }

                // // Remove the ship from the list of ships to place
                // self.ships_to_place.retain(|&s| s != *selected_ship);

                // // Reset the selected ship after placement
                // self.selected_ship = None;
            
        
        }
        
    }

    fn computer_turn(&mut self) {
        let mut rng = rand::thread_rng();
        let mut target_row = rng.gen_range(0..GRID_SIZE);
        let mut target_col = rng.gen_range(0..GRID_SIZE);
    
        // Simple logic: if a hit is found, target adjacent cells
        // for row in 0..GRID_SIZE {
        //     for col in 0..GRID_SIZE {
        //         if self.player_board[row][col] == CellState::Hit {
        //             // Target adjacent cells
        //             let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        //             for (dx, dy) in directions.iter() {
        //                 let new_row = row as isize + dx;
        //                 let new_col = col as isize + dy;
        //                 if new_row >= 0 && new_row < GRID_SIZE as isize && new_col >= 0 && new_col < GRID_SIZE as isize {
        //                     target_row = new_row as usize;
        //                     target_col = new_col as usize;
        //                     break;
        //                 }
        //             }
        //         }
        //     }
        // }
    
        if self.player_board[target_row][target_col] == CellState::Empty {
            self.player_board[target_row][target_col] = CellState::Miss; 
        }
        else if self.player_board[target_row][target_col] == CellState::Occupied {
            self.player_board[target_row][target_col] = CellState::Hit; 
            self.total_computer_hits += 1;
        }
        else {
             self.is_player_turn = !self.is_player_turn;
        }

        self.is_player_turn = !self.is_player_turn;
        //println!("{target_row} {target_col}");

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

    fn draw_game_screen(&self, ctx: &mut Context) -> GameResult {
        let border_color = Color::from_rgb(255, 255, 255);

        let (player_board_x, computer_board_x, boards_y) = self.calculate_positions();

        self.draw_board(ctx, &self.player_board, player_board_x, boards_y, border_color, true)?;
        self.draw_board(ctx, &self.computer_board, computer_board_x, boards_y, border_color, false)?;

        self.draw_labels(ctx, player_board_x, computer_board_x, boards_y)?;

        // Draw ship lists
        self.draw_ship_list(ctx, player_board_x - 250.0, boards_y, true)?; // Player's ships on the left
        self.draw_ship_list(ctx, computer_board_x + GRID_SIZE as f32 * CELL_SIZE + 50.0, boards_y, false)?; // Computer's ships on the right

        Ok(())
    }

    fn draw_ship_list(&self, ctx: &mut Context, x: f32, y: f32, is_player: bool) -> GameResult {
        let ships = if is_player {
            vec![
                ShipType::Carrier,
                ShipType::Battleship,
                ShipType::Cruiser,
                ShipType::Submarine,
                ShipType::Destroyer,
            ]
        } else {
            vec![
                ShipType::Carrier,
                ShipType::Battleship,
                ShipType::Cruiser,
                ShipType::Submarine,
                ShipType::Destroyer,
            ]
        };

        let mut y_offset = y;
        for ship in ships {
            let ship_text = Text::new(
                TextFragment::new(format!("{:?} (Size: {})", ship, ship.size()))
                    .color(Color::from_rgb(255, 255, 255))
                    .scale(24.0),
            );

            graphics::draw(ctx, &ship_text, graphics::DrawParam::default().dest([x, y_offset]))?;

            // Draw ship model below the text
            let model_x = x;
            let model_y = y_offset + 30.0;
            let model_width = CELL_SIZE * ship.size() as f32;
            let model_height = CELL_SIZE / 2.0;

            let ship_model = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(model_x, model_y, model_width, model_height),
                Color::from_rgb(100, 100, 100),
            )?;

            graphics::draw(ctx, &ship_model, graphics::DrawParam::default())?;

            y_offset += 80.0;
        }

        Ok(())
    }

    fn calculate_positions(&self) -> (f32, f32, f32) {
        let window_width = 1600.0; // Updated window width
        let window_height = 1000.0; // Updated window height
        let board_width = GRID_SIZE as f32 * CELL_SIZE;
        let spacing = 100.0; // Increased spacing for better layout

        // Calculate the starting x position for the player's board
        let player_board_x = (window_width / 2.0) - board_width - spacing;

        // Calculate the starting x position for the computer's board
        let computer_board_x = (window_width / 2.0) + spacing;

        // Calculate the y position for both boards (centered vertically)
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
        is_player_board: bool,
    ) -> GameResult {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = x_offset + col as f32 * CELL_SIZE;
                let y = y_offset + row as f32 * CELL_SIZE;
                let cell_color:Color;

                
                if is_player_board{
                    cell_color = match board[row][col] {
                        CellState::Empty => Color::from_rgb(0, 128, 255), // Water
                        CellState::Occupied => Color::from_rgb(100, 100, 100), // Dark grey for occupied cells
                        CellState::Hit => Color::from_rgb(255, 0, 0),     // Hit
                        CellState::Miss => Color::from_rgb(255, 255, 255), // Miss
                        CellState::Hovered => Color::from_rgb(200, 200, 200), 
                    };
                }
                else {
                    cell_color = match board[row][col] {
                        CellState::Empty => Color::from_rgb(0, 128, 255), // Water
                        CellState::Occupied => Color::from_rgb(0, 128, 255), // Dark grey for occupied cells
                        CellState::Hit => Color::from_rgb(255, 0, 0),     // Hit
                        CellState::Miss => Color::from_rgb(255, 255, 255), // Miss
                        CellState::Hovered => Color::from_rgb(200, 200, 200), 
                    };
                }
                
    
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
        selected_ship: &mut Option<ShipType>,
    ) -> GameResult {
        let instruction = graphics::Text::new(("Place your ships", graphics::Font::default(), 32.0));
        graphics::draw(ctx, &instruction, (ggez::mint::Point2 { x: 20.0, y: 20.0 }, Color::WHITE))?;
    
        // Draw the player's board
        let player_board_x = 100.0;
        let player_board_y = 100.0;
        self.draw_board(ctx, &self.player_board, player_board_x, player_board_y, Color::WHITE, true)?;
    
        // Highlight cells where the ship would be placed
        if let Some(ship) = selected_ship {
            let col = ((self.mouse_x - player_board_x) / CELL_SIZE).floor() as usize;
            let row = ((self.mouse_y - player_board_y) / CELL_SIZE).floor() as usize;
    
            if self.is_ship_horizontal {
                if col + ship.size() <= GRID_SIZE && row < GRID_SIZE {
                    for i in 0..ship.size() {
                        let x = player_board_x + (col + i) as f32 * CELL_SIZE;
                        let y = player_board_y + row as f32 * CELL_SIZE;
                        let highlight = Mesh::new_rectangle(
                            ctx,
                            DrawMode::fill(),
                            Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                            Color::from_rgba(200, 200, 200, 128), // Light grey with transparency
                        )?;
                        graphics::draw(ctx, &highlight, graphics::DrawParam::default())?;
                    }
                }
            } else {
                if row + ship.size() <= GRID_SIZE && col < GRID_SIZE {
                    for i in 0..ship.size() {
                        let x = player_board_x + col as f32 * CELL_SIZE;
                        let y = player_board_y + (row + i) as f32 * CELL_SIZE;
                        let highlight = Mesh::new_rectangle(
                            ctx,
                            DrawMode::fill(),
                            Rect::new(x, y, CELL_SIZE, CELL_SIZE),
                            Color::from_rgba(200, 200, 200, 128), // Light grey with transparency
                        )?;
                        graphics::draw(ctx, &highlight, graphics::DrawParam::default())?;
                    }
                }
            }
        }
    
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
            // if !self.ships_to_place.contains(ship) {
            //     let cross_line = Mesh::new_line(
            //         ctx,
            //         &[
            //             ggez::mint::Point2 { x: model_x, y: model_y },
            //             ggez::mint::Point2 { x: model_x + model_width, y: model_y + model_height },
            //         ],
            //         2.0,
            //         Color::from_rgb(255, 0, 0),
            //     )?;
            //     graphics::draw(ctx, &cross_line, graphics::DrawParam::default())?;
            // }
    
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
    
    /////////////////////////////////////////////////////////////////////
    
    // fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
    //     self.mouse_x = x;
    //     self.mouse_y = y;
    // }

    fn check_for_winner(&mut self, ctx: &mut Context) {
        if self.total_player_hits == 17 {
            //GO to win screne
            println!("WIN");
            ggez::event::quit(ctx);
        }
        else if self.total_computer_hits == 17{
            //Go to lose screen
            println!("LOSE");
            ggez::event::quit(ctx);
        }
        

    }

}

impl EventHandler for BattleshipGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_state == GameState::Playing && !self.is_player_turn {
            self.computer_turn();
            self.check_for_winner(ctx);
            //self.is_player_turn = true;
        }
        Ok(())
    }

   

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
        y: f32,
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
                    self.computer_board[row][col] = CellState::Miss; 
                }
                else if self.computer_board[row][col] == CellState::Occupied {
                    self.computer_board[row][col] = CellState::Hit; 
                    self.total_player_hits += 1;
                }
                else {
                    self.is_player_turn = !self.is_player_turn;
                }
    
                self.is_player_turn = !self.is_player_turn;
            }
        }
    
        if self.game_state == GameState::ShipPlacement {
            if button == MouseButton::Right {
                // Toggle ship orientation on right-click
                if self.selected_ship.is_some() {
                    self.is_ship_horizontal = !self.is_ship_horizontal;
                }
            } else if button == MouseButton::Left {
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
    
                    // Check if the ship can be placed
                    let can_place = if self.is_ship_horizontal {
                        if col + selected_ship.size() > GRID_SIZE {
                            false
                        } else {
                            (0..selected_ship.size()).all(|i| self.player_board[row][col + i] == CellState::Empty)
                        }
                    } else {
                        if row + selected_ship.size() > GRID_SIZE {
                            false
                        } else {
                            (0..selected_ship.size()).all(|i| self.player_board[row + i][col] == CellState::Empty)
                        }
                    };
    
                    if can_place {
                        if self.is_ship_horizontal {
                            for i in 0..selected_ship.size() {
                                self.player_board[row][col + i] = CellState::Occupied; // Mark as occupied
                            }
                        } else {
                            for i in 0..selected_ship.size() {
                                self.player_board[row + i][col] = CellState::Occupied; // Mark as occupied
                            }
                        }
    
                        // Remove the ship from the list of ships to place
                        self.ships_to_place.retain(|&s| s != *selected_ship);
    
                        // Reset the selected ship after placement
                        self.selected_ship = None;
                    }
                }
    
                // Check if the "Continue" button is clicked
                if self.ships_to_place.is_empty() {
                    let button_x = 500.0;
                    let button_y = 700.0;
                    let button_width = 200.0;
                    let button_height = 50.0;
    
                    if x >= button_x && x <= button_x + button_width
                        && y >= button_y && y <= button_y + button_height
                    {
                        self.place_computer_ships();
                        self.game_state = GameState::Playing;
                    }
                }
            }
        }
    }

    // fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
    //     self.mouse_x = x;
    //     self.mouse_y = y;

    //     // let board_x = 100.0;
    //     // let board_y = 100.0;

    //     // let col = ((x - board_x) / CELL_SIZE).floor() as usize;
    //     // let row = ((y - board_y) / CELL_SIZE).floor() as usize;


    //     // for r in 0..10 {
    //     //     for c in 0..10 {
    //     //         if self.player_board[r][c] != CellState::Hovered {
    //     //             old_board[r][c] = self.player_board[r][c];
    //     //         }
    //     //     }
    //     // }

    //     // if self.game_state == GameState::ShipPlacement && col < 10 && row < 10  {

    //     //     for r in 0..10 {
    //     //         for c in 0..10 {
    //     //             if self.player_board[r][c] == CellState::Hovered {
    //     //                 self.player_board[r][c] = old_board[r][c]; // Reset to empty or original
    //     //             }
    //     //         }
    //     //     }

    //     //     self.player_board[row][col] = CellState::Hovered;

           

    //     // }
        
    //     //println!("{x} {y}\n");
    // }

    
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("battleship", "Author Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Battleship"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1600.0, 1000.0))
        .build()?;

    let game = BattleshipGame::new();
    event::run(ctx, event_loop, game)
}