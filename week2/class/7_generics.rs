pub struct Point<T1, T2> { // No problem for T1 == T2
    x: T1,
    _y: T2,
}

impl<T1, T2> Point<T1, T2> {
    pub fn x(&self) -> &T1 {
        return &self.x;
    }
}


fn main() {

    let p1 = Point { x: 5, _y: 10};
    let p2 = Point { x: 5.0, _y: 10.0};
    let p3 = Point { x: 5, _y: 10.0};
    let p3 = Point { x: p1, _y: 10.0};

    // println!("p1.x = {:?}", p1.x);
    println!("p2.x = {:?}", p2.x);
    println!("p3.x = {:?}", p3.x);
}
