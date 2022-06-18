#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl : implementation 블록
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // 트레이트 등은 문서를 잘 참고하자
    println!("rect1: {:?}", rect1);
    println!("사각형의 면적 : {} 제곱 픽셀", area(&rect1));
    println!("사각형의 면적 : {} 제곱 픽셀", rect1.area());
    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));
}

// 구조체의 소유권을 잠깐 빌려쓰기 위해 참조(&) 사용
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
