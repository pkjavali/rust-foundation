//this depicts a traditional class realization in rust
trait ShapeArea {
    fn area(&self) -> i32;
}
pub struct Shape {
    //public access
    pub side-a:i32,
    pub side-b:i32,
    pub rad:i32,
    //private access
    pri-a:i32
}

impl Shape {
    fn area(&self) -> i32{
        self.side-a * self.side-b
    }
}

