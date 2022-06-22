#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// 모듈에는 다른 모듈, 구조체, 열거자, 상수, 트레이트, 함수 모두 추가할 수 있다.
mod front_of_house {
    mod hosting {
        fn add_to_waillist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// mod : module
mod front_of_house {
    // 외부에서 이걸 사용하게 해주고 싶으면 pub를 선언해주어야 한다.
    // pub은 시작 부모부터 쓰려는 곳의 아이템까지  모두에 선언을 해주어야 한다.
    // 쉽게 말해서 eat_at_restaurant 부터 타고 타고 들어가면서 pub을 선언해야 한다.
    // 이해를 위해 부연설명을 한다.
    // front_of_house 가 pub 이 아닌데도 가능한 이유는 eat_at_restaurant 와 같은 모듈에서
    // 정의되었기 때문이다.
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// pub : public
pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}

// super 사용 예제
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // back_of_house의 부모 모듈로 가기 위해 super 사용
        super::server_order();
    }

    fn cook_order() {}
}

// 구조체와 열거자 공개
mod back_of_house {
    // 구조체를 정의할 때 pub 키워드를 사용한다면 구조체는 공개되지만,
    // 구조체의 필드는 여전히 비공개다.
    // 필요에 따라 각 필드를 공개하거나 비공개 할 수 있다.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }
}

// 빵은 선택할 수 있으나 과일은 주어진 대로 먹어야 한다.
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    // meal.seasonal_fruit = String::from("블루베리");
    // 이렇게 선언 불가.
}

// 열거자는 공개시 모든 열것값 또한 공개된다.
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// use 사용하기
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use 키워드와 경로를 추가해 바로 사용할 수 있다.
// 절대경로
use crate::front_of_house::hosting;
// 상대경로
use self::front_of_house::hosting;
// 팁이라면, 개발이 더 진행되서 self 키워드가 사라질 수도 있다. 러스트 개발자들이 싫어한단다.
// 함수 자체 전까지 use로 불러오는게 관용적인 방법이다.
// 로컬이 아닌 가져왔다는 사실을 더 잘 알 수 있어서.

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 반면, 구조체, 열거자, 혹은 가티 다른 아이템을 use 구문으로 가져올 때는
// 전체 경로를 사용하는 것이 관용적이다.
// 이유는 따로 없단다. 더 익숙해서?
use std::collection::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 혹여나 서로 다른 부모 모듈에 같은 이름을 가진 아이템이 있다면 함수처럼 그 전까지 적어주는게
// 맞다.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {}

fn function2() -> io::Result<()> {}

// 다른 방법이라면 as를 이용해 새로운 이름을 부여하는 것이다.
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}

fn function2() -> IoResult<()> {}

// use 키워드를 이용해 어떠한 것을 가져오면 이는 새 범위에서는 비공개 이름이 된다. 이를 접근할 수
// 있게 해주려면 똑같이 pub를 사용하자.
// 이를 다시 내보내기(re-exporting) 이라고 한다.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
// 다시 내보내기는 다른 개발자가 생각하는 코드의 호출 방식과 실제 코드의 내부 구조가 다를 때
// 유용하다.
