pub struct Point<T> {
    x: T,
    y: T,
}
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl<T> Point<T> {
    pub fn new(t1: T, t2: T) -> Self {
        Point { x: t1, y: t2 }
    }
}
pub fn make_larger_or_equal<'a>(x: &'a mut String, y: &str) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        *x = y.to_string();
        x
    }
}
