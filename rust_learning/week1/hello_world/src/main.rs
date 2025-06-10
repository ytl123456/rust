// 第1周 - Hello World增强版
// 学习目标：变量、数据类型、函数基础

fn main() {
    // 基础打印
    println!("🦀 欢迎来到Rust世界！");
    
    // 变量练习
    variable_practice();
    
    // 数据类型练习
    data_types_practice();
    
    // 函数练习
    let result = add_numbers(10, 20);
    println!("10 + 20 = {}", result);
    
    // 字符串练习
    string_practice();
}

// 变量与可变性练习
fn variable_practice() {
    println!("\n=== 变量练习 ===");
    
    // 不可变变量
    let x = 5;
    println!("x = {}", x);
    
    // 可变变量
    let mut y = 10;
    println!("y = {}", y);
    y = 15;
    println!("y 修改后 = {}", y);
    
    // 变量遮蔽
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("z = {}", z);
}

// 数据类型练习
fn data_types_practice() {
    println!("\n=== 数据类型练习 ===");
    
    // 整数类型
    let small_number: i8 = 127;
    let big_number: i64 = 1_000_000;
    println!("小数字: {}, 大数字: {}", small_number, big_number);
    
    // 浮点数
    let pi: f64 = 3.14159;
    let e: f32 = 2.718;
    println!("π = {}, e = {}", pi, e);
    
    // 布尔值
    let is_rust_fun = true;
    let is_learning = false;
    println!("Rust有趣吗? {}, 正在学习吗? {}", is_rust_fun, !is_learning);
    
    // 字符
    let heart = '❤';
    let chinese = '中';
    println!("字符: {} {}", heart, chinese);
    
    // 元组
    let person: (String, i32, bool) = (String::from("张三"), 25, true);
    println!("姓名: {}, 年龄: {}, 在职: {}", person.0, person.1, person.2);
    
    // 数组
    let numbers = [1, 2, 3, 4, 5];
    println!("数组第一个元素: {}", numbers[0]);
    println!("数组长度: {}", numbers.len());
}

// 函数练习
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // 表达式，不需要分号
}

// 字符串练习
fn string_practice() {
    println!("\n=== 字符串练习 ===");
    
    let greeting = "你好";
    let name = "Rust学习者";
    println!("{}, {}!", greeting, name);
    
    // String类型
    let mut message = String::from("今天是我学习Rust的");
    message.push_str("第一天");
    println!("{}", message);
}
