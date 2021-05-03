use std::fmt;

// Structure of a Point

#[derive(Copy, Clone)]
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
    fn area(&self) -> f32 {
        ((self.a.x*(self.b.y-self.c.y) + self.b.x*(self.c.y - self.a.y) + self.c.x*(self.a.y - self.b.y)) as f32 /2.0).abs()
    }

    // Find area of a triangle with Heron's Formula
    fn area_heron(&self, perimeter:f32) -> f32 {
        let p = perimeter/(2 as f32);
        f32::sqrt(p * (p-self.a.distance(&self.b)) * (p-self.b.distance(&self.c)) * (p-self.c.distance(&self.a)))
    }

    // Check whether a point is inside a triangle.
    // This is done by creating smaller triangles with the point and seeing whether `ABC = ABP + PBC + APC`
    fn is_point_inside(&self, point: &Point) -> bool {
        let tr_a = Triangle{a: self.a, b: self.b, c: *point};
        let tr_b = Triangle{a: *point, b: self.b, c: self.c};
        let tr_c = Triangle{a: self.a, b: *point, c: self.c};

        self.area() == tr_a.area() + tr_b.area() + tr_c.area()
    }

}

fn main() {
    println!("\n==== Ultimate Triangle Functions");

    let tr = Triangle{
        a: Point{x:5, y: 5},
        b: Point{x:10, y: 0},
        c: Point{x: 0, y: 0},
    };

    println!("Points of the triangle : {}", tr);
    println!("The length of the three sides of the triangle : {} {} {}", tr.a.distance(&tr.b), tr.b.distance(&tr.c), tr.c.distance(&tr.a));
    println!("Perimeter of the triangle is : {}", tr.area());
    println!("Area of the triangle is : {}", tr.area());
    println!("Area of the triangle is using Heron's Formula : {}", tr.area_heron(tr.perimeter()));
    
    let random_point = Point{x: 4, y: 3};
    
    println!("Is the point {} in the triangle? : {}", random_point, tr.is_point_inside(&random_point));
}
