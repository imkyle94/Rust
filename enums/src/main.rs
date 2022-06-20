fn main() {
    enum IpAddrKind {
        v4,
        v6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };
}

fn main() {
    // 열거자의 값에 데이터를 직접 지정해 구조체 선언을 생략했다.
    enum IpAddr {
        v4(String),
        v6(String),
    }

    let home = IpAddr::v4(String::from("127.0.0.1"));

    let loopback = IpAddr::v6(String::from("::1"));
}

fn main() {
    // 구조체는 다음과 같은 형식으로 구조를 가질 수 없다.
    enum IpAddr {
        v4(u8, u8, u8, u8),
        v6(String),
    }

    let home = IpAddr::v4(127, 0, 0, 1);

    let loopback = IpAddr::v6(String::from("::1"));
}

fn main() {
    struct Ipv4Addr {}

    struct Ipv6Addr {}

    // 열거자의 값에는 문자열, 숫자, 구조체 등 어떤 종류의 데이터도 저장할 수 있다. 다른 열거자를
    // 저장해도 가능하다.
    enum IpAddr {
        v4(Ipv4Addr),
        v6(Ipv6Addr),
    }
}

fn main() {
    // 이와 같은 열거자를 정의하는 것은 각기 다른 구조체 타입을 정의하는 것과 유사하다. 다른
    // 점이라면 struct 키워드를 사용하지 않는다. 열거자의 개별 값들은
    // 모두 Message 타입이라는 것이다.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },    // 익명 구조체
        Write(String),              // 하나의 String값
        ChangeColor(i32, i32, i32), // 세 개의 i32 값
    }

    // 이는 너무 복잡하다. enum의 장점을 잘 생각할 것.`
    struct QuitMessage; // 유닛 구조체
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // 튜플 구조체
    struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

    // 열거자에도 메서드를 정의할 수 있다.
    impl Message {
        fn call(&self) {}
    }

    let m = Message::Write(String::from("hello"));

    m.call();
}

// 러스트는 null이 없다. Option 열거자를 대신 사용할 수 있다.
fn main() {
    // Option 열거자로 어떤 값의 존재 여부를 표현한다.
    // 어떤 값이 널값을 가질 수도 있다면 코드를 짤 때 사용하는 것이다.
    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

fn main() {
    enum Coin {
        Penny,
        Nickle,
        Dime,
        Quarter,
    }

    // match 표현식은 흐름 제어(if문)이다.
    // 타 언어의 switch와 비슷한듯?
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("행운의 페니!");
                1
            }
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // 이하 등등
    }

    enum Coin {
        Penny,
        Nickle,
        Dime,
        Quarter(UsState),
    }

    // 예제처럼 값을 바인딩 시킬 수도 있다.
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

// match 표현식과 열거자를 조합해 다양한 상황을 처리할 수 있다.
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// if-else 식을 똑같이 구현할 수 있다.
fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // 나머지 : _
        _ => (),
    }
}

// if let 문법은 주어진 값에 대해 하나의 패턴만 검사하고 나머지 값은 무시하는 math 표현식이다.
fn main() {
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// 정리하여 자신이 작성한느 API를 위한 사용자 정의 타입을 정의하면 타입 안정성을 확보할 수 있다.
// 즉, 컴파일러의 도움을 받아 각 함수가 기대하는 타입의 값만을 함수에 전달하도록 보장할 수 있다.
