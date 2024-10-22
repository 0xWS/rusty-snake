use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
pub struct SnakeGame {
    width: i32,
    height: i32,
    snake: Vec<Point>,
    direction: Direction,
    food: Point,
    game_over: bool,
}

#[wasm_bindgen]
impl SnakeGame {
    pub fn new(width: i32, height: i32) -> SnakeGame {
        let snake = vec![Point { x: width / 2, y: height / 2 }];
        let direction = Direction::Right;
        let food = SnakeGame::generate_food(width, height, &snake);
        
        SnakeGame {
            width,
            height,
            snake,
            direction,
            food,
            game_over: false,
        }
    }

    fn generate_food(width: i32, height: i32, snake: &Vec<Point>) -> Point {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        loop {
            let food = Point {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..height),
            };
            if !snake.contains(&food) {
                return food;
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn snake_head_x(&self) -> i32 {
        self.snake[0].x
    }

    pub fn snake_head_y(&self) -> i32 {
        self.snake[0].y
    }

    pub fn food_x(&self) -> i32 {
        self.food.x
    }

    pub fn food_y(&self) -> i32 {
        self.food.y
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn snake_length(&self) -> usize {
        self.snake.len()
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        let opposite_direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };

        if new_direction != opposite_direction {
            self.direction = new_direction;
        }
    }

    pub fn step(&mut self) {
        if self.game_over {
            return;
        }

        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => Point { x: head.x, y: (head.y - 1 + self.height) % self.height },
            Direction::Right => Point { x: (head.x + 1) % self.width, y: head.y },
            Direction::Down => Point { x: head.x, y: (head.y + 1) % self.height },
            Direction::Left => Point { x: (head.x - 1 + self.width) % self.width, y: head.y },
        };

        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);

        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.food = SnakeGame::generate_food(self.width, self.height, &self.snake);
        } else {
            self.snake.pop();
        }
    }

    pub fn snake_cells(&self) -> Vec<u32> {
        self.snake
            .iter()
            .map(|&Point { x, y }| (y * self.width + x) as u32)
            .collect()
    }
}