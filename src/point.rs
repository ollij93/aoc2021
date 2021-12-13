use std::str::FromStr;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Point {
    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        match s.split_once(",") {
            None => Err(()),
            Some((xstr, ystr)) => {
                let x = xstr.parse::<u32>();
                let y = ystr.parse::<u32>();
                match (x, y) {
                    (Ok(x), Ok(y)) => Ok(Point { x, y }),
                    _ => Err(()),
                }
            }
        }
    }
    type Err = ();
}
