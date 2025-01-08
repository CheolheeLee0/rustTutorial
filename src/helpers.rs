// pub으로 선언하면 main에서 helpers 직접 접근 가능
mod mod_number;
mod mod_string;

pub fn function_string() {
    mod_string::function();
}

pub fn function_number() {
    let result = mod_number::add_number(2, 3);
    println!("mod_number::add_number() : {}", result);
}