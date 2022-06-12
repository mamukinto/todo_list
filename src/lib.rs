use std::{io::{self, Write}, fs::OpenOptions};


pub fn run() {
    let mut tasks = load_from_saved();
    //read the user input

    println!("Here are all the items:");
    for task in &tasks {
        println!("{}", task);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    

    //parse the user input
    let command = input.trim();
    let mut command_split = command.split_whitespace();
    let command_name = command_split.next().unwrap();
    // let command_args = command_split.collect::<Vec<&str>>();
    let command_argument = command_split.collect::<Vec<&str>>().join(" ");
    match command_name {
        "add" => {
            tasks.push(Task::new(command_argument.to_string(),"?".to_string()));
            save(&tasks);
            run();
        }
        "remove" => {
            let index = command_argument.parse::<usize>().unwrap();
            tasks.remove(index);
            save(&tasks);
            run();
        }
        "done" => {
            let index = command_argument.parse::<usize>().unwrap();
            tasks[index].complete();
            //println!("{}",&tasks[index]);
            save(&tasks);
            run();
        }
        "removeall" => {
            tasks.clear();
            save(&tasks);
            run();
        }
        "exit" => {
            println!("Goodbye!");
            return;
        }
        _ => {
            println!("Unknown command");
            run();
        }
    }
}

pub fn load_from_saved() -> Vec<Task> {

    let mut tasks: Vec<Task> = Vec::new();
    
    if std::fs::read_to_string("tasks.txt").is_err() {
        return tasks;
    }
    let contents = std::fs::read_to_string("tasks.txt").unwrap();
    for line in contents.lines() {
        let task = Task::from_str(line);
        tasks.push(task);
    }
    // Return an empty vector
    tasks
}

pub fn save(tasks : &Vec<Task>) {
    //write the tasks to existing file tasks.txt
    let mut file = OpenOptions::new()
    .read(true)
    .write(true) 
    .truncate(true)
    .create(true)
    .open("tasks.txt")
    .unwrap();
    for task in tasks {
        if let Err(e) = writeln!(file, "{}",task.to_string()) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

use core::fmt;

pub struct Task {
    pub name: String,
    pub description: String,
    pub completed: bool,
}
impl Task {
    pub fn new(name: String, description: String) -> Task {
        Task {
            name: name,
            description: description,
            completed: false,
        }
    }
    pub fn complete(&mut self) {
        self.completed = true;
    }
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    pub fn from_str(s: &str) -> Task {
        let mut parts = s.split(",");
        let name = parts.next().unwrap();
        let description = parts.next().unwrap();
        let completed = parts.next().unwrap().parse::<bool>().unwrap();
        Task { name: name.to_string(), description: description.to_string(), completed: completed }
    }
    pub fn to_string(&self) -> String {
        format!("{},{},{}", self.name, self.description, self.completed)
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_completed() {
            write!(f, "[x] {}", self.name)
            
        } else {
            write!(f, "[ ] {}", self.name)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::Task;

    #[test]
    fn task_to_string_works_properly() {
        let task = Task::new("Test".to_string(), "Test".to_string());
        assert_eq!("Test,Test,false",task.to_string());
    }
    #[test]
    fn task_from_string_works_properly() {
        let task = Task::from_str("Test,Test,false");
        assert_eq!("Test",task.name);
        assert_eq!("Test",task.description);
        assert_eq!(false,task.completed);
    }

    #[test]
    fn task_is_completed_works_properly() {
        let mut task = Task::new("Test".to_string(), "Test".to_string());
        task.complete();
        assert_eq!(true,task.is_completed());
    }

    #[test]
    fn task_display_works_properly() {
        let mut task = Task::new("Test".to_string(), "Test".to_string());
        assert_eq!("[ ] Test",format!("{}",task));
        task.complete();
        assert_eq!("[x] Test",format!("{}",task));
    }
}