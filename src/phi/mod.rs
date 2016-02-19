#[macro_use]
mod events;

use sdl2::render::Renderer;

// We cannot call functions at top-level. However, `struct_events` is not your
// usual function: it's a macro. Which means that you can use a macro to do
// pretty much anything _normal_ code would.
struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    },
    else: {
        quit: Quit { .. }
    }
}

pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    // Called on every frame to take care of the logic and
    // rendering of the current view.
    // `elapsed` is in seconds
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}
