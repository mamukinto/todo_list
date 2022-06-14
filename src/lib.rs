use std::{io::{self, Write, Error}, fs::OpenOptions};


pub fn run(show_index : bool) {
    let mut tasks = load_from_saved().unwrap_or_default();
    //read the user input

    println!("Here are all the items:");
    let mut index = 0;
    for task in &tasks {
        if show_index {
            println!("{} {}", index, task);
        } else {
            println!("{}", task);
        }
        index += 1;
    }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    

    //parse the user input
    let command = input.trim();
    let mut command_split = command.split_whitespace();
    let command_name = command_split.next().unwrap();
    // let command_args = command_split.collect::<Vec<&str>>();
    let command_argument = command_split.collect::<Vec<&str>>().join(" ");
    match command_name.to_lowercase().as_str() {
        "add" => {
            tasks.push(Task::new(command_argument.to_string(), false,None));
            save(&tasks);
            run(show_index);
        }
        "sub" => {
            tasks.push(Task::new(command_argument.to_string(), true,Some(get_index_of_previous_main_task(&tasks))));
            save(&tasks);
            run(show_index);
        }
        "remove" => {
            let index = command_argument.parse::<usize>().unwrap();
            remove_task(&mut tasks, index);
            save(&tasks);
            run(show_index);
        }
        "done" => {
            let index = command_argument.parse::<usize>().unwrap();
            tasks[index].complete();
            mark_subtasks_as_done(&mut tasks, index);
            //println!("{}",&tasks[index]);
            save(&tasks);
            run(show_index);
        }
        "undone" => {
            let index = command_argument.parse::<usize>().unwrap();
            tasks[index].uncomplete();
            save(&tasks);
            run(show_index);
        }
        "doneall" => {
            tasks.iter_mut().for_each(|task| task.complete());
            save(&tasks);
            run(show_index);
        }
        "removeall" => {
            tasks.clear();
            save(&tasks);
            run(show_index);
        }
        "toggle_index" => {
            run(!show_index);
        }
        "help" => {
            println!("--------------------------------------------------------------------------------");
            println!("add <task> - add a new task");
            println!("sub <task> - add a new subtask");
            println!("remove <index> - remove a task");
            println!("done <index> - mark a task as done");
            println!("undone <index> - mark a task as undone");
            println!("removeall - remove all tasks");
            println!("doneall - mark all tasks as done");
            println!("toggle_index - toggle the index display");
            println!("exit - exit the program");
            println!("help - show this help");
            println!("--------------------------------------------------------------------------------");
            run(show_index);
        }
        "exit" => {
            println!("Goodbye!");
            return;
        }
        _ => {
            println!("Unknown command");
            run(show_index);
        }
    }
}

fn get_index_of_previous_main_task(tasks : &Vec<Task>) -> usize {
    //find the last non sub task
    let mut index = tasks.len() - 1;
    while tasks[index].is_sub() {
        index -= 1;
    }
    index
}

fn remove_task(tasks : &mut Vec<Task>, index : usize) {
    // found subtasks and remove them
    let mut subtasks = Vec::new();
    for task in tasks.iter() {
        if task.is_sub() && task.parent_index() == Some(index) {
            subtasks.push(tasks.iter().position(|x| *x == *task).unwrap());
        }
    }
    subtasks.sort();
    subtasks.reverse();
    for subtask in subtasks {
        remove_task(tasks, subtask);
    }
    //remove the task
    tasks.remove(index);

}

fn mark_subtasks_as_done(tasks : &mut Vec<Task>, index : usize) {
    for task in &mut tasks[index..] {
        if task.is_sub() && task.parent_index().unwrap() == index {
            task.complete();
        }
    }
}

