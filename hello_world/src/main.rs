fn main() {
    let var0 : Option<i32> = Option::None;
    let var1  : Option<&str> = Option::Some("Hello, world!");
    let var2  : Option<i32> = Option::Some(23);

    println!("before fn - var0: {:?}, var1 : {:?}, var2 : {:?}", var0 ,var1 ,var2);

    let var0 = check_data(&var0);
    let var1 = check_data(&var1);
    let var2 = check_data(&var2);

    println!("before fn - var0: {:?}, var1 : {:?}, var2 : {:?}", var0 ,var1 ,var2);
}
fn check_data<T: std::fmt::Debug + Copy>(s : &Option<T>) -> Option<T> {
    match s{
        Some(i) => {
            println!("Here is in func. Some - {:?}", i);
            Some(*i)
        },
        None => {
            println!("Here is in func. data - None");
            None
        }
    }
}