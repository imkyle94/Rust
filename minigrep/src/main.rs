use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // panic! 외 다른 에러 처리 방법
    // 파이프 문자 관심 있게 보기
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        // 프로그램 즉시 중단 및 종료 상태 코드로 전달한 값 리턴.
        process::exit(1);
    });

    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    // if let 구문과 unwrap_or_else 함수의 본문은 같다.
    if let Err(e) = run(config) {
        println!("애플리케이션 에러: {}", e);

        process::exit(1);
    }
}

// Return 타입 리턴.
// 트레이트 객체 Box<dyn Error>
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Result<String>
    // ? 연산자 : 에러 발생 시 현재 함수 호출자에게 에러값을 리턴할 수 있다.
    let contents = fs::read_to_string(config.filename)?;

    println!("파일 내용:\n{}", contents);

    Ok(())
}

// 구조체 선언
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// 함수로 분기
// fn parse_config(args: &[String]) -> Config {
//     // Config 인스턴스가 소유할 수 있는 데이터의 복제본 생성.
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }
