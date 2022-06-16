fn main() {
    let x = plus_one(5);
    println!("x의 값: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);
}

// 구문 vs 표현식
// 구문 : 함수를 선언하는 것, let 키워드를 이용해 변수를 생성하고 값을 대입하는 것.
// 표현식 : 어떤 값으로 평가 되는 것. 함수를 호출하는 것, 매크로를 호출하는 것.
fn main() {
    // 구문
    let y = 6;
    // 여기서 6 자체는 값으로 평가되기 때문에 표현식이다.
    // 5 + 6 역시 표현 식이다. 11의 값으로 평가 되기 때문에

    // 오류 예제 1.
    let x = (let y = 6);
    // let y = 6 의 리턴 값이 없어 x에 값이 대입되지 않았다.
}

// {} 역시 표현식이다.
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("y의 값: {}", y);
    // y 값은 4가 된다.
    // 지금까지 봤던 코드와는 다르게 표현식은 마지막에 세미콜론을 포함하지 않는다.
    // 만일 여기에 세미콜론이 붙는다면 구문이 되어 값을 리턴하지 않는다.
}

// 함수 구문, 표현식 예제
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("x의 값: {}", x);
}

// x = 6
fn main() {
    let x = plus_one(5);
    println!("x의 값: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// 오류 발생
fn main() {
    let x = plus_one(5);
    println!("x의 값: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
