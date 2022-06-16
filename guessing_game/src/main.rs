use std::io;
use std::cmp::Ordering;
// Ordering 열거자 : Less, Greater, Equal
use rand::Rng;
// Rng 트레이트(trait) : 객체지향 언어에서의 인터페이스 + 추상 클래스 짬뽕.

fn main() {
    println!("숫자를 맞혀봅시다!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // 크레이트를 사용하는 방법은 각 크레이트의 문서를 참고하는 것이 일반적이다.
    // cargo doc --open 명령 실행 시 의존 패키지 문서를 보여준다.

    println!("사용자가 맞혀야 할 숫자: {}", secret_number);

    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요.");

        // mut : 가변 변수, String : UTF-8 문자열 타입
        let mut guess = String::new();
        // :: : 연관함수 (String 타입 자체에 new 함수가 구현되어 있다. = 정적 메서드)

        // stdin : 터미널로부터의 표준 입력에 대한 Stdin 타입의 인스턴스를 리턴한다.
        // read_line : 사용자가 입력한 값을 표준 입력으로부터 읽어 문자열에 저장한다.
        // & : 참조(reference) 타입임을 명시.
        // 참조 타입을 사용하는 이유 : 프로그램의 다른 곳에서도 해당 데이터를 여러 번 메모리에 복사할 필요 없이 접근하기 위해
        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");
        // Result 타입 : 열거자(enums, or enumerations) : 미리 정의된 범위의 값을 갖는 타입
        // Result 열거자는 Ok와 Err(에러)를 가지고 있다. 즉, 에러 처리에 사용된다.

        // 러스트는 변수 가리기(shadowing)를 사용한다. 타입 변환시 보통 사용한다.
        // parse : 구문 분석
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}
