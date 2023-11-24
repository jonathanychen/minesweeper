#[derive(Debug, Clone, Copy)]
pub struct Tile {
    x: i32,
    y: i32,
    value: Value,
    visible: bool,
}

impl Tile {
    pub fn new(x: i32, y: i32, mine: bool) -> Tile {
        Tile {
            x,
            y,
            value: match mine {
                true => Value::Mine,
                false => Value::Zero,
            },
            visible: false
        }
    }

    pub fn reveal(&mut self) -> Value {
        self.visible = true;
        self.value
    }

    pub fn get_value(&self) -> Value {
        self.value
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_associated_value(number: i32) -> Option<Value> {
        match number {
            0 => Some(Value::Zero),
            1 => Some(Value::One),
            2 => Some(Value::Two),
            3 => Some(Value::Three),
            4 => Some(Value::Four),
            5 => Some(Value::Five),
            6 => Some(Value::Six),
            7 => Some(Value::Seven),
            8 => Some(Value::Eight),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Mine,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visible {
    Start,
    Visible,
    Hidden,
    Flagged,
}