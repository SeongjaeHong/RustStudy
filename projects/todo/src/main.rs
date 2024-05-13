use std::{env, process::exit};
use todo::{create_todo_table, read_todo, Action};

fn main() {
    let mut todo_table = create_todo_table().unwrap();

    let mut args = env::args();
    args.next();
    let action = args.next();

    // If the action is empty, read todo list.
    if let None = action {
        read_todo(todo_table);
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

    if args.len() == 0 {
        let err_msg = match action {
            Action::Add => "Input a new job to add.\n",
            Action::Remove => "Input a job name or a number of a job to remove.\n",
            Action::Done => "Input a job name or a number of a job.\n",
        };
        eprint!("{err_msg}");
        exit(1);
    }

    for _ in 0..args.len() {
        let new_job = args.next().unwrap();
        action.run(&todo_table, &new_job[..]);
    }
}
