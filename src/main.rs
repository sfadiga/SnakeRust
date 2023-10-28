use comfy::*;
mod game;
use game::*;


simple_game!("Snake", GameState, config, setup, update);


struct SmallGlobalState {
   pub game: Game,
}

pub struct GameState {}

impl GameState {
    pub fn new(_c: &EngineContext) -> Self {
        Self {}
    }
}

static STATE: Lazy<AtomicRefCell<SmallGlobalState>> = Lazy::new(|| {
   AtomicRefCell::new(SmallGlobalState { game : Game::create() })
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

   state.game._snake.update();
   state.game._snake.draw();

   //state.game._fruit.update();
   state.game._fruit.draw();


}