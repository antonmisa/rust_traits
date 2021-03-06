trait HasArea {
	fn area(&self) -> f64;
}

struct Circle {
	x: f64,
	y: f64,
	radius: f64,
}

impl HasArea for Circle {
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}
}

struct Square {
	x: f64,
	y: f64,
	side: f64,
}

impl HasArea for Square {
	fn area(&self) -> f64 {
		self.side * self.side
	}
}

fn print_area<T: HasArea>(shape: &T) {
	println!("area is {}", shape.area());
}

fn main() {
    let c = Circle { x: 0., y: 0., radius: 1. };
	let s = Square { x: 0., y: 0., side: 1. };
	
	print_area(&c);
	print_area(&s);
}
