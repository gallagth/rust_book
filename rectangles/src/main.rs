#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	fn square(size: u32) -> Rectangle {
		Rectangle { width: size, height: size }
	}
}

fn main() {
	let width1 = 30;
	let height1 = 50;

	let rect1 = (width1, height1);

	let rect2 = Rectangle{ width: 30, height: 50 };
	let rect3 = Rectangle{ width: 40, height: 60 };
	let rect4 = Rectangle{ width: 40, height: 60 };

	println!("The area of the rectangle is {} square pixels", area(width1, height1));
	println!("The area of the rectangle is {} square pixels", area_tuples(rect1));
	println!("The area of the rectangle is {} square pixels", area_struct(&rect2));
	println!("rectangle is {:?}", rect3);
	println!("The area of the rectangle is {} square pixels", rect4.area());

	println!("rect4 can hold rect2: {}", rect4.can_hold(&rect2));
	println!("rect2 can hold rect4: {}", rect2.can_hold(&rect4));

	let sq = Rectangle::square(25);
	println!("cool square {:?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}