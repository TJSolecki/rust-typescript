use std::fmt::Display;
use super::area::Area;

#[derive(Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self { 
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({}, {}): {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}

pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.points.len() {
            return None;
        }

        let point = self.points[self.idx];
        self.idx += 1;

        return Some(point);
    }
}

impl From<&Rect> for RectIter {
    
    fn from(value: &Rect) -> Self {
        return RectIter {
            points: vec![
                (value.x, value.y),
                (value.x + value.width, value.y),
                (value.x, value.y + value.height),
                (value.x + value.width, value.y + value.height)
            ],
            idx: 0
        };
    }
}

impl IntoIterator for &Rect {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return self.into();
    }
}

impl IntoIterator for Rect {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return (&self).into();
    }
}
