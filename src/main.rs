use comfy::*;
mod game;
use game::*;

simple_game!("Snake", GameStruct, config, setup, update);

pub const TICK: u64 = 10;
enum GameState {
    Welcome,
    Playing,
    GameOver,
}

struct SmallGlobalState {
    pub game: Game,
    pub tick: u64,
    pub game_state: GameState,
}

pub struct GameStruct {}

impl GameStruct {
    pub fn new(_c: &EngineContext) -> Self {
        Self {}
    }
}

static STATE: Lazy<AtomicRefCell<SmallGlobalState>> = Lazy::new(|| {
    AtomicRefCell::new(SmallGlobalState {
        game: Game::create(),
        tick: 0,
        game_state: GameState::Welcome,
    })
});

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(SCREEN_WIDTH, SCREEN_HEIGHT),
        ..config
    }
}

fn setup(_state: &mut GameStruct, _c: &mut EngineContext) {}

fn update(_state: &mut GameStruct, _c: &mut EngineContext) {
    let mut state = STATE.borrow_mut();
    state.tick += 1; // controls rendering

    if is_key_down(KeyCode::W) {
        state.game._snake.set_speed(0.0, 1.0);
    }
    if is_key_down(KeyCode::S) {
        state.game._snake.set_speed(0.0, -1.0);
    }
    if is_key_down(KeyCode::A) {
        state.game._snake.set_speed(-1.0, 0.0);
    }
    if is_key_down(KeyCode::D) {
        state.game._snake.set_speed(1.0, 0.0);
    }
    if is_key_down(KeyCode::Return) {
        state.game_state = GameState::Playing;
    }

    if state.game._snake.game_over() {
        state.game_state = GameState::GameOver;
    }

    match state.game_state {
        GameState::Welcome => {
            draw_text(
                format!("Snake Game (Enter to Start)",).as_str(),
                vec2(0.0, 0.0),
                GREEN,
                TextAlign::Center,
            );
        }
        GameState::Playing => {
            if state.tick % TICK == 0 {
                state.game._snake.draw();
                state.game._fruit.draw();

                state.game._snake.update();

                let fx: f32 = state.game._fruit.px;
                let fy: f32 = state.game._fruit.py;
                if state.game._snake.check_eat(fx, fy) {
                    state.game._fruit.new_pos();
                }
            }
        }
        GameState::GameOver => {
            draw_text(
                format!("GAME OVER (Enter to try again)",).as_str(),
                vec2(0.0, 0.0),
                RED,
                TextAlign::Center,
            );
            if is_key_down(KeyCode::Return) {
                state.game_state = GameState::Playing;
                state.game = Game::create();
            }
        }
    }
}
