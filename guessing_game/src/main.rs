use std::io;
 
fn main() {
    println!("숫자를 맞혀봅시다!");

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

    println!("입력한 값: {}", guess);
}
