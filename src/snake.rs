use super::piston_window::*;
extern crate rand;

use std::io::{self, Write};
use self::rand::Rng;


const BLOCK_SIZE : f64 = 10.0;
pub const WINDOW_SIZE_X : f64 = 500.0;
pub const WINDOW_SIZE_Y : f64 = 500.0;

pub const SIZE_X : usize = 50;
pub const SIZE_Y : usize = 50;

const START_SPEED  : u32 = 1;

#[derive(PartialEq, Clone,Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE
}

pub struct Game {
    snake : Vec<Block>,
    mouse : Block,
    speed : u32,
    board : Board,
    pub over  : bool,
    tick : u32,
    collision_ticks : u32

}

#[derive(Copy,Clone)]
pub struct Block {
    x : usize,
    y : usize,
}

pub struct Board {
    head_dir : [[Direction; (SIZE_X+2) as usize]; (SIZE_Y+2) as usize],
}

impl Block {
  
}

fn random_block() -> Block {
    let mut rng = rand::thread_rng();
    Block {
        x: rng.gen_range::<usize>(0, SIZE_X),
        y: rng.gen_range::<usize>(0, SIZE_Y)
    }
}


impl Game {

    pub fn new() -> Game {
        let start_x = SIZE_X/2;
        let start_y = SIZE_Y/2;
        let head_dir = [[Direction::NONE; SIZE_X+2]; SIZE_Y+2];


        let snake = vec![
            Block{
                x : start_x,
                y : start_y
            }];

        Game {
            snake,
            mouse : random_block(),
            speed : START_SPEED,
            board: Board {
                head_dir
            },
            over  : false,
            tick : 0,
            collision_ticks: 0
        }
    }

    fn move_block(&self,  block :Block, dir : Direction ) -> Block {
        let mut ret_block = block.clone();

        let mut speed = 0;
        if self.tick % 10 == 0 {
            speed = 1;
        }

        match dir {
            Direction::NONE => {},
            Direction::DOWN =>ret_block.y = ret_block.y + speed as usize,
            Direction::UP => {
                if ret_block.y > 0 {
                    ret_block.y = ret_block.y - speed as usize
                }
            },
            Direction::LEFT => {
                if ret_block.x > 0 {
                    ret_block.x = ret_block.x - speed as usize
                }
            },
            Direction::RIGHT => ret_block.x = ret_block.x + speed as usize,
        };
        ret_block
    }

    fn new_block(&self, front_block : &Block, dir: Direction) -> Block{
        let mut ret_block = front_block.clone();
        match dir {
            Direction::RIGHT => ret_block.x = ret_block.x-1,
            Direction::LEFT => ret_block.x = ret_block.x+1,
            Direction::UP => ret_block.y = ret_block.y +1,
            Direction::DOWN => ret_block.y = ret_block.y -1,
            _ => {}
        }
        ret_block
    }

    pub fn on_input(&mut self, but: Button) {
        let head_loc : &mut Direction = &mut self.board.head_dir[self.snake[0].x][self.snake[0].y];
        match but {
            Button::Keyboard(Key::Up) => {
                if *head_loc != Direction::DOWN {
                    *head_loc = Direction::UP;
                }

            }
            Button::Keyboard(Key::Down) => {
                if *head_loc != Direction::UP {
                   *head_loc = Direction::DOWN;
                }
            }
            Button::Keyboard(Key::Left) => {
                if *head_loc != Direction::RIGHT{
                    *head_loc = Direction::LEFT;
                }
            }
            Button::Keyboard(Key::Right) => {
                if *head_loc != Direction::LEFT {
                    *head_loc = Direction::RIGHT;
                }
            }
            _ => {}
        }
      
    }

    pub fn on_update(&mut self, upd: UpdateArgs) {
        self.over |= self.wall_collide();
        if self.over {
            return;
        }
        self.tick += 1;
        let head_loc: Direction = self.board.head_dir[self.snake[0].x][self.snake[0].y];
        {
            let board = &self.board.head_dir;
            self.snake = self.snake.iter().map(|block| {
                let dir = board[block.x][block.y];
                self.move_block(*block, dir)
            }).collect();
        }
        {
            let head = &self.snake[0];
            if self.over {
                return;
            }
            self.board.head_dir[head.x][head.y] = head_loc;
            self.over |= self.snake.iter().skip(1).any(|block| {
                block.x == head.x && block.y == head.y
            });
        }

        if self.mouse_collide() {
            self.mouse = random_block();
            let tail = self.snake.last().unwrap().clone();
            let dir = self.board.head_dir[tail.x][tail.y];
            let new_blk = self.new_block(&tail,dir);
            self.board.head_dir[new_blk.x][new_blk.y] = dir;
            self.snake.push(new_blk);
        }
    }


    fn wall_collide(&self,) -> bool {
        self.snake[0].x > SIZE_X
            || self.snake[0].x == 0
            || self.snake[0].y > SIZE_Y
            || self.snake[0].y == 0
    }

    fn mouse_collide(&self) -> bool {
        self.snake[0].x == self.mouse.x && self.snake[0].y == self.mouse.y
    }

    pub fn on_draw(&mut self, ren: RenderArgs,  w: &mut PistonWindow, e: &Event ) {

        w.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            for blk in self.snake.iter() {
                let mut x = blk.x;
                let mut y = blk.y;
                if blk.x > 0 {
                    x = blk.x - 1;
                }
                if blk.y > 0 {
                    y = blk.y - 1;
                }
                let center = c.transform.trans((x as f64) * BLOCK_SIZE, ((y as f64) * BLOCK_SIZE));
                let square = rectangle::square(0.0, 0.0, BLOCK_SIZE as f64);
                let snake_color = [0.0, 0.0, 0.0, 1.0];
                rectangle(snake_color, square, center, g);
            }
            let mut x = self.mouse.x;
            let mut y = self.mouse.y;
            if x > 0 {
                x = x - 1;
            }
            if y > 0 {
                y = y - 1;
            }
            let center = c.transform
                .trans((x as f64)*BLOCK_SIZE, (y as f64)*BLOCK_SIZE);
            let square = rectangle::square(0.0, 0.0, BLOCK_SIZE as f64);
            let mouse_color = [1.0, 0.0, 0.0, 1.0];
            rectangle(mouse_color, square, center, g);

        });
    }
}