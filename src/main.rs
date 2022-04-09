enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(frame: Frame) -> Game {
        Game {
            frame,
            ball: Ball::default(),
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 1 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width + 2 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 1 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height + 2 {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            x: 3,
            y: 5,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        }
    }
}

fn main() {
    let window = pancurses::initscr();

    let (max_y, max_x) = window.get_max_yx();
    let width = max_x as u32 - 4;
    let height = max_y as u32 - 4;
    let frame = Frame { width, height };

    // Don't allow small frame
    if frame.width < 10 || frame.height < 10 {
        pancurses::endwin();
        panic!(
            "Width ({width}) or height ({height}) are too small",
            height = frame.height,
            width = frame.width,
        );
    }

    let mut game = Game::new(frame);
    const SLEEP_DURATION: i32 = 33;
    // Disable visibility of block cursor during ball rendering
    pancurses::curs_set(0);
    // Disable showing of input keys
    pancurses::noecho();
    // Show initial game state
    render_initial_game(&window, &game);

    loop {
        // Hide ball in old position
        window.mvaddch(game.ball.y as i32, game.ball.x as i32, ' ');
        // Move ball
        game.step();
        // Show ball in new position
        window.mvaddch(game.ball.y as i32, game.ball.x as i32, 'o');
        window.refresh(); // Update the screen

        // Wait between moves (it doesn't block input key reading)
        window.timeout(SLEEP_DURATION);

        match window.getch() {
            // Quit game
            Some(pancurses::Input::Character('q')) => {
                pancurses::endwin();
                return;
            }
            // Pause ball animation
            Some(pancurses::Input::Character('p')) | Some(pancurses::Input::Character(' ')) => {
                while window.getch().is_none() {}
            }
            // Terminal size was changed
            Some(pancurses::Input::KeyResize) => {
                // Change window size
                let (lines, cols) = window.get_max_yx();
                pancurses::resize_term(0, 0);
                // Reset game's frame and ball position
                game = Game::new(Frame {
                    height: lines as u32 - 4,
                    width: cols as u32 - 4,
                });
                // Clear existing content
                window.clear();
                render_initial_game(&window, &game);
            }
            _ => (),
        }
    }
}

fn render_initial_game(window: &pancurses::Window, game: &Game) {
    // Render game area
    window.border('|', '|', '-', '-', '+', '+', '+', '+');
    // Render ball in initial position
    window.mvaddch(game.ball.y as i32, game.ball.x as i32, 'o');
    // Show updates on screen
    window.refresh();
}
