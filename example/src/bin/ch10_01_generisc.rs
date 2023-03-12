// 1 struct 中的泛型定义
struct Point1<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// 2 enum中的泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 3 方法中的泛型
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 定义方法时也可以为泛型指定限制（constraint）。例如，可以选择为 Point1<f32> 实例实现方法，而不是为泛型 Point1 实例。
impl Point1<f32> {
    // 这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，而其他 T 不是 f32 类型的 Point1<T> 实例则没有定义此方法。
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    {
        // 1 struct 中的泛型定义
        let integer = Point1 { x: 5, y: 10 };
        let float = Point1 { x: 1.0, y: 4.0 };

        let both_integer = Point2 { x: 5, y: 10 };
        let both_float = Point2 { x: 1.0, y: 4.0 };
        let integer_and_float = Point2 { x: 5, y: 4.0 };
    }
    {
        // 3 方法中的泛型
        let p = Point1 { x: 5, y: 10 };
        println!("p.x = {}", p.x());

        //
        let p1 = Point2 { x: 5, y: 10.4 };
        let p2 = Point2 { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}

// 泛型代码的性能.在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
