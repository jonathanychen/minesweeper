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

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

#[derive(Debug, Clone, Copy)]
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
