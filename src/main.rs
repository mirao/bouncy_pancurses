/** Difference between window and frame width/height */
const WINDOW_FRAME_DIFF: i32 = 4;
/** Wait between ball moves, in msec */
const SLEEP_DURATION: i32 = 33;

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: i32,
    y: i32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: i32,
    height: i32,
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

    let mut game = create_game(&window);
    // Disable visibility of block cursor during ball rendering
    pancurses::curs_set(0);
    // Disable showing of input keys
    pancurses::noecho();
    // Show initial game state
    render_initial_game(&window, &game);

    // How long to wait on input key after move is done - therefore it's delay between moves
    window.timeout(SLEEP_DURATION);

    loop {
        // Hide ball in old position
        window.mvaddch(game.ball.y, game.ball.x, ' ');
        // Move ball
        game.step();
        // Show ball in new position
        window.mvaddch(game.ball.y, game.ball.x, 'o');
        window.refresh(); // Update the screen

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
                pancurses::resize_term(0, 0);
                // Reset game to initial state
                game = create_game(&window);
                // Clear existing frame content
                window.clear();
                render_initial_game(&window, &game);
            }
            _ => (),
        }
    }
}

fn create_game(window: &pancurses::Window) -> Game {
    let (max_y, max_x) = window.get_max_yx();
    let width = max_x - WINDOW_FRAME_DIFF;
    let height = max_y - WINDOW_FRAME_DIFF;
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
    Game::new(frame)
}

fn render_initial_game(window: &pancurses::Window, game: &Game) {
    // Render game area
    window.border('|', '|', '-', '-', '+', '+', '+', '+');
    // Render ball in initial position
    window.mvaddch(game.ball.y, game.ball.x, 'o');
    // Show updates on screen
    window.refresh();
}
