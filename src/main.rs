struct Task {
    complete: bool,
    description: String,
    id: i8,
}

fn main() {
    let mut input: String;
    let mut task_list: Vec<Task> = Vec::new();
    let mut task_id: i8 = 0;

    loop {
        input = String::new();
        println!("Enter a command: ");
        std::io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command == "quit" {
            break;
        } else if command == "list" {
            for task in &task_list {
                println!("{}: {}", task.id, task.description);
            }
        } else if command == "add" {
            input = String::new();
            println!("Enter task description: ");
            std::io::stdin().read_line(&mut input).unwrap();

            let description = input.trim();

            task_list.push(Task {
                complete: false,
                description: description.to_string(),
                id: task_id,
            });

            task_id += 1;
        } else if command == "complete" {
            input = String::new();
            println!("Enter task id: ");
            std::io::stdin().read_line(&mut input).unwrap();

            let id: i8 = input.trim().parse().unwrap();

            for task in &mut task_list {
                if task.id == id {
                    task.complete = true;
                }
            }
        } else if command == "remove" {
            input = String::new();
            println!("Enter task id: ");
            std::io::stdin().read_line(&mut input).unwrap();

            let id: i8 = input.trim().parse().unwrap();

            task_list.retain(|task| task.id != id);
        } else {
            println!("Invalid command");
        }
    }
}

