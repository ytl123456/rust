// 第2周 - 简单待办事项列表
// 学习目标：Vec操作、字符串处理、基础所有权

use std::io;

fn main() {
    println!("📝 欢迎使用Rust待办事项列表！");
    
    let mut todo_list: Vec<String> = Vec::new();
    
    loop {
        show_menu();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");
        
        let choice = input.trim();
        
        match choice {
            "1" => add_task(&mut todo_list),
            "2" => show_tasks(&todo_list),
            "3" => complete_task(&mut todo_list),
            "4" => remove_task(&mut todo_list),
            "5" => {
                println!("再见！感谢使用待办事项列表 👋");
                break;
            }
            _ => println!("无效选择，请重新输入！"),
        }
    }
}

// 显示菜单
fn show_menu() {
    println!("\n=== 待办事项菜单 ===");
    println!("1. 添加任务");
    println!("2. 查看任务");
    println!("3. 完成任务");
    println!("4. 删除任务");
    println!("5. 退出");
    println!("请选择操作 (1-5):");
}

// 添加任务 - 练习可变引用
fn add_task(todo_list: &mut Vec<String>) {
    println!("请输入新任务:");
    
    let mut task = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("读取输入失败");
    
    let task = task.trim().to_string();
    
    if !task.is_empty() {
        todo_list.push(task.clone());
        println!("✅ 任务 '{}' 已添加", task);
    } else {
        println!("❌ 任务不能为空！");
    }
}

// 显示任务 - 练习不可变引用
fn show_tasks(todo_list: &Vec<String>) {
    if todo_list.is_empty() {
        println!("📭 暂无任务，享受你的空闲时光吧！");
        return;
    }
    
    println!("\n📋 当前任务列表:");
    for (index, task) in todo_list.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
    println!("总共 {} 个任务", todo_list.len());
}

// 完成任务 - 练习向量操作和所有权
fn complete_task(todo_list: &mut Vec<String>) {
    if todo_list.is_empty() {
        println!("📭 没有任务需要完成！");
        return;
    }
    
    show_tasks(todo_list);
    println!("请输入要完成的任务编号:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    
    match input.trim().parse::<usize>() {
        Ok(index) => {
            if index > 0 && index <= todo_list.len() {
                let completed_task = todo_list.remove(index - 1);
                println!("🎉 恭喜完成任务: '{}'", completed_task);
            } else {
                println!("❌ 无效的任务编号！");
            }
        }
        Err(_) => println!("❌ 请输入有效的数字！"),
    }
}

// 删除任务 - 练习向量操作
fn remove_task(todo_list: &mut Vec<String>) {
    if todo_list.is_empty() {
        println!("📭 没有任务可以删除！");
        return;
    }
    
    show_tasks(todo_list);
    println!("请输入要删除的任务编号:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    
    match input.trim().parse::<usize>() {
        Ok(index) => {
            if index > 0 && index <= todo_list.len() {
                let removed_task = todo_list.remove(index - 1);
                println!("🗑️ 已删除任务: '{}'", removed_task);
            } else {
                println!("❌ 无效的任务编号！");
            }
        }
        Err(_) => println!("❌ 请输入有效的数字！"),
    }
}

// 练习任务：
// 1. 添加任务优先级功能
// 2. 添加任务分类功能
// 3. 添加保存到文件的功能
// 4. 添加任务搜索功能
