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
    Alt(Key), // aka Meta, aka Option.
    Cmd(Key), // Apple's Command key.
}

// From vim, :help map-special-keys:
// DETAIL: Vim first checks if a sequence from the keyboard is mapped.  If it
// isn't the terminal key codes are tried.  If a terminal code is found it is
// replaced with the internal code.  Then the check for a mapping is done again
// (so you can map an internal code to something else).  What is written into
// the script file depends on what is recognized. If the terminal key code was
// recognized as a mapping the key code itself is written to the script file.
// If it was recognized as a terminal code the internal code is written to the
// script file.
//
// The above suggests we should be precise about how we store our keys.
pub fn parse_angle(buffer: &str) -> Option<Key> {
    use self::Key::*;
    let key = match buffer.to_lowercase().as_ref() {
        "nul" => Null,
        "bs" => Backspace,
        "tab" => Tab,
        "nl" => Char('\n'),
        "ff" => Char('\x0c'),
        "cr" | "return" | "enter" => Char('\r'),
        "esc" => Esc,
        "space" => Space,
        "lt" => Char('<'),
        "bslash" => Char('\\'),
        "bar" => Char('|'),
        "del" => Delete,
        "csi" => Csi,
        "xcsi" => XCsi,
        "eol" => Eol,
        "up" => Up,
        "down" => Down,
        "left" => Left,
        "right" => Right,
        "f1" => F(1),
        "f2" => F(2),
        "f3" => F(3),
        "f4" => F(4),
        "f5" => F(5),
        "f6" => F(6),
        "f7" => F(7),
        "f8" => F(8),
        "f9" => F(9),
        "f10" => F(10),
        "f11" => F(11),
        "f12" => F(12),
        "help" => Help,
        "undo" => Undo,
        "insert" => Insert,
        "home" => Home,
        "end" => End,
        "pageup" => PageUp,
        "pagedown" => PageDown,
        "khome" => KeypadHome,
        "kend" => KeypadEnd,
        "kpageup" => KeypadPageUp,
        "kpagedown" => KeypadPageDown,
        "kplus" => KeypadPlus,
        "kminus" => KeypadMinus,
        "kmultiply" => KeypadMultiply,
        "kdivide" => KeypadDivide,
        "kenter" => KeypadEnter,
        "kpoint" => KeypadDot,
        "k0" => Keypad(0),
        "k1" => Keypad(1),
        "k2" => Keypad(2),
        "k3" => Keypad(3),
        "k4" => Keypad(4),
        "k5" => Keypad(5),
        "k6" => Keypad(6),
        "k7" => Keypad(7),
        "k8" => Keypad(8),
        "k9" => Keypad(9),
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
    use key::Key::*;
    use key::MultiKey::*;

    named!(
        shift<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<S-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| Shift(x)) }
        )
    );
    named!(
        ctrl<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<C-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| Ctrl(x)) }
        )
    );
    named!(
        meta<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<M-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| Alt(x)) }
        )
    );
    named!(
        alt<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<A-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| Alt(x)) }
        )
    );
    named!(
        cmd<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<D-"), is_not!(">"), tag_s!(">")),
            |st| { parse_any(st).map(|x| Cmd(x)) }
        )
    );

    named!(
        angle<&str, MultiKey>,
        map_opt!(
            delimited!(tag_no_case_s!("<"), is_not!(">"), tag_s!(">")),
            |st| { parse_angle(st).map(|x| A(x)) }
        )
    );
    named!(
        key<&str, MultiKey>,
        map_opt!(
            take_s!(1),
            |st| { parse_key(st).map(|x| A(x)) }
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
        assert_eq!(parse::shift("<s-tab>"), IResult::Done("", Shift(Tab)));
    }

    #[test]
    fn ctrl_test() {
        assert_eq!(parse::ctrl("<c-tab>"), IResult::Done("", Ctrl(Tab)));
    }

    #[test]
    fn alt_test() {
        assert_eq!(parse::alt("<a-tab>"), IResult::Done("", Alt(Tab)));
    }

    #[test]
    fn meta_test() {
        assert_eq!(parse::meta("<m-tab>"), IResult::Done("", Alt(Tab)));
    }

    #[test]
    fn cmd_test() {
        assert_eq!(parse::cmd("<d-tab>"), IResult::Done("", Cmd(Tab)));
    }

    #[test]
    fn any_test() {
        assert_eq!(parse::any("<tab>"), IResult::Done("", A(Tab)));
    }

    #[test]
    fn parse_key_test() {
        assert_eq!(parse::parse("S"), vec![A(Char('S'))]);
    }

    #[test]
    fn parse_test() {
        assert_eq!(parse::parse("<S-Tab>S"), vec![Shift(Tab), A(Char('S'))]);
    }
}
