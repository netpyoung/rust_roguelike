extern crate tcod;
use tcod::BackgroundFlag;
use tcod::console::{Console, Root, Offscreen};
use tcod::input::KeyCode;

fn main() {
    let mut root = Root::initializer().init();
    let (width, height) = (80, 30);
    let mut con = Offscreen::new(width, height);
    // let mut con = Console::init_root(80, 50, "libtcod Rust tutorial", false);
    let mut exit = false;

    while !(root.window_closed() || exit) {
        root.clear();
        con.put_char(40, 25, '@', BackgroundFlag::Set);

        root.flush();

        let keypress = root.wait_for_keypress(true);
        match keypress.code {
            KeyCode::Escape => exit = true,
            _ => {}
        }
    }
}
