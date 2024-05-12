use std::{env, process::exit};
use todo::{read_todo, Action};

fn main() {
    let mut args = env::args();
    args.next();
    let action = args.next();

    // If the action is empty, read todo list.
    if let None = action {
        read_todo().unwrap_or_else(|e| {
            eprint!("Can't read todo file: {e}");
            exit(1);
        });
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
}
