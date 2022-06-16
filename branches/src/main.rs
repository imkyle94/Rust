fn main() {
    let number = 3;

    // if 문은 반드시 불리언 타입을 리턴해야 한다.
    if number < 5 {
        println!("조건이 일치합니다.");
    } else {
        println!("조건이 일치하지 않습니다.");
    }

    // else if가 너무 많아지면 지저분하므로 러스트의 강력한 분기 구조인 match 표현식을 잘 이용하는 습관을 들이자.

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number의 값: {}", number);
}

// 러스트는 반복 루프로 loop, while, for 을 제공한다.
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("발사!");
}

// 루프 반복시 컴파일러가 조건 검사를 하기 때문에 인덱싱이 확실하면 for문을 이용하자.
// 추가로 iter() 등의 메서드를 잘 활용하면 좋을 것이다.
fn main() {
    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("요소의 값: {}", element);
    }
}

// 메서드를 잘 쓰자!
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("발사!");
}
