// 第2周 - 猜数字游戏
// 学习目标：控制流、循环、基础所有权概念

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("🎯 猜数字游戏！");
    println!("我想了一个1到100之间的数字，你能猜到吗？");
    
    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    
    // 游戏主循环
    loop {
        attempts += 1;
        println!("\n第{}次尝试，请输入你的猜测:", attempts);
        
        // 读取用户输入
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");
        
        // 解析输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入有效的数字！");
                attempts -= 1; // 无效输入不计入尝试次数
                continue;
            }
        };
        
        // 验证输入范围
        if guess < 1 || guess > 100 {
            println!("请输入1到100之间的数字！");
            attempts -= 1;
            continue;
        }
        
        // 比较并给出提示
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！再试试看 📉"),
            Ordering::Greater => println!("太大了！再试试看 📈"),
            Ordering::Equal => {
                println!("🎉 恭喜你猜对了！");
                println!("答案是 {}，你用了 {} 次尝试", secret_number, attempts);
                break;
            }
        }
        
        // 给出鼓励信息
        give_encouragement(attempts);
    }
    
    // 评估表现
    evaluate_performance(attempts);
}

// 给出鼓励信息的函数
fn give_encouragement(attempts: u32) {
    match attempts {
        1..=3 => println!("很好！继续保持 💪"),
        4..=6 => println!("不错，就快猜到了 🤔"),
        7..=10 => println!("加油，你可以的！ 💪"),
        _ => println!("别放弃，每次尝试都在接近答案！ 🎯"),
    }
}

// 评估表现的函数 - 练习所有权
fn evaluate_performance(attempts: u32) {
    let message = match attempts {
        1 => "太幸运了！一次就猜中！🍀",
        2..=5 => "表现很棒！你很有天赋！⭐",
        6..=10 => "不错的表现！👍",
        11..=15 => "还可以，多练习会更好！📚",
        _ => "虽然尝试次数多了点，但坚持就是胜利！🏆",
    };
    
    println!("\n{}", message);
    
    // 所有权练习：字符串的克隆和借用
    let performance_record = format!("本次游戏用了{}次尝试", attempts);
    println!("游戏记录: {}", performance_record);
    
    // 借用字符串，不获取所有权
    print_record(&performance_record);
}

// 借用字符串的函数 - 练习引用和借用
fn print_record(record: &str) {
    println!("保存记录: {}", record);
}

// 练习任务：
// 1. 添加难度级别选择（不同范围的数字）
// 2. 添加提示系统（热度提示：很冷、冷、温暖、很热）
// 3. 添加游戏历史记录
