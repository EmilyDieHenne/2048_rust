use macroquad::prelude::* ;
use ::rand::Rng;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Board  {
    tiles: Vec<Vec<i32>>,
    score: i32,
    game_over: bool
}
impl Board {
    fn player_move(&mut self, direction:Direction) {
        let has_moved = match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        };
        if !has_moved {
            return;
        }

        self.set_tile();
        if !self.has_legal_move() {
            self.game_over = true;
        }
    }
    fn has_moved(&mut self, new_tiles: Vec<Vec<i32>>) -> bool {

        if self.tiles == new_tiles {
            return false;
        }
        self.tiles = new_tiles;
        return true;
    }
    fn move_right(&mut self ) -> bool {
        let mut new_tiles = vec![vec![0;4];4]; 

        for y in 0..4{ 
            let mut previous_tile : i32 = 0;
            let mut position = 3;
            for x in 0..4{ 
                let tile :i32= self.tiles[3 - x][y];

                if tile == 0 {
                    continue;
                }
                if tile != previous_tile {
                    new_tiles[position][y] = tile;
                    if position != 0 {
                        position -= 1;
                    }
                    previous_tile = tile;
                }
                else{
                    self.score += 2i32.pow(tile as u32+ 1);

                    new_tiles[position +1 ][y] = tile + 1;
                    previous_tile = 0;
                }

            }

        }

        return self.has_moved(new_tiles);
    }
    fn move_left(&mut self ) -> bool {
        let mut new_tiles = vec![vec![0;4];4]; 

        for y in 0..4{ 
            let mut previous_tile : i32 = 0;
            let mut position = 0;
            for x in 0..4{ 
                let tile = self.tiles[x][y];

                if tile == 0 {
                    continue;
                }
                if tile != previous_tile {
                    new_tiles[position][y] = tile;
                    position += 1;
                    previous_tile = tile;
                }
                else{

                    self.score += 2i32.pow(tile as u32+ 1);
                    new_tiles[position -1 ][y] = tile + 1;

                    previous_tile = 0;
                }

            }
        }
        return self.has_moved(new_tiles);
    }
    fn move_up(&mut self ) -> bool {
        let mut new_tiles = vec![vec![0;4];4]; 
        let mut x = 0;
        for col in self.tiles.iter(){
            let mut previous_tile : i32 = 0;
            let mut position = 0;
            for tile in col.iter() {
                if tile == &0 {
                    continue;
                }
                if tile != &previous_tile {
                    new_tiles[x][position] = *tile;
                    position += 1;

                    previous_tile = *tile;
                }
                else{

                    self.score += 2i32.pow(*tile as u32+ 1);
                    new_tiles[x][position-1] = *tile + 1;

                    previous_tile = 0;
                }

            }
            x += 1;
        }
        return self.has_moved(new_tiles);
    }

    fn move_down(&mut self ) -> bool {


        let mut new_tiles=  vec![vec![0;4];4]; 
        let mut x = 0;
        for col in self.tiles.iter(){
            let mut previous_tile : i32 = 0;
            let mut position = 3;
            for tile in col.iter().rev() {
                if tile == &0 {
                    continue;
                }
                if tile != &previous_tile {
                    new_tiles[x][position] = *tile;
                    if position != 0 {
                        position -= 1;
                    }
                    previous_tile = *tile;
                }
                else{

                    self.score += 2i32.pow(*tile as u32+ 1);
                    new_tiles[x][position + 1] = *tile + 1;
                    previous_tile = 0;
                }

            }
            x += 1;
        }
        return self.has_moved(new_tiles);

    }
    fn has_legal_move(&self) -> bool {
        if self.tiles
            .iter()
            .any(|x| x.iter().any(|y| y == &0 )) {
            return true;
        }
        for x in 0..4{
            for y in 0..4{
                if x != 3 {
                    if self.tiles[x][y] == self.tiles[x+1][y] {
                        return true
                    }
                }

                if y != 3 {
                    if self.tiles[x][y] == self.tiles[x][y+1] {
                        return true
                    }
                }
            }
        }
        return false;
    }

    fn set_tile(&mut self) {
        let mut rng = ::rand::thread_rng();
        loop{
            let x = rng.gen_range(0..4);
            let y = rng.gen_range(0..4);
            if self.tiles[x][y] == 0 {
                self.tiles[x][y] = 1;
                break;
            }
        }
        
    }
}

