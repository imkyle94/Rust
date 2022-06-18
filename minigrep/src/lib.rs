use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;

// pub 키워드 추가

// 구조체 선언
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sentitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Return 타입 리턴.
// 트레이트 객체 Box<dyn Error>
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Result<String>
    // ? 연산자 : 에러 발생 시 현재 함수 호출자에게 에러값을 리턴할 수 있다.
    let contents = fs::read_to_string(config.filename)?;

    println!("파일 내용:\n{}", contents);

    //  let mut f = File::open(config.filename);

    //    let mut contents = String::new();
    //   f.read_to_string(&mut contents)?;

    //   for line in search(&config.query, &contents) {
    //     println!("{}", line);
    // }

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

// 명시적 수정 'a
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // lines 메서드는 반복자를 리턴한다.
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
