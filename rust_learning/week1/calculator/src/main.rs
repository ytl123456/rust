// 第1周 - 简单计算器
// 学习目标：函数、用户输入、基础运算

use std::io;

fn main() {
    println!("🧮 欢迎使用Rust计算器！");
    println!("支持的操作: +, -, *, /");
    
    loop {
        println!("\n请输入第一个数字:");
        let num1 = read_number();
        
        println!("请输入运算符 (+, -, *, /) 或 'q' 退出:");
        let operator = read_operator();
        
        if operator == "q" {
            println!("再见！");
            break;
        }
        
        println!("请输入第二个数字:");
        let num2 = read_number();
        
        let result = calculate(num1, &operator, num2);
        match result {
            Some(value) => println!("结果: {} {} {} = {}", num1, operator, num2, value),
            None => println!("错误: 除数不能为零！"),
        }
    }
}

// 读取数字输入
fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("请输入有效的数字:"),
        }
    }
}

// 读取运算符输入
fn read_operator() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    
    input.trim().to_string()
}

// 执行计算
fn calculate(num1: f64, operator: &str, num2: f64) -> Option<f64> {
    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                None
            }
        }
        _ => {
            println!("不支持的运算符: {}", operator);
            None
        }
    }
}

// 练习题：尝试添加以下功能
// 1. 支持更多运算符（%, ^）
// 2. 支持历史记录
// 3. 支持连续计算
