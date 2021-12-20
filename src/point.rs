use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Sub;
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PointI {
    pub x: i32,
    pub y: i32,
}

impl FromStr for PointI {
    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        match s.split_once(",") {
            None => Err(()),
            Some((xstr, ystr)) => {
                let x = xstr.parse::<i32>();
                let y = ystr.parse::<i32>();
                match (x, y) {
                    (Ok(x), Ok(y)) => Ok(PointI { x, y }),
                    _ => Err(()),
                }
            }
        }
    }
    type Err = ();
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl FromStr for Point3 {
    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        match s.split_once(",") {
            None => Err(()),
            Some((xstr, yzstr)) => match yzstr.split_once(",") {
                None => Err(()),
                Some((ystr, zstr)) => {
                    let x = xstr.parse::<i32>();
                    let y = ystr.parse::<i32>();
                    let z = zstr.parse::<i32>();
                    match (x, y, z) {
                        (Ok(x), Ok(y), Ok(z)) => Ok(Point3 { x, y, z }),
                        _ => Err(()),
                    }
                }
            },
        }
    }
    type Err = ();
}
impl Debug for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl<'a> Add<&'a Point3> for Point3 {
    type Output = Point3;
    fn add(self, other: &'a Point3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub<Point3> for Point3 {
    type Output = Point3;
    fn sub(self, other: Point3) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl<'a> Sub<&'a Point3> for Point3 {
    type Output = Point3;
    fn sub(self, other: &'a Point3) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl<'a, 'b> Sub<&'a Point3> for &'b Point3 {
    type Output = Point3;
    fn sub(self, other: &'a Point3) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