pub fn load_from_saved() -> Result<Vec<Task>,Error> {

    let mut tasks: Vec<Task> = Vec::new();

    let contents = std::fs::read_to_string("tasks.txt")?;
    for line in contents.lines() {
        let task = Task::from_str(line);
        if let Some(task) = task {
            tasks.push(task);
        } else {
            println!("Failed to parse task: {}", line);
        }
    }
    Ok(tasks)
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

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Task {
    name: String,
    is_sub: bool,
    is_completed: bool,
    parent: Option<usize>,  //index of parent task
}
impl Task {
    pub fn new(name: String, is_sub: bool, parent: Option<usize>) -> Task {
        Task {
            name: name,
            is_sub: is_sub,
            is_completed: false,
            parent: parent,
        }
    }
    pub fn complete(&mut self) {
        self.is_completed = true;
    }
    pub fn uncomplete(&mut self) {
        self.is_completed = false;
    }
    pub fn is_completed(&self) -> bool {
        self.is_completed
    }
    pub fn is_sub(&self) -> bool {
        self.is_sub
    }
    pub fn make_sub(&mut self) {
        self.is_sub = true;
    }
    pub fn parent_index(&self) -> Option<usize> {
        self.parent
    }
    pub fn from_str(s: &str) -> Option<Task> {
        let mut parts = s.split(",");
        let name = parts.next()?;
        let is_sub = parts.next()?.parse::<bool>().ok()?;
        let completed = parts.next()?.parse::<bool>().ok()?;
        let parent = parts.next()?.parse::<usize>().ok()?;
        Some(Task { name: name.to_string(), is_sub: is_sub, is_completed: completed, parent: Some(parent) })
    }
    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.name, self.is_sub, self.is_completed, self.parent.unwrap_or(usize::MAX))
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix_str = if self.is_sub { " -> " } else { "" };
        if self.is_completed() {
            write!(f, "{}[x] {}",prefix_str, self.name)
            
        } else {
            write!(f, "{}[ ] {}",prefix_str, self.name)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{Task, get_index_of_previous_main_task, mark_subtasks_as_done, remove_task};

    #[test]
    fn task_to_string_works_properly() {
        let task = Task::new("Test".to_string(), true, None);
        assert_eq!("Test,true,false,18446744073709551615",task.to_string());
    }
    #[test]
    fn subtask_to_string_works_properly() {
        let task = Task::new("Test".to_string(), true, Some(0));
        assert_eq!("Test,true,false,0",task.to_string());
    }
    #[test]
    fn task_from_string_works_properly() {
        let task = Task::from_str("Test,false,false");
        if let Some(task) = task {
            assert_eq!("Test",task.name);
            assert_eq!(false,task.is_sub);
            assert_eq!(false,task.is_completed());
        }
    }
    #[test]
    fn task_from_string_fails_properly() {
        let task = Task::from_str("T,e,s,t,,,Te,s,,t");
        assert!(task.is_none());
    }

    #[test]
    fn task_is_completed_works_properly() {
        let mut task = Task::new("Test".to_string(), true, None);
        task.complete();
        assert_eq!(true,task.is_completed());
    }

    #[test]
    fn task_display_works_properly() {
        let mut task = Task::new("Test".to_string(), false, None);
        assert_eq!("[ ] Test",format!("{}",task));
        task.complete();
        assert_eq!("[x] Test",format!("{}",task));
    }
    #[test]
    fn sub_task_display_works_properly() {
        let mut task = Task::new("Test".to_string(), true, None);
        assert_eq!(" -> [ ] Test",format!("{}",task));
        task.complete();
        assert_eq!(" -> [x] Test",format!("{}",task));
    }
    #[test]
    fn get_index_of_previous_main_task_works_properly() {
        let mut tasks = Vec::new();
        tasks.push(Task::new("main".to_string(), false, None));
        tasks.push(Task::new("sub".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        
        tasks.push(Task::new("main1".to_string(), false, None));
        tasks.push(Task::new("sub1".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        
        assert_eq!(Some(0), tasks[1].parent);
        assert_eq!(Some(2), tasks[3].parent);
    }


    #[test]
    fn main_task_also_completes_sub_tasks() {
        let task = Task::new("Test".to_string(), false, None);
        let mut tasks = vec![task];
        let sub_task = Task::new("Sub".to_string(), true, Some(get_index_of_previous_main_task(&tasks)));
        let sub_task2 = Task::new("Sub2".to_string(), true, Some(get_index_of_previous_main_task(&tasks)));
        tasks.push(sub_task);
        tasks.push(sub_task2);
        tasks[0].complete();
        mark_subtasks_as_done(&mut tasks, 0);
        assert!(tasks[1].is_completed());
        assert!(tasks[2].is_completed());
    }

    #[test]
    fn removing_a_task_works_properly() {
        let mut tasks = Vec::new();
        tasks.push(Task::new("main".to_string(), false, None));
        tasks.push(Task::new("sub".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("main1".to_string(), false, None));
        tasks.push(Task::new("sub1".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        remove_task(&mut tasks, 0);
        assert_eq!(2,tasks.len()); 
    }
    #[test]
    fn removing_a_task_with_multiple_sub_tasks_works_properly() {
        let mut tasks = Vec::new();
        tasks.push(Task::new("main".to_string(), false, None));
        tasks.push(Task::new("sub".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("main1".to_string(), false, None));
        tasks.push(Task::new("sub1".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub2".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub3".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub4".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub5".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub6".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub7".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub8".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub9".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));
        tasks.push(Task::new("sub10".to_string(), true, Some(get_index_of_previous_main_task(&tasks))));

        remove_task(&mut tasks, 2);


        assert_eq!(2,tasks.len());
    }
}