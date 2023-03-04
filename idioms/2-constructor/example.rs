struct Circle {
    radius: f64
}

impl Default for Circle {
    fn default( ) -> Self {
        return Self {
            radius: 10.0
        };
    }
}

impl Circle {

    // acts as the constructor for the `Circle` struct
    pub fn new(radius: f64) -> Self {
        return Circle {
            radius
        };
    }
}

fn main( ) { }