use regex::Regex;
use std::{
    fmt, fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::exit,
};

pub const DEFAULT_PATH: &str = ".todo/todo_list.txt";

#[derive(Debug)]
pub enum Action {
    Add,
    Remove,
    Done,
}

pub struct Todo {
    pub todo: String,
    pub state: TodoState,
}

pub enum TodoState {
    Done,
    NotYet,
}

pub fn update_file(todo_table: &Vec<Todo>) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(DEFAULT_PATH)
        .expect("Can't open the todo list");

    let mut contents = String::new();
    for (idx, todo) in todo_table.iter().enumerate() {
        contents.push_str(format!("{}: {} ({})\n", idx, todo.todo, todo.state).as_str());
    }

    if let Err(e) = writeln!(file, "{contents}") {
        eprintln!("Couldn't write in the file: {}", e);
    }
}

impl Action {
    pub fn run(&self, todo_table: &mut Vec<Todo>, item: &str) {
        match self {
            Action::Add => self.add(todo_table, item),
            Action::Remove => self.remove(todo_table, item),
            Action::Done => self.done(todo_table, item),
        }
    }

    // TODO: Write on the first line if the file is empty. Currently, it writes from the second line.
    fn add(&self, todo_table: &mut Vec<Todo>, item: &str) {
        todo_table.push(Todo {
            todo: String::from(item),
            state: TodoState::NotYet,
        });
    }

    // TODO: Remove jobs only with indexes.
    fn remove(&self, todo_table: &mut Vec<Todo>, item: &str) {
        let mut key = item
            .parse::<usize>()
            .expect("Input only index numbers of jobs");
        key -= 1;
        todo_table.remove(key);
    }

    // TODO: Finish jobs only with indexes.
    fn done(&self, todo_table: &mut Vec<Todo>, item: &str) {
        let mut key = item
            .parse::<usize>()
            .expect("Input only index numbers of jobs");
        key -= 1;

        if key > todo_table.len() {
            println!("Input correct index numbers of jobs");
            exit(1);
        }
        todo_table[key].state = TodoState::Done;
    }
}

impl fmt::Display for TodoState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoState::Done => write!(f, "Done"),
            TodoState::NotYet => write!(f, "Not yet"),
        }
    }
}

fn create_default_todo() {
    // Create default todo list.
    //
    // Permission of the todo list is 666.
    let default_path = PathBuf::from(DEFAULT_PATH);
    if default_path.parent().is_some() {
        if let Err(e) = fs::create_dir_all(default_path.parent().unwrap()) {
            eprintln!("[Error]: {}", e);
            exit(1);
        }
    }

    fs::write(&default_path, "").unwrap_or_else(|e| {
        eprintln!("[Error] Can't create default todo file: {}", e);
        exit(1);
    });

    let mut perm = fs::metadata(&default_path).unwrap().permissions();
    perm.set_mode(0o666);
    fs::set_permissions(&default_path, perm).unwrap();

    println!(
        "Created a default todo list at {}",
        default_path.as_path().to_str().unwrap()
    );
}

pub fn create_todo_table() -> Result<Vec<Todo>, io::Error> {
    let default_path = PathBuf::from(DEFAULT_PATH);

    if let Ok(exists) = default_path.try_exists() {
        if !exists {
            create_default_todo();
        }
    }

    let file = fs::read_to_string(&default_path)?;
    let lines = file.lines();
    let mut todo_table: Vec<Todo> = Vec::new();
    let re = Regex::new(r"([0-9]+): (.+) \((.+)\)").unwrap();

    let mut need_update = false;
    for line in lines.into_iter() {
        if !line.is_empty() {
            let caps = re.captures(line).unwrap();
            let state = match caps[3].to_lowercase().as_str() {
                "not yet" => TodoState::NotYet,
                "done" => TodoState::Done,
                _ => {
                    println!(
                        r#"Warning: The state "{}" of a job "{}" has been changed to "{}" since it is not supported state "#,
                        &caps[3], &caps[2], "Not yet"
                    );
                    need_update = true;
                    TodoState::NotYet
                }
            };

            todo_table.push(Todo {
                todo: String::from(&caps[2]),
                state,
            });
        }
    }

    if need_update {
        update_file(&todo_table);
    }

    Ok(todo_table)
}

pub fn read_todo(todo_table: &[Todo]) {
    if todo_table.is_empty() {
        println!("There is not any job todo.")
    } else {
        for (idx, todo) in todo_table.iter().enumerate() {
            println!("{}: {} ({})", idx + 1, todo.todo, todo.state,);
        }
    }
}
