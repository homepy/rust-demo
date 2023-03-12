#![allow(unused)]
fn main() {
    // Iterator trait 和 next 方法
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
        // 此处省略了方法的默认实现
    }

    {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got: {}", val);
        }

        // iter 方法生成一个不可变引用的迭代器;
        // 如果我们希望迭代可变引用，则可以调用 iter_mut.
        // 如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter;
    }

    {
        // 产生其他迭代器的方法
        // 迭代器适配器（iterator adaptors）
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // 消费迭代器的sum()方法
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    // 使用捕获其环境的闭包
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 捕获其环境的闭包
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