fn get_move() -> Option<Direction> {

    if is_key_released(KeyCode::A){
        return Some(Direction::Left);
    }

    if is_key_released(KeyCode::D){
        return Some(Direction::Right);
    }

    if is_key_released(KeyCode::W){
        return Some(Direction::Up);
    }
    if is_key_released(KeyCode::S){
        return Some(Direction::Down);
    }

    return None;
}
fn get_tile_color(value:i32) -> Color{
    if value == 0 {
        return WHITE;
    }
    match value{
        1 => RED,
        2 => ORANGE,
        3 => YELLOW,
        4 => LIME,
        5 => GREEN,
        6 => BLUE,
        7 => PURPLE,
        8 => PINK,
        _ => get_tile_color(value - 8),
    }
}

fn get_text(value: i32) -> String{
    let mut text :i32 = 1;
    for _i in 0..value {
        text *= 2;
    }
    if text == 1 {
        return "".to_string();
    }
    return text.to_string();
}


fn draw_board(board: &Board){
    clear_background(WHITE);
    let board_size = screen_height() * 0.8; 
    let padding_left = (screen_width() - board_size) / 2.0; 
    let padding_top = screen_height() * 0.1;

    draw_text(
        format!("Score: {}", board.score).as_str(), 
        padding_left, 
        padding_top * 0.75,
        padding_top,
        BLACK);

    const BORDER_WIDTH : f32 = 3.0;
    draw_rectangle(
        padding_left - BORDER_WIDTH,
        padding_top - BORDER_WIDTH,
        board_size + 2.0* BORDER_WIDTH,
        board_size + 2.0 *BORDER_WIDTH ,
        BLACK
    );

    let tile_size = board_size  * 0.25;

    for x in 0..4 {
        for y in 0..4 {
            draw_rectangle(
                padding_left + tile_size* x as f32 + BORDER_WIDTH,
                padding_top + tile_size * y as f32 + BORDER_WIDTH,
                tile_size - 2.0 * BORDER_WIDTH,
                tile_size - 2.0 * BORDER_WIDTH,
                get_tile_color(board.tiles[x][y])
            );
            draw_text(
                &get_text(board.tiles[x][y]), 
                padding_left + tile_size * x as f32  + 0.45 * tile_size,
                padding_top + tile_size * y as f32 + BORDER_WIDTH + 0.5* tile_size,
                32.0, 
                BLACK);
        }
    }
    if board.game_over {
        let button_width :f32= board_size / 1.07;
        let button_height :f32= tile_size ;

        draw_rectangle(
            (screen_width() - button_width) / 2.0 , 
            (screen_height()  - button_height) / 2.0,
            button_width,
            button_height, 
            BLACK);

        draw_text(
            "press SPACE to",
            (screen_width() ) / 2.0  - 0.47 * button_width , 
            (screen_height()  ) /2.0 - 0.15 * button_height,
            button_height / 2.0,
            WHITE);
        
        draw_text(
            "start a new Game",
            (screen_width() ) / 2.0  - 0.47 * button_width , 
            (screen_height()  ) /2.0 + 0.35 * button_height ,
            button_height / 2.0,
            WHITE);
    }
}


#[macroquad::main("BasicShapes")]
async fn main() {
    let mut board = Board{ 
        tiles: vec![vec![0;4];4] ,
        score: 0,
        game_over: true
    }; 

    loop { 
        if !board.game_over {
            match get_move(){
                Some(direction) =>board.player_move(direction),
                None => ()
            }
        }
        else{
            if is_key_released(KeyCode::Space){
                board.tiles = vec![vec![0;4];4];
                board.score = 0;
                board.game_over = false;
                board.set_tile();
                board.set_tile();
            }
        }

        draw_board(&board);
        next_frame().await
    } 
}
