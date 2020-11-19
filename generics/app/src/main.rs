fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.2, y: 4.1 };
    let integer_float_point = PointDiffTypes { x: 5, y: 4.1 };
    println!("integer_point.x = {}", integer_point.x());

    let p1 = PointDiffTypes { x: 5, y: 10.4 };
    let p2 = PointDiffTypes { x: "Hello", y: 'c' };
    let p1p2 = p1.mixup(p2);
    println!("p1p2.x = {}, p1p2.y = {}", p1p2.x, p1p2.y);
}

fn largest<T>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_number(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &Vec<char>) -> &char {
    let mut largest = &list[0];
    for c in list {
        if c > largest {
            largest = c;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct PointDiffTypes<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> PointDiffTypes<T, U> {
    fn mixup<V, W>(self, other: PointDiffTypes<V, W>) -> PointDiffTypes<T, W> {
        PointDiffTypes {
            x: self.x,
            y: other.y,
        }
    }
}
