extern crate tcod;

pub type TcodInputKey = self::tcod::input::Key;

#[derive(Debug, Copy, Clone)]
pub enum KeyCode {
    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Special
    Shift,
    Escape,

    // Default
    None,
}


#[derive(Debug, Copy, Clone)]
pub enum Key {
    Printable(char),
    SpecialKey(KeyCode),
}


#[derive(Debug, Copy, Clone)]
pub struct KeyboardInput {
    pub key: Key,
}


pub trait InputComponent<T> {
    fn translate_input(&self, T) -> KeyboardInput;
}

pub struct TcodInputComponent;

impl TcodInputComponent {
    pub fn new() -> TcodInputComponent {
        TcodInputComponent
    }
}

impl InputComponent<TcodInputKey> for TcodInputComponent {
    fn translate_input(&self, key_state: TcodInputKey) -> KeyboardInput {
        let key: Key = if key_state.shift {
            match key_state.code {
                self::tcod::input::KeyCode::Number5 => Key::Printable('%'),
                self::tcod::input::KeyCode::Number6 => Key::Printable('^'),
                self::tcod::input::KeyCode::Number7 => Key::Printable('&'),
                self::tcod::input::KeyCode::Number8 => Key::Printable('*'),
                _ => Key::SpecialKey(KeyCode::None),
            }
        } else {
            match key_state.code {
                self::tcod::input::KeyCode::Char => Key::Printable(key_state.printable),
                self::tcod::input::KeyCode::Up => Key::SpecialKey(KeyCode::Up),
                self::tcod::input::KeyCode::Down => Key::SpecialKey(KeyCode::Down),
                self::tcod::input::KeyCode::Left => Key::SpecialKey(KeyCode::Left),
                self::tcod::input::KeyCode::Right => Key::SpecialKey(KeyCode::Right),
                self::tcod::input::KeyCode::Shift => Key::SpecialKey(KeyCode::Shift),
                self::tcod::input::KeyCode::Escape => Key::SpecialKey(KeyCode::Escape),
                _ => Key::SpecialKey(KeyCode::None),
            }
        };
        KeyboardInput { key: key }
    }
}
