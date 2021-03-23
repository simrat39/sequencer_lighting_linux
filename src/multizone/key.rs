use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    Escape,
    Backtick,
    Tab,
    Caps,
    ShiftLeft,
    ControlLeft,
    F12,
    Equals,
    F9,
    Nine,
    O,
    L,
    Comma,
    WindowsRight,
    ArrowLeft,
    F1,
    One,
    Q,
    A,
    WindowsLeft,
    PrintScreen,
    F10,
    Zero,
    P,
    SemiColon,
    FullStop,
    Enter,
    ArrowDown,
    F2,
    Two,
    W,
    S,
    Z,
    AltLeft,
    ScreenLock,
    Backspace,
    F11,
    Dash,
    SquareBracketOpen,
    Apostrophe,
    ForwardSlash,
    F3,
    Three,
    E,
    D,
    X,
    Pause,
    Delete,
    P1,
    NumLock,
    Num6,
    F4,
    Four,
    R,
    F,
    C,
    Insert,
    End,
    Num8,
    P2,
    Num1,
    F5,
    Five,
    T,
    G,
    V,
    Omen,
    PageOn,
    Stop,
    Num9,
    P3,
    NumStar,
    Num2,
    F6,
    Six,
    Y,
    H,
    B,
    PageUp,
    ShiftRight,
    Previous,
    P4,
    NumDash,
    Num3,
    F7,
    Seven,
    U,
    J,
    N,
    AltRight,
    SquareBracketClose,
    ControlRight,
    PlayPause,
    Num4,
    P5,
    NumPlus,
    Num0,
    F8,
    Eight,
    I,
    K,
    M,
    Fn,
    Backslash,
    ArrowUp,
    Forward,
    Num5,
    NumEnter,
    NumFullStop,
    Next,
}

impl Key {
    pub fn get_rows() -> Vec<Vec<Key>> {
        vec![
            vec![Key::P1, Key::P2, Key::P3, Key::P4, Key::P5],
            vec![
                Key::Escape,
                Key::Backtick,
                Key::Tab,
                Key::Caps,
                Key::ShiftLeft,
                Key::ControlLeft,
            ],
            vec![Key::One, Key::Q, Key::A, Key::Z, Key::WindowsLeft],
            vec![Key::F1, Key::Two, Key::W, Key::S, Key::X, Key::AltLeft],
            vec![Key::F2, Key::Three, Key::E, Key::D, Key::C],
            vec![Key::F3, Key::Four, Key::R, Key::F, Key::V],
            vec![Key::F4, Key::Five, Key::T, Key::G, Key::B],
            vec![Key::Six, Key::Y, Key::H, Key::N],
            vec![Key::F5, Key::Seven, Key::U, Key::J, Key::M],
            vec![Key::F6, Key::Eight, Key::I, Key::K, Key::Comma],
            vec![
                Key::F7,
                Key::Nine,
                Key::O,
                Key::L,
                Key::FullStop,
                Key::AltRight,
            ],
            vec![
                Key::F8,
                Key::Zero,
                Key::P,
                Key::SemiColon,
                Key::ForwardSlash,
                Key::Fn,
            ],
            vec![Key::F9, Key::Dash, Key::SquareBracketOpen, Key::Apostrophe],
            vec![Key::F10, Key::Equals, Key::SquareBracketClose],
            vec![
                Key::F11,
                Key::F12,
                Key::Backspace,
                Key::Backslash,
                Key::Enter,
                Key::ShiftRight,
                Key::WindowsRight,
                Key::ControlRight,
            ],
            vec![Key::PrintScreen, Key::Insert, Key::Delete, Key::ArrowLeft],
            vec![
                Key::ScreenLock,
                Key::Omen,
                Key::End,
                Key::ArrowUp,
                Key::ArrowDown,
            ],
            vec![Key::Pause, Key::PageUp, Key::PageOn /* arrow right */],
            vec![
                Key::PlayPause,
                Key::NumLock,
                // num 7
                Key::Num4,
                Key::Num1,
                Key::Num0,
            ],
            vec![
                Key::Stop,
                // num slash
                Key::Num8,
                Key::Num5,
                Key::Num2,
            ],
            vec![
                Key::Previous,
                Key::NumStar,
                Key::Num9,
                Key::Num6,
                Key::Num3,
                Key::NumFullStop,
            ],
            vec![Key::Forward, Key::NumStar, Key::NumPlus, Key::NumEnter],
        ]
    }

