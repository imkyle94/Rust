use std::net::TcpListener;

fn main() {
    // bind 함수 : new 함수처럼 TcpListener 인스턴스 리턴
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 스트림 : 클라이언트와 서버 사이의 연결
    // 연결 : 클라이언트가 서버와 연결하고 서버가 응답을 생성한 후 연결을 닫기까지의 전반적인
    // 요청과 응답 과정
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("연결됨");
    }
    // stream 변수는 루프의 끝에 도달하면 범위를 벗어나며, drop 메서드가 실행된다.

    // 한 번의 브라우저 요청에 여러 개의 메세지가 출력된다.
    // 그 이유는 페이지에 대한 요청 포함해 favicon.ico 등의 다른 자원에 대한 요청도 생성되었기
    // 때문이다.
    // 또한 서버가 데이터를 전혀 리턴하지 않았기 때문이라고 볼 수도 있다.
}
