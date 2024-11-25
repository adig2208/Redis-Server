use std::io;

struct Task {
    description: String,
    status: bool
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            status: false,
        }
    }
}

fn main() {
    println!("Todo List");
    let mut task_list: Vec<Task> = Vec::new();
    loop {
        let task = get_input("What would you like to do? Choose from the below options:\n 1] Add a task\n 2] View all tasks\n 3] Update task status\n 4] Delete task\n 5] Exit");

        match task.as_str() {
            "1" => {
                let description = get_input("Add a task description.");
                task_list.push(Task::new(description.to_string()));
                println!("Task added successfully");
            }
            "2" => {
                if task_list.is_empty() {
                    println!("All tasks completed!")
                } else {
                    for (index, task) in task_list.iter().enumerate() {
                        let status = if task.status {"[x]"} else {"[]"};
                        println!("{}: {} {}", index+1, status, task.description)
                    }
                }
            }
            "3" => {
                if task_list.is_empty() {
                    println!("All tasks completed!");
                    continue;
                }
                for (index, task) in task_list.iter().enumerate() {
                    let status = if task.status {"[x]"} else {"[]"};
                    println!("{}: {} {}", index+1, status, task.description)
                }
                let val = get_input("Which task would you like to update?").trim().parse::<usize>();
                match val {
                    Ok(num) if num > 0 && num <= task_list.len() => {
                        let confirm = get_input("Do you want to mark this task as complete? (y/n): ");
                        if confirm.to_lowercase() == "y" {
                            task_list[num - 1].status = true;
                            println!("Task marked as complete.");
                        }
                    }
                    _ => println!("Invalid task number."),
                }
            }
            "4" => {
                if task_list.is_empty() {
                    println!("All tasks completed!");
                    continue;
                }
                for (index, task) in task_list.iter().enumerate() {
                    let status = if task.status {"[x]"} else {"[]"};
                    println!("{}: {} {}", index+1, status, task.description)
                }
                let val = get_input("Which task would you like to delete?").trim().parse::<usize>();
                match val {
                    Ok(num) if num > 0 && num <= task_list.len() => {
                        task_list.remove(num - 1);
                        println!("Task deleted successfully.");
                    }
                    _ => println!("Invalid task number."),
                }
            }
            "5" => {
                println!("Good Bye!");
                break;
            }
            _ => { println!("Invalid input");}
        };
    }
    
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Error reading input");
    s.trim().to_string()
}