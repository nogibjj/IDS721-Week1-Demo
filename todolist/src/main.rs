use std::io;

struct Task {
    name: String,
    completed: bool,
}

fn main() {
    let mut tasks = vec![];

    loop {
        println!("Please select an option:");
        println!("1. List tasks");
        println!("2. Add task");
        println!("3. Mark task as completed");
        println!("4. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => list_tasks(&tasks),
            "2" => add_task(&mut tasks),
            "3" => mark_task(&mut tasks),
            "4" => break,
            _ => println!("Invalid input"),
        }
    }
}

fn list_tasks(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {} - {}", i + 1, task.name, task.completed);
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter task name:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    tasks.push(Task {
        name: input.to_string(),
        completed: false,
    });
}

fn mark_task(tasks: &mut Vec<Task>) {
    println!("Enter task number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let task_num = match input.parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    if task_num > 0 && task_num <= tasks.len() {
        let task = &mut tasks[task_num - 1];
        task.completed = true;
    } else {
        println!("Invalid task number");
    }
}
