use std::{
    collections::HashMap, fmt, fs, io::Write, os::unix::fs::PermissionsExt, path::PathBuf,
    process::exit,
};

#[derive(Debug)]
pub enum Action {
    Add,
    Remove,
    Done,
}

impl Action {
    pub fn run(&self, item: &str) {
        match self {
            Action::Add => self.add(item),
            Action::Remove => self.remove(item),
            Action::Done => self.done(item),
        }
    }
    // TODO: Write on the first line if the file is empty. Currently, it writes from the second line.
    fn add(&self, item: &str) {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(DEFAULT_PATH)
            .expect("Can't open the todo list");

        if let Err(e) = writeln!(file, "{item}") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    fn remove(&self, item: &str) {}
    fn done(&self, item: &str) {}
}

pub struct Todo {
    todo: String,
    state: TodoState,
}

enum TodoState {
    Done,
    NotYet,
}

impl fmt::Display for TodoState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoState::Done => write!(f, "Done"),
            TodoState::NotYet => write!(f, "Not yet"),
        }
    }
}

const DEFAULT_PATH: &str = ".todo/todo_list.txt";

pub fn create_default_todo(default_path: &PathBuf) {
    // Create default todo list.
    //
    // Permission of the todo list is 666.
    if let Some(_) = &default_path.parent() {
        if let Err(e) = fs::create_dir_all(&default_path.parent().unwrap()) {
            eprintln!("[Error]: {}", e);
            exit(1);
        }
    }

    fs::write(&default_path, "").unwrap_or_else(|e| {
        eprintln!("[Error] Can't create default todo file: {}", e);
        exit(1);
    });
    println!(
        "Created a default todo list at {}",
        default_path.as_path().to_str().unwrap()
    );

    let mut perm = fs::metadata(&default_path).unwrap().permissions();
    perm.set_mode(0o666);
    fs::set_permissions(&default_path, perm).unwrap();
    println!("Create todo list!");
}

pub fn read_todo() -> Result<(), std::io::Error> {
    let default_path = PathBuf::from(DEFAULT_PATH);

    if let Ok(exists) = default_path.try_exists() {
        if !exists {
            create_default_todo(&default_path);
        }
    } else {
        eprintln!("Can't check whether the todo list exists.");
        exit(1);
    }

    let file = fs::read_to_string(&default_path)?;
    let lines = file.lines();
    let mut todos = HashMap::new();
    let mut idx = 1;
    for line in lines.into_iter() {
        if line.len() > 0 {
            todos.insert(
                idx,
                Todo {
                    todo: String::from(line),
                    state: TodoState::NotYet,
                },
            );
            idx += 1;
        }
    }

    if todos.len() == 0 {
        println!("There is not any job todo.")
    } else {
        for todo in todos {
            println!("{}: {} ({})", todo.0, todo.1.todo, todo.1.state);
        }
    }

    Ok(())
}
