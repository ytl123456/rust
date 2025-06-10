// 第3周 - 学生成绩管理系统
// 学习目标：结构体、方法、关联函数、所有权深入

use std::io;

// 学生结构体
#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    scores: Vec<f64>,
}

// 为Student实现方法
impl Student {
    // 关联函数 - 创建新学生
    fn new(id: u32, name: String, age: u8) -> Student {
        Student {
            id,
            name,
            age,
            scores: Vec::new(),
        }
    }
    
    // 方法 - 添加成绩
    fn add_score(&mut self, score: f64) {
        if score >= 0.0 && score <= 100.0 {
            self.scores.push(score);
        }
    }
    
    // 方法 - 计算平均分
    fn average_score(&self) -> f64 {
        if self.scores.is_empty() {
            0.0
        } else {
            self.scores.iter().sum::<f64>() / self.scores.len() as f64
        }
    }
    
    // 方法 - 获取等级
    fn get_grade(&self) -> String {
        let avg = self.average_score();
        match avg {
            90.0..=100.0 => "优秀".to_string(),
            80.0..=89.9 => "良好".to_string(),
            70.0..=79.9 => "中等".to_string(),
            60.0..=69.9 => "及格".to_string(),
            _ => "不及格".to_string(),
        }
    }
    
    // 方法 - 显示学生信息
    fn display_info(&self) {
        println!("学号: {}", self.id);
        println!("姓名: {}", self.name);
        println!("年龄: {}", self.age);
        println!("成绩: {:?}", self.scores);
        println!("平均分: {:.2}", self.average_score());
        println!("等级: {}", self.get_grade());
        println!("总科目数: {}", self.scores.len());
    }
}

// 学生管理器结构体
struct StudentManager {
    students: Vec<Student>,
    next_id: u32,
}

impl StudentManager {
    // 关联函数 - 创建新的管理器
    fn new() -> StudentManager {
        StudentManager {
            students: Vec::new(),
            next_id: 1,
        }
    }
    
    // 方法 - 添加学生
    fn add_student(&mut self, name: String, age: u8) {
        let student = Student::new(self.next_id, name, age);
        self.students.push(student);
        println!("✅ 学生添加成功，学号: {}", self.next_id);
        self.next_id += 1;
    }
    
    // 方法 - 查找学生（借用）
    fn find_student(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|student| student.id == id)
    }
    
    // 方法 - 查找学生（可变借用）
    fn find_student_mut(&mut self, id: u32) -> Option<&mut Student> {
        self.students.iter_mut().find(|student| student.id == id)
    }
    
    // 方法 - 显示所有学生
    fn display_all_students(&self) {
        if self.students.is_empty() {
            println!("📭 暂无学生信息");
            return;
        }
        
        println!("\n📚 所有学生信息:");
        for student in &self.students {
            println!("\n--- 学生信息 ---");
            student.display_info();
        }
    }
    
    // 方法 - 计算班级平均分
    fn class_average(&self) -> f64 {
        if self.students.is_empty() {
            return 0.0;
        }
        
        let total: f64 = self.students
            .iter()
            .map(|student| student.average_score())
            .sum();
        
        total / self.students.len() as f64
    }
}

fn main() {
    println!("🎓 欢迎使用学生成绩管理系统！");
    
    let mut manager = StudentManager::new();
    
    loop {
        show_menu();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");
        
        match input.trim() {
            "1" => add_student_interactive(&mut manager),
            "2" => add_score_interactive(&mut manager),
            "3" => view_student_interactive(&manager),
            "4" => manager.display_all_students(),
            "5" => show_class_statistics(&manager),
            "6" => {
                println!("再见！👋");
                break;
            }
            _ => println!("❌ 无效选择，请重新输入！"),
        }
    }
}

fn show_menu() {
    println!("\n=== 学生管理系统菜单 ===");
    println!("1. 添加学生");
    println!("2. 添加成绩");
    println!("3. 查看学生信息");
    println!("4. 查看所有学生");
    println!("5. 班级统计");
    println!("6. 退出");
    println!("请选择操作 (1-6):");
}

fn add_student_interactive(manager: &mut StudentManager) {
    println!("请输入学生姓名:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("读取输入失败");
    let name = name.trim().to_string();
    
    if name.is_empty() {
        println!("❌ 姓名不能为空！");
        return;
    }
    
    println!("请输入学生年龄:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("读取输入失败");
    
    match age_input.trim().parse::<u8>() {
        Ok(age) => {
            if age >= 6 && age <= 100 {
                manager.add_student(name, age);
            } else {
                println!("❌ 年龄应在6-100之间！");
            }
        }
        Err(_) => println!("❌ 请输入有效的年龄！"),
    }
}

fn add_score_interactive(manager: &mut StudentManager) {
    println!("请输入学生学号:");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).expect("读取输入失败");
    
    match id_input.trim().parse::<u32>() {
        Ok(id) => {
            match manager.find_student_mut(id) {
                Some(student) => {
                    println!("请输入成绩 (0-100):");
                    let mut score_input = String::new();
                    io::stdin().read_line(&mut score_input).expect("读取输入失败");
                    
                    match score_input.trim().parse::<f64>() {
                        Ok(score) => {
                            if score >= 0.0 && score <= 100.0 {
                                student.add_score(score);
                                println!("✅ 成绩添加成功！");
                            } else {
                                println!("❌ 成绩应在0-100之间！");
                            }
                        }
                        Err(_) => println!("❌ 请输入有效的成绩！"),
                    }
                }
                None => println!("❌ 未找到学号为 {} 的学生！", id),
            }
        }
        Err(_) => println!("❌ 请输入有效的学号！"),
    }
}

fn view_student_interactive(manager: &StudentManager) {
    println!("请输入学生学号:");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).expect("读取输入失败");
    
    match id_input.trim().parse::<u32>() {
        Ok(id) => {
            match manager.find_student(id) {
                Some(student) => {
                    println!("\n--- 学生详细信息 ---");
                    student.display_info();
                }
                None => println!("❌ 未找到学号为 {} 的学生！", id),
            }
        }
        Err(_) => println!("❌ 请输入有效的学号！"),
    }
}

fn show_class_statistics(manager: &StudentManager) {
    if manager.students.is_empty() {
        println!("📭 暂无学生数据进行统计");
        return;
    }
    
    println!("\n📊 班级统计信息:");
    println!("学生总数: {}", manager.students.len());
    println!("班级平均分: {:.2}", manager.class_average());
    
    // 统计各等级人数
    let mut excellent = 0;
    let mut good = 0;
    let mut average = 0;
    let mut pass = 0;
    let mut fail = 0;
    
    for student in &manager.students {
        match student.get_grade().as_str() {
            "优秀" => excellent += 1,
            "良好" => good += 1,
            "中等" => average += 1,
            "及格" => pass += 1,
            "不及格" => fail += 1,
            _ => {}
        }
    }
    
    println!("优秀: {} 人", excellent);
    println!("良好: {} 人", good);
    println!("中等: {} 人", average);
    println!("及格: {} 人", pass);
    println!("不及格: {} 人", fail);
}

// 练习任务：
// 1. 添加删除学生功能
// 2. 添加修改学生信息功能
// 3. 添加成绩排序功能
// 4. 添加保存到文件功能
