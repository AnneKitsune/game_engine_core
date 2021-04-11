//! A simple crate that takes care of updating the main loop of a game engine.
//! It is based on a stack-based state machine and is agnostic of the engine used.
//! It does not rely on an ECS nor on other libraries. Sleeping is configurable.
//! When sleeping is disabled, this crate is compatible with WASM. When enabled,
//! the game loop will run at the target framerate.
#![deny(missing_docs)]
pub use game_clock::*;
pub use game_state_machine::*;
use spin_sleep::LoopHelper;

/// The main structure of the engine core loop.
/// It holds the data necessary to the execution of a game engine.
pub struct Engine<SD, F: Fn(&mut SD, &Time)> {
    loop_helper: LoopHelper,
    state_machine: StateMachine<SD>,
    state_data: SD,
    time: Time,
    post_update: F,
}

impl<SD, F: Fn(&mut SD, &Time)> Engine<SD, F> {
    /// Creates a new `Engine`.
    /// The initial state and state data will be used to initialize the state machine.
    /// The post update function will be stored. It is called at the end of game frames.
    /// `max_fps` specifies the maximum number of frames that can happen within a second.
    pub fn new<I: State<SD> + 'static>(init_state: I, mut init_state_data: SD, post_update: F, max_fps: f32) -> Self {
        let loop_helper = LoopHelper::builder().build_with_target_rate(max_fps);
        let mut state_machine = StateMachine::default();
        let time = Time::default();
        state_machine.push(Box::new(init_state), &mut init_state_data);
        Self {
            loop_helper,
            state_machine,
            state_data: init_state_data,
            time,
            post_update,
        }
    }

    /// Runs a single frame of the engine. Returns false if this was the last
    /// frame the engine will run and returns true if the engine can be run again.
    /// The sleep argument specifies if this function should take care of sleeping
    /// the thread. It is recommended to always have it to true, except in the
    /// case where you are using the engine in an async context. If set to false,
    /// the `Time` argument in the post_update callback will be meaningless and you
    /// will have to calculate the time difference yourself.
    ///
    /// This function is most useful when called from WASM or in the context of
    /// another loop. For instance, winit and bracket-lib are both libraries that
    /// require control of the main loop, for compatibility with mobile and web platforms.
    /// Here, we can let them take care of the main loop and simple call `engine_frame`.
    pub fn engine_frame(&mut self, sleep: bool) -> bool {
        if sleep {
            let delta = self.loop_helper.loop_start();
            {
                self.time.advance_frame(delta);
            }
        }

        self.state_machine.update(&mut self.state_data);
        if sleep {
            self.loop_helper.loop_sleep();
        }
        (self.post_update)(&mut self.state_data, &self.time);
        self.state_machine.is_running()
    }

    /// Runs the engine until the state machine quits.
    /// Generics:
    /// - SD: The type of the data that is passed to states when updating.
    /// - I: The type of the initial state. This is the first state that it started
    /// when the engine is started.
    /// - F: The post update function. This function is called after each loop of
    /// of the engine. It receives the state data mutable and a reference to the
    /// structure keeping track of the time. This function is called *after* sleeping
    /// at the end of the frame, which means it is equivalent to the start of the next
    /// frame.
    pub fn engine_loop(&mut self) {
        while self.engine_frame(true) {}
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_loop() {
        struct MyState;
        impl State<i32> for MyState {
            fn update(&mut self, state_data: &mut i32) -> StateTransition<i32> {
                *state_data += 1;
                StateTransition::Quit
            }
        }
        Engine::new(MyState, 0, |s, _| {*s+=1; assert_eq!(*s, 2);},1000.0).engine_loop();
    }
}
