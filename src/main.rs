use std::fmt;

// Structure of a Point

struct Point {
    x: i32,
    y: i32,
}

// Implementing fmt::Display for Point{}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.x, self.y)
    }
}

impl Point {
    // Distance between two points
    fn distance(&self, p: &Point) -> f32{
        f32::sqrt((i32::pow(self.y - p.y, 2) + i32::pow(self.x - p.x, 2)) as f32)
    }
}

// Structure of a Triangle

struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

// Implementing fmt::Display for Triangle{}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A{}, B{}, C{}", self.a, self.b, self.c)
    }
}

impl Triangle {

    // Find perimeter of a triangle
    fn perimeter(&self) -> f32{
        self.a.distance(&self.b) + self.b.distance(&self.c) + self.c.distance(&self.a)
    }

    // Find area of a triangle
    fn area(&self, perimeter: f32) -> f32{
        let p = perimeter/(2 as f32);
        f32::sqrt(p * (p-self.a.distance(&self.b)) * (p-self.b.distance(&self.c)) * (p-self.c.distance(&self.a)))
    }
}

fn main() {
    println!("\n==== Ultimate Triangle Functions");

    println!("{}", Point{x: 2, y: 3});

    let tr = Triangle{
        a: Point{x:5, y: 5},
        b: Point{x:10, y: 0},
        c: Point{x: 0, y: 0},
    };

    println!("Points of the triangle : {}", tr);
    println!("The length of the three sides of the triangle : {} {} {}", tr.a.distance(&tr.b), tr.b.distance(&tr.c), tr.c.distance(&tr.a));
    println!("Area of the triangle is : {}", tr.area(tr.perimeter()));
}
