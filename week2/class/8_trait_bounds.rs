pub struct Point<T1: Clone, T2: Clone> { // No problem for T1 == T2
    x: T1,
    _y: T2,
}

/** TRAIT BOUNDS
 * Trait bounds are a restriction on the types, so that we can specify behaviour they must implement
 */
impl<T1, T2: Clone> Point<T1, T2>
    where T1: Clone // + Debug + Display
{
    pub fn x(&self) -> &T1 {
        return &self.x;
    }

    fn mixup<X2: Clone, Y2: Clone>(&self, other: &Point<X2, Y2>) -> Point<T1, Y2> {
        Point { 
            x: self.x.clone(),
            _y: other._y.clone(),
         }
    }
}


fn main() {

    let p1 = Point { x: 5, _y: 10};
    let p2 = Point { x: 6.0, _y: 11};

    println!("p1.x = {:?}", p1.x);
    println!("p1.mixup(p2).x = {:?}", p1.mixup(&p2).x);
    println!("p1.mixup(p2)._y = {:?}", p1.mixup(&p2)._y);
}
