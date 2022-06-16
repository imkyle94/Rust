fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x의 값: {}", x);

    // mut vs shadowing
    let spaces = "    ";
    let spaces = spaces.len();

    let mut spaces = "    ";
    let spaces = spaces.len();

    let guess: u32 = "42".parse().expect("숫자가 아닙니다!");
    
    // 러스트는 정적 타입 언어이다. 컴파일 시점에 모든 변수의 타입이 결정되어있어야 한다.
    // 부동 소수점 타입
    let x = 2.0;
    let y: f32 = 3.0;

    // 불리언 타입
    let t = true;
    let f: bool = false;

    // 문자 타입
    // 러스트의 char 타입은 4byte 크기의 유니코드 스칼라값이다.
    let c = "hi";

    // 튜플 타입
    // 튜플은 고정된 길이를 가지며 한 번 정의하면 그 크기를 키우거나 줄일 수 없다.
    // 각각의 값은 패턴 매칭을 이용해 얻을 수 있다.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    // 인덱싱을 이용할 수도 있다.
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 배열 타입
    // 배열은 모기든 데이터 타입이 동일해야 한다.
    // 또한, 타 언어와 다르게 배열은 고정 길이다.
    // 월(Month) 등의 데이터 등은 배열을 쓰는 것이 적합할 것이다.
    let a = [1, 2, 3, 4, 5];
    // 같은 값 넣기
    let a = [3; 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 배열은 다루고자 하는 데이터를 힙(heap)이 아닌 스택(stack) 메모리에 할당하거나 항상 고정된
    // 개수의 요소들을 다룰 때 유용하다.
    // 표준 라이브러리는 벡터(Vector)를 지원하며, 벡터는 크기 조절이 가능하다. 둘 중 어느 것을
    // 사용할지 모르겠다면 벡터를 사용하는 것이 일반적으로 더 낫다.
}
