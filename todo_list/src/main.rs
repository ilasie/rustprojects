use std::{ io::{ stdin, Write, stdout }, fmt, process::exit };

struct Task { about: String, completed: bool }

impl Task {
    fn build(about: &str, completed: bool) -> Self {
        Task {
            about: about.to_string(),
            completed
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.completed {
            write!(f, "[x] ")?;
        } else {
            write!(f, "[ ] ")?;
        }
        write!(f, "{}", self.about)
    }
}

struct TodoList { tasks: Vec<Task> }

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn show_all(&self) {
        println!("HERE your todo-list:");
        let mut cnt: usize = 1;
        for task in &self.tasks {
            println!("{}. {}", cnt, task);
            cnt += 1;
        }
    }

    fn tick(&mut self, task_id: usize) {
        if task_id > self.tasks.len() {
            eprintln!("Error: Found task does not exist.");
        }
        self.tasks[task_id-1].completed = !self.tasks[task_id-1].completed;
    }

    fn delete_task(&mut self, task_id: usize) {
        if task_id > self.tasks.len() {
            eprintln!("Error: Found task does not exist.");
        }
        self.tasks.remove(task_id-1);
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    
    // To extend: TodoList::from_file()
    //
    // println!("HERE Your TODO List:");
    // todo_list.show_all();

    println!("
Usage:
    a <about>          Add a Task
    d <task_id>        Delete a task
    t <task_id>        Change the completed of a task
    s                  Show your todo-list
    q                  Quit
    V                  Print version
    h                  Print usage");
    print!("> ");
    stdout().flush().expect("Error: Cannot flush stdout.");

    loop {
        let mut cmd = String::new();
        stdin().read_line(&mut cmd).expect("Error: Cannot read input.");

        match cmd.chars().next() {
            Some(method) => {
                match method {
                    'a' => {
                        match cmd.trim().split_once(' ') {
                            Some((_, about)) => {
                                todo_list.add_task(
                                    Task::build(about.trim_matches('"'),
                                    false)
                                );
                                println!("Success to add a task!");
                            }
                            None => println!("Please check your input.")
                        }
                    }
                    'd' => {
                        match cmd.trim().split_once(' ') {
                            Some((_, task_id)) => {
                                let task_id: usize = task_id.parse()
                                    .expect("Error: Cannot get task id");
                                todo_list.delete_task(task_id);
                                println!("Success to delete a task!");
                            }
                            None => println!("Please check your input.")
                        }
                    }
                    't' => {
                        match cmd.trim().split_once(' ') {
                            Some((_, task_id)) => {
                                let task_id: usize = task_id.parse()
                                    .expect("Error: Cannot get task id");
                                todo_list.tick(task_id);
                                println!("Success to tick a task!");
                            }
                            None => println!("Please check your input.")
                        }
                    }
                    's' => todo_list.show_all(),
                    'q' => exit(0),
                    'V' => println!("{}", env!("CARGO_PKG_VERSION")),
                    'h' => {
                        println!("
Usage:
    a <about>          Add a Task
    d <task_id>        Delete a task
    t <task_id>        Change the completed of a task
    q                  Quit
    V                  Print version
    h                  Print usage");
                    }
                    _ => println!("No such command.")
                };
            }
            None => {}
        };
        print!("> ");
        stdout().flush().expect("Error: Cannot flush stdout.");
    }
}