    pub fn position_in_buffer_r(&self) -> (u8, u8) {
        match self {
            Key::Escape => (3, 4),
            Key::Backtick => (3, 5),
            Key::Tab => (3, 6),
            Key::Caps => (3, 7),
            Key::ShiftLeft => (3, 8),
            Key::ControlLeft => (3, 9),
            Key::F12 => (3, 10),
            Key::Equals => (3, 11),
            Key::F9 => (3, 12),
            Key::Nine => (3, 13),
            Key::O => (3, 14),
            Key::L => (3, 15),
            Key::Comma => (3, 16),
            Key::WindowsRight => (3, 17),
            // 3,18 ?
            Key::ArrowLeft => (3, 19),
            Key::F1 => (3, 20),
            Key::One => (3, 21),
            Key::Q => (3, 22),
            Key::A => (3, 23),
            // 3, 24 ?
            Key::WindowsLeft => (3, 25),
            Key::PrintScreen => (3, 26),
            // 3, 27 ?
            Key::F10 => (3, 28),
            Key::Zero => (3, 29),
            Key::P => (3, 30),
            Key::SemiColon => (3, 31),
            Key::FullStop => (3, 32),
            // 3, 33 ?
            Key::Enter => (3, 34),
            Key::ArrowDown => (3, 35),
            Key::F2 => (3, 36),
            Key::Two => (3, 37),
            Key::W => (3, 38),
            Key::S => (3, 39),
            Key::Z => (3, 40),
            Key::AltLeft => (3, 41),
            Key::ScreenLock => (3, 42),
            Key::Backspace => (3, 43),
            Key::F11 => (3, 44),
            Key::Dash => (3, 45),
            Key::SquareBracketOpen => (3, 46),
            Key::Apostrophe => (3, 47),
            Key::ForwardSlash => (3, 48),
            // 3, 49 ?
            // 3, 50 ?
            // 3, 51 ?
            Key::F3 => (3, 52),
            Key::Three => (3, 53),
            Key::E => (3, 54),
            Key::D => (3, 55),
            Key::X => (3, 56),
            // 3, 57 ?
            Key::Pause => (3, 58),
            Key::Delete => (3, 59),
            // 3, 60 ?
            // 3, 61 ?
            Key::P1 => (3, 62),

            // Next buf
            Key::NumLock => (4, 4),
            Key::Num6 => (4, 5),
            // 4, 6 ?
            // 4, 7 ?
            Key::F4 => (4, 8),
            Key::Four => (4, 9),
            Key::R => (4, 10),
            Key::F => (4, 11),
            Key::C => (4, 12),
            // 4, 13 ?
            Key::Insert => (4, 14),
            Key::End => (4, 15),
            // 4, 16 ?
            Key::Num8 => (4, 17),
            Key::P2 => (4, 18),
            // 4, 19 ?
            // 4, 20 ?
            Key::Num1 => (4, 21),
            // 4, 22 ?
            // 4, 23 ?
            Key::F5 => (4, 24),
            Key::Five => (4, 25),
            Key::T => (4, 26),
            Key::G => (4, 27),
            Key::V => (4, 28),
            // 4, 29 ?
            Key::Omen => (4, 30),
            Key::PageOn => (4, 31),
            Key::Stop => (4, 32),
            Key::Num9 => (4, 33),
            Key::P3 => (4, 34),
            // 4, 35 ?
            Key::NumStar => (4, 36),
            Key::Num2 => (4, 37),
            // 4, 38 ?
            // 4, 39 ?
            Key::F6 => (4, 40),
            Key::Six => (4, 41),
            Key::Y => (4, 42),
            Key::H => (4, 43),
            Key::B => (4, 44),
            // 4, 45 ?
            Key::PageUp => (4, 46),
            Key::ShiftRight => (4, 47),
            Key::Previous => (4, 48),
            // 4, 49 ?
            Key::P4 => (4, 50),
            // 4, 51 ?
            Key::NumDash => (4, 52),
            Key::Num3 => (4, 53),
            // 4, 54 ?
            // 4, 55 ?
            Key::F7 => (4, 56),
            Key::Seven => (4, 57),
            Key::U => (4, 58),
            Key::J => (4, 59),
            Key::N => (4, 60),
            Key::AltRight => (4, 61),
            Key::SquareBracketClose => (4, 62),
            Key::ControlRight => (4, 63),

            // next buf
            Key::PlayPause => (5, 4),
            Key::Num4 => (5, 5),
            Key::P5 => (5, 6),
            // 5, 7 ?
            Key::NumPlus => (5, 8),
            Key::Num0 => (5, 9),
            // 5, 10 ?
            // 5, 11 ?
            Key::F8 => (5, 12),
            Key::Eight => (5, 13),
            Key::I => (5, 14),
            Key::K => (5, 15),
            Key::M => (5, 16),
            Key::Fn => (5, 17),
            Key::Backslash => (5, 18),
            Key::ArrowUp => (5, 19),
            Key::Forward => (5, 20),
            Key::Num5 => (5, 21),
            // 5, 22 ?
            // 5, 23 ?
            // 5, 24 ?
            Key::NumEnter => (5, 24),
            Key::NumFullStop => (5, 25),
            Key::Next => (5, 25),
        }
    }
}
