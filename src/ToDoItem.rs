use std::io::stdin;

struct ToDoItem{
    description: String,
    completed:bool
}

pub(crate) fn main() {
    let mut todos: Vec<ToDoItem> = Vec::new();

    loop {
        println!("\n请选择一个操作:");
        println!("1. 添加待办事项");
        println!("2. 标记待办事项为已完成");
        println!("3. 显示所有待办事项");
        println!("4. 退出");

        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("读取失败");
        let command = command.trim(); // 移除首尾空白，包括换行符

        match command {
            "1" => add_todo(&mut todos), // 注意这里，你需要传递什么？
            "2" => complete_todo(&mut todos),
            "3" => show_todos(&todos),
            "4" => {
                println!("感谢使用！");
                break;
            }
            _ => println!("无效的命令，请重新输入。"),
        }
    }
}

fn show_todos(todoList: &Vec<ToDoItem>) {
    if todoList.len() == 0 {
        println!("没有待办事项。");
    }
    else{
        println!("代办事项列表：");
        for (i,todo) in todoList.iter().enumerate(){
            println!("{}, 描述: {}, 状态: {}", i + 1, todo.description, if todo.completed { "已完成" } else { "未完成" });
        }
    }
}

fn complete_todo(p0: &mut Vec<ToDoItem>) {
    if p0.len() == 0{
        println!("没有待办事项可以标记为已完成。");
        return;
    }
    else{
        for(i,todo) in p0.iter_mut().enumerate(){
            if todo.completed == false{
                todo.completed = true;
                println!("已将待办事项 {} 标记为已完成。", i + 1);
            }
            else{
                println!("待办事项 {} 已经完成。", i + 1);
            }
        }
        println!("所有待办事项已标记为已完成。");
    }
}

fn add_todo(todos: &mut Vec<ToDoItem> ){
    println!("请输入代办");
    let mut description = String::new();
    stdin().read_line(&mut description).expect("读取失败");
    let description = description.trim().to_string(); // 移除首尾空白，包括换行符
    if description.len() == 0 {
        println!("代办事项不能为空，请重新输入。");
        return;
    }
    else{
        todos.push(ToDoItem{description,completed:false});
    }
}

fn process_and_return_new_list(mut original_todos:Vec<ToDoItem>) -> Vec<ToDoItem>{
    println!("正在处理待办事项列表...");

    let mut unfinised_todos: Vec<ToDoItem> = Vec::new();
    //可以使用into_iter()来获取所有权并消耗原始列表
    //需要注意的是，他会把原始的Vec全部消费
    // for todo in original_todos.into_iter(){
    //     if !todo.completed{
    //         unfinised_todos.push(todo);
    //     }
    // }
    //也可以使用drain()方法来获取所有权并消耗原始列表
    for todo in original_todos.drain(..){
        if !todo.completed {
            unfinised_todos.push(todo);
        }
    }
    unfinised_todos

}

pub(crate) fn output(){
    let inital_tasks = vec![ToDoItem{description:String::from("学习 Rust"), completed: false},
                            ToDoItem{description:String::from("完成项目"), completed: false}];


    println!("原始列表：{:?}", inital_tasks.len());
    //转移inital_task的所有权
    let  manager = ToDoListManager::new(inital_tasks);
    let filtered_tasks = process_and_return_new_list(manager.tasks);

    println!("处理后的待办事项列表：{:?}", filtered_tasks.len());
}

struct ToDoListManager {
    tasks: Vec<ToDoItem>
}

impl ToDoListManager{
    fn new(inital_tasks: Vec<ToDoItem>) -> Self{
        println!("正在初始化待办事项管理器...");
        ToDoListManager{
            tasks: inital_tasks
        }
    }

    fn add_task(&mut self, todoItem: ToDoItem){
        self.tasks.push(todoItem);
        println!("已添加待办事项");
    }

    fn show_tasks(&self) {
        if self.tasks.is_empty() {
            println!("没有待办事项。");
        } else {
            println!("待办事项列表：");
            for (i, task) in self.tasks.iter().enumerate() {
                println!("{}. 描述: {}, 状态: {}", i + 1, task.description, if task.completed { "已完成" } else { "未完成" });
            }
        }
    }
}