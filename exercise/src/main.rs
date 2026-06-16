// 1. 가습기모드 2. 가습량 => 자료구조 struct로 구현
// 1. 입력받고 출력해서 보여주는 간단한 로직
use std::io;

#[derive(Debug)]
enum HumMode{
    Auto,
    Target,
    Minimum,
}
#[derive(Debug)]
enum HumSpeed{
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

struct Humidifier{
    h_mode : HumMode,
    h_step : HumSpeed,
}
impl Humidifier{
    fn new(mode : u32, step : u32) -> Humidifier{
        Humidifier{
            h_mode : match mode{
                1 => HumMode::Auto,
                2 => HumMode::Target,
                3 => HumMode::Minimum,
                _ => HumMode::Auto,
            },
            h_step : match step{
                0 => HumSpeed::Zero,
                1 => HumSpeed::One,
                2 => HumSpeed::Two,
                3 => HumSpeed::Three,
                4 => HumSpeed::Four,
                5 => HumSpeed::Five,
                _ => HumSpeed::Zero,
            }
        }
    }
    fn setup(&mut self, mode : u32, step : u32){
        self.h_mode = match mode{
            1 => HumMode::Auto,
            2 => HumMode::Target,
            3 => HumMode::Minimum,
            _ => HumMode::Auto,
        };
        self.h_step = match step {
            0 => HumSpeed::Zero,
            1 => HumSpeed::One,
            2 => HumSpeed::Two,
            3 => HumSpeed::Three,
            4 => HumSpeed::Four,
            5 => HumSpeed::Five,
            _ => HumSpeed::Zero,
        };
    }
    
}
fn main() {

    let mut input_mode  = String::new();
    let mut input_step = String::new();
    let mut hum = Humidifier::new(1,1);
    

    println!("가습기 모드와 출력을 선택하세요");
    println!("가습기 모드는 ? (auto:1, target:2, minimum:3) : ");
    
    io::stdin().read_line(&mut input_mode).expect("input error");
    
    println!("출력단계는 ? (0-5)  : ");
    io::stdin().read_line(&mut input_step).expect("input error");

    let input_mode : u32 = input_mode.trim().parse().expect("failed to convert");
    let input_step : u32 = input_step.trim().parse().expect("failed to convert");

    hum.setup(input_mode, input_step);
    state_humidifier(&hum);

}
fn state_humidifier(h : & Humidifier) {
    println!("==> 현재 가습기모드는 [{:?}]이고 출력단계는 [{:?}]단계입니다.", h.h_mode, h.h_step);
}
