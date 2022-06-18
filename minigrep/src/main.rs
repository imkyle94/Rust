use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // panic! 외 다른 에러 처리 방법
    // 파이프 문자 관심 있게 보기
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        // 프로그램 즉시 중단 및 종료 상태 코드로 전달한 값 리턴.
        process::exit(1);
    });

    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    // if let 구문과 unwrap_or_else 함수의 본문은 같다.
    if let Err(e) = minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);

        process::exit(1);
    }
}

// 함수로 분기
// fn parse_config(args: &[String]) -> Config {
//     // Config 인스턴스가 소유할 수 있는 데이터의 복제본 생성.
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }
