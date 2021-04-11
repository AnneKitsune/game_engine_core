<a href="https://crates.io/crates/game_engine_core">
    <img src="https://img.shields.io/crates/v/game_engine_core.svg" alt="Game Engine Core" />
</a>

# Game Engine Core

Support an Open Source Developer! :hearts:  
[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

Read the [documentation](https://docs.rs/game_engine_core).

# Features

* Create and store a stack-based state machine.
* Manually update individual game frames.
* Automatically run a game loop.
* Game engine agnostic.
* Does not rely on ECS.

# Usage
Add the following to you Cargo.toml file:
```
game_engine_core = "*"
```

Use it like so:
```rust
use game_engine_core::*;

struct MyState;
impl State<i32> for MyState {
    fn update(&mut self, state_data: &mut i32) -> StateTransition<i32> {
        *state_data += 1;
        StateTransition::Quit
    }
}

fn main() {
    Engine::new(MyState, 0, |_, _| {}, 1000.0)
        .engine_loop();
}
```

### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)

