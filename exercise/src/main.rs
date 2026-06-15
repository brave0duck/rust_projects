// 1. 가습기모드 2. 가습량 => 자료구조 struct로 구현
// 1. 입력받고 출력해서 보여주는 간단한 로직
use std::io;

enum Mode{
    auto,
    target,
    minimum,
}
enum WindSpeed{
    zero,
    one,
    two,
    three,
    four,
    five,
}
struct Humidifier{
    h_mode : Mode,
    h_step : WindSpeed,
}
impl Humidifier{
    fn new(&self, mode : u32, step : u32) -> Humidifier{
        Humidifier{
            h_mode = match mode{
                1=> Mode::auto,
                2=> Mode::target,
                3=> Mode::minimum,
            },
            h_step => match step{
                0=> WindSpeed::zero,
                1=> WindSpeed::one,
                2=> WindSpeed::two,
                3=> WindSpeed::three,
                4=> WindSpeed::four,
                5=> WindSpeed::five,
            }
        }
    }
}
fn main() {

    let mut input_mode  = String::new();
    let mut input_step = String::new();

    

    println!("가습기 모드와 출력을 선택하세요");
    println!("가습기 모드는 ? (auto:1, target:2, minimum:3) ");
    io::stdin().read_line(&mut input_mode).expect("input error");
    println!("출력단계는 ? ");
    io::stdin().read_line(&mut input_step).expect("input error");

    let input_mode : u32 = input_mode.trim().parse().expect("failed to convert");
    let input_step : u32 = input_step.trim().parse().expect("failed to convert");

    state_humidifier()

    println!("Hello, world!");
}

fn state_humidifier(h : & Humidifier) {
    println!("가습기의 모드는 : {} , 출력은 : {} 입니다");
}
