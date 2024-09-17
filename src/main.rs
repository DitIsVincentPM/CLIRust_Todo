use std::io;

struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            done: false,
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    println!("Welcome to the TODO app.");

    loop {
        println!("\n1. Add task");
        println!("2. List tasks");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => {
                println!("Goodbye");
                break;
            }
            _ => println!("Invalid choice.")
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter a task description:");

    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");
    let description = description.trim().to_string();

    let task = Task::new(description);
    tasks.push(task);
    println!("Task added");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.done { "Done" } else { "Not Done" };
            println!("{}. {} [{}]", index + 1, task.description, status);
        }
    }
}

fn remove_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to remove.");
        return;
    }

    println!("Enter the number of the task to remove:");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    if index == 0 || index > tasks.len() {
        println!("Task number out of range.");
    } else {
        tasks.remove(index - 1);
        println!("Task removed!");
    }
}
