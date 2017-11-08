use nom::IResult;
use std::str::from_utf8;

#[derive(Serialize, Deserialize, PartialOrd, Ord, Debug, Copy, Clone,
         PartialEq, Eq, Hash)]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    F(u8),
    /// Normal character.
    Char(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,
    Space,
    Tab,
    Csi,
    XCsi,
    Eol,
    Help,
    Undo,
    Keypad(u8),
    KeypadHome,
    KeypadEnd,
    KeypadPageUp,
    KeypadPageDown,
    KeypadPlus,
    KeypadMinus,
    KeypadMultiply,
    KeypadDivide,
    KeypadEnter,
    KeypadDot,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, Debug, Copy, Clone,
         PartialEq, Eq, Hash)]
pub enum MultiKey {
    A(Key), // A single keystroke.
    Shift(Key),
    Ctrl(Key),
    Alt(Key),
    Cmd(Key),
}

pub fn parse_angle(buffer: &str) -> Option<Key> {
    let key = match buffer.to_lowercase().as_ref() {
        "nul" => Key::Null,
        "bs" => Key::Backspace,
        "tab" => Key::Tab,
        "nl" => Key::Char('\n'),
        "ff" => Key::Char('\x0c'),
        "cr" | "return" | "enter" => Key::Char('\r'),
        "esc" => Key::Esc,
        "space" => Key::Space,
        "lt" => Key::Char('<'),
        "bslash" => Key::Char('\\'),
        "bar" => Key::Char('|'),
        "del" => Key::Delete,
        "csi" => Key::Csi,
        "xcsi" => Key::XCsi,
        "eol" => Key::Eol,
        "up" => Key::Up,
        "down" => Key::Down,
        "left" => Key::Left,
        "right" => Key::Right,
        "f1" => Key::F(1),
        "f2" => Key::F(2),
        "f3" => Key::F(3),
        "f4" => Key::F(4),
        "f5" => Key::F(5),
        "f6" => Key::F(6),
        "f7" => Key::F(7),
        "f8" => Key::F(8),
        "f9" => Key::F(9),
        "f10" => Key::F(10),
        "f11" => Key::F(11),
        "f12" => Key::F(12),
        "help" => Key::Help,
        "undo" => Key::Undo,
        "insert" => Key::Insert,
        "home" => Key::Home,
        "end" => Key::End,
        "pageup" => Key::PageUp,
        "pagedown" => Key::PageDown,
        "khome" => Key::KeypadHome,
        "kend" => Key::KeypadEnd,
        "kpageup" => Key::KeypadPageUp,
        "kpagedown" => Key::KeypadPageDown,
        "kplus" => Key::KeypadPlus,
        "kminus" => Key::KeypadMinus,
        "kmultiply" => Key::KeypadMultiply,
        "kdivide" => Key::KeypadDivide,
        "kenter" => Key::KeypadEnter,
        "kpoint" => Key::KeypadDot,
        "k0" => Key::Keypad(0),
        "k1" => Key::Keypad(1),
        "k2" => Key::Keypad(2),
        "k3" => Key::Keypad(3),
        "k4" => Key::Keypad(4),
        "k5" => Key::Keypad(5),
        "k6" => Key::Keypad(6),
        "k7" => Key::Keypad(7),
        "k8" => Key::Keypad(8),
        "k9" => Key::Keypad(9),
        _ => {
            return None;
        }
    };
    Some(key)
}

pub fn parse_key(buffer: &str) -> Option<Key> {
    if buffer.chars().count() == 1 {
        return Some(Key::Char(buffer.chars().next().unwrap()));
    }
    None
}

pub fn parse_any(buffer: &str) -> Option<Key> {
    parse_angle(buffer).or(parse_key(buffer))
}

#[macro_use]
pub mod parse {
    use super::*;

    named!(
        shift<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<S-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| MultiKey::Shift(x)) }
        )
    );
    named!(
        ctrl<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<C-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| MultiKey::Ctrl(x)) }
        )
    );
    named!(
        meta<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<M-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| MultiKey::Alt(x)) }
        )
    );
    named!(
        alt<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<A-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| MultiKey::Alt(x)) }
        )
    );
    named!(
        cmd<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<D-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| MultiKey::Cmd(x)) }
        )
    );

    named!(
        angle<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<"), is_not!(">"), tag_s!(">")),
            |st| { parse_angle(st).map(|x| MultiKey::A(x)) }
        )
    );
    named!(
        key<&str, MultiKey>,
        map_opt!(
            take_s!(1),
            |st| { parse_key(st).map(|x| MultiKey::A(x)) }
        )
    );
    named!(any<&str, MultiKey>, alt!( angle | key ));

    named!(parse_keys<&str, Vec<MultiKey>>, many0!(
            alt!(shift | ctrl | meta | alt | cmd | any)));

    pub fn parse(st: &str) -> Vec<MultiKey> {
        match parse_keys(st) {
            IResult::Done(_, x) => x,
            _ => Vec::new(),
        }
    }

    #[test]
    fn shift_test() {
        assert_eq!(
            parse::shift("<s-tab>"),
            IResult::Done("", MultiKey::Shift(Key::Tab))
        );
    }

    #[test]
    fn ctrl_test() {
        assert_eq!(
            parse::ctrl("<c-tab>"),
            IResult::Done("", MultiKey::Ctrl(Key::Tab))
        );
    }

    #[test]
    fn alt_test() {
        assert_eq!(
            parse::alt("<a-tab>"),
            IResult::Done("", MultiKey::Alt(Key::Tab))
        );
    }

    #[test]
    fn meta_test() {
        assert_eq!(
            parse::meta("<m-tab>"),
            IResult::Done("", MultiKey::Alt(Key::Tab))
        );
    }

    #[test]
    fn cmd_test() {
        assert_eq!(
            parse::cmd("<d-tab>"),
            IResult::Done("", MultiKey::Cmd(Key::Tab))
        );
    }

    #[test]
    fn any_test() {
        assert_eq!(
            parse::any("<tab>"),
            IResult::Done("", MultiKey::A(Key::Tab))
        );
    }

    #[test]
    fn parse_key_test() {
        assert_eq!(parse::parse("S"), vec![MultiKey::A(Key::Char('S'))]);
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse::parse("<S-Tab>S"),
            vec![MultiKey::Shift(Key::Tab), MultiKey::A(Key::Char('S'))]
        );
    }
}
