use std::{
    collections::HashMap, env, fmt, fs, io::Write, os::unix::fs::PermissionsExt, path::PathBuf,
    process::exit,
};

#[derive(Debug)]
enum Action {
    Add,
    Remove,
    Done,
}

impl Action {
    fn run(&self, item: &str) {
        match self {
            Action::Add => self.add(item),
            Action::Remove => self.remove(item),
            Action::Done => self.done(item),
        }
    }
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

struct Todo {
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

fn create_default_todo(default_path: &PathBuf) {
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

fn read_todo() -> Result<HashMap<usize, Todo>, std::io::Error> {
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

    return Ok(todos);
}

fn main() {
    let mut args = env::args();
    args.next();
    let action = args.next();
    if let None = action {
        let todos = read_todo().unwrap_or_else(|e| {
            eprint!("Can't read todo file: {e}");
            exit(1);
        });

        if todos.len() == 0 {
            println!("There is not any job todo.")
        } else {
            for todo in todos {
                println!("{}: {} ({})", todo.0, todo.1.todo, todo.1.state);
            }
        }
        exit(1);
    }
    let action = action.unwrap().to_lowercase();
    let action = match &action[..] {
        "add" => Action::Add,
        "remove" => Action::Remove,
        "done" => Action::Done,
        _ => {
            eprint!("Wrong action type is given. Choose between [Add, Remove, Done]\n");
            exit(1);
        }
    };

    let new_job = args.next();
    if let None = new_job {
        let err_msg = match action {
            Action::Add => "Input a new job to add.\n",
            Action::Remove => "Input a job name or a number of a job to remove.\n",
            Action::Done => "Input a job name or a number of a job.\n",
        };
        eprint!("{err_msg}");
        exit(1);
    }

    let new_job: String = new_job.unwrap();
    action.run(&new_job[..]);

    // let todos = read_todo().unwrap_or_else(|e| {
    //     eprint!("Can't read todo file: {e}");
    //     exit(1);
    // });

    // if todos.len() == 0 {
    //     println!("There is not any job todo.")
    // } else {
    //     for todo in todos {
    //         println!("{}: {} ({})", todo.0, todo.1.todo, todo.1.state);
    //     }
    // }
}
