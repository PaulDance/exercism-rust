use enum_iterator::Sequence;
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    ResistorColor::from_int(value)
        .map(|c| format!("{c:?}"))
        .unwrap_or("value out of range".to_owned())
}

pub fn colors() -> Vec<ResistorColor> {
    enum_iterator::all().collect()
}
