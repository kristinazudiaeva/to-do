use std::io::{self, Write};

struct Task {
    description: String,
    done: bool,
}

struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    fn new() -> Todo {
        Todo { tasks: Vec::new() }
    }

    fn add_task (&mut self, description: String) {
        let task = Task {
            description,
            done: false,
        };
        self.tasks.push(task);
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
        }
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        }
    }

    fn edit_task(&mut self, index: usize, description: String) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.description = description;
        }
    }

    fn print_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[✓]" } else { "[✕]" };
            println!("{}. {} {}", i + 1, status, task.description);
        }
    }
}

fn main() {
    let mut todo = Todo::new();

    loop {
        print!("Введите команды(add/complete/remove/edit/print/exit): ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command = command.trim();
        match command {
            "add" => {
                print!("Добавьте задачу: ");
                io::stdout().flush().unwrap();

                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                todo.add_task(description.trim().to_string());
                println!("Задача выполнена!");
            }
            "complete" => {
                print!("Введите номер задачи: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let index = index.trim().parse::<usize>().unwrap_or(0);
                todo.complete_task(index - 1);
                println!("Задача отмечена!");
            }
            "remove" => {
                print!("Введите номер задачи: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let index = index.trim().parse::<usize>().unwrap_or(0);
                todo.remove_task(index - 1);
                println!("Задача выполнена!");
            }
            "edit" => {
                print!("Введите номер задачи для редактирования: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let index = index.trim().parse::<usize>().unwrap_or(0);
                print!("Введите новую задачу: ");
                io::stdout().flush().unwrap();

                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                todo.edit_task(index - 1, description.trim().to_string());
                println!("Задача выполнена!");
            }
            "print" => {
                println!("Список задач:");
                todo.print_tasks();
            }
            "exit" => break,
            _ => println!("Ошибка"),
        }

        println!();
    }
}
