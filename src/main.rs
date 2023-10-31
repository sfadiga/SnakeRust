use comfy::*;
mod game;
use game::*;

simple_game!("Snake", GameState, config, setup, update);

pub const TICK: u64 = 10;

struct SmallGlobalState {
    pub game: Game,
    pub tick: u64,
}

pub struct GameState {}

impl GameState {
    pub fn new(_c: &EngineContext) -> Self {
        Self {}
    }
}

static STATE: Lazy<AtomicRefCell<SmallGlobalState>> = Lazy::new(|| {
    AtomicRefCell::new(SmallGlobalState {
        game: Game::create(),
        tick: 0,
    })
});

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(SCREEN_WIDTH, SCREEN_HEIGHT),
        ..config
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn update(_state: &mut GameState, _c: &mut EngineContext) {
    let mut state = STATE.borrow_mut();
    state.tick += 1;

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

    if state.tick % TICK == 0 {

        state.game._snake.update();

        let fx: f32 = state.game._fruit.px;
        let fy: f32 = state.game._fruit.py;
        if state.game._snake.check_eat(fx, fy) {
            state.game._fruit.new_pos();
        }

        state.game._snake.draw();
        state.game._fruit.draw();
    }

    draw_text(
        format!(
            "snake size:{}, tail:{}",
            state.game._snake.size, state.game._snake.tail.len()
        )
        .as_str(),
        vec2(-8.0, 14.0),
        WHITE,
        TextAlign::Center,
    );
    draw_text(
        format!(
            "fruit px:{}, py:{}",
            state.game._fruit.px, state.game._fruit.py
        )
        .as_str(),
        vec2(-8.0, 13.0),
        WHITE,
        TextAlign::Center,
    );
}
