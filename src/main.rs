
mod io;
mod types;
mod output;

use termion::{color, style};
use std::iter::FromIterator;
use std::env;

fn main()
{
    let file_path = format!("{}/.cli-todo", dirs::home_dir().unwrap().display());
    let mut tasks: Vec<Task> =  load_tasks_from_disk(&file_path);

    let args: Vec<String> = env::args().collect();

    if args.len() == 1
    {
        print_tasks(&tasks);
    }
    else if args[1] == "create"
    {
        if args.len() == 3
        {
            create_task(&mut tasks, &args[2]);
            save_tasks_to_disk(&tasks, &file_path);
        }
        else
        {
            println!("A task title must be supplied.");
            return;
        }
    }
    else if args[1] == "status"
    {
        if args.len() == 4
        {
            set_task_status(&mut tasks, &args[2], &args[3]);
            save_tasks_to_disk(&tasks, &file_path);
        }
    }

}

fn print_tasks(tasks: &Vec<Task>)
{
    let spacing = 2;

    let mut statuses: Vec<String> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    let mut ids: Vec<String> = Vec::new();

    statuses.push(String::from("Status"));
    titles.push(String::from("Title"));
    ids.push(String::from("ID"));

    let mut statuses_col_len = statuses[0].len();
    let mut titles_col_len = titles[0].len();
    let mut ids_col_len = ids[0].len();

    for task in tasks
    {
        let status = task.status.to_string();
        let id = format!("{:?}", task.id);

        if status.len() > statuses_col_len
        {
            statuses_col_len = status.len();
        }

        if task.title.len() > titles_col_len
        {
            titles_col_len = task.title.len();
        }

        if id.len() > ids_col_len
        {
            ids_col_len = id.len();
        }

        statuses.push(status);
        titles.push(task.title.clone());
        ids.push(id);
    }

    
    for i in 0..statuses.len()
    {
        
        let status = statuses[i].clone();
        let title = titles[i].clone();
        let id = ids[i].clone();

        let mut status_style = String::from("");
        let mut title_style = String::from("");
        let mut id_style = String::from("");

        if i == 0
        {
            status_style = format!("{}", style::Bold);
            title_style = format!("{}", style::Bold);
            id_style = format!("{}", style::Bold);
        }

        if i != 0
        {
            let task = &tasks[i - 1]; //- 1 here because the first element is the header of the list
            status_style = task.status.get_color_and_style();

            title_style = match task.status
            {
                TaskStatus::Done => format!("{}", style::CrossedOut),
                _ => title_style
            };

            id_style = format!("{}", color::Fg(color::AnsiValue::grayscale(14)));
        }
        
        
        print_w_spacing(id, ids_col_len + spacing, id_style);
        print_w_spacing(status, statuses_col_len + spacing, status_style);
        print_w_spacing(title, titles_col_len + spacing, title_style);

        print!("\n");
    }
}

fn print_w_spacing(text: String, min_len: usize, style: String)
{
    print!("{}{}{}{}", style, text, style::Reset, String::from_iter(vec![' '; min_len - text.len()].into_iter()));
}

fn create_task(tasks: &mut Vec<Task>, title: &String)
{
    let mut id = 0;

    if tasks.len() > 0
    {
        id = tasks[tasks.len() - 1].id + 1;
    }

    tasks.push(Task { id: id, title: title.clone(), status: TaskStatus::Pending });

    println!("Task created successfully!\n");
    print_tasks(tasks);
}

fn set_task_status(tasks: &mut Vec<Task>, task_id: &String, status: &String)
{
    let id = str::parse::<usize>(task_id).unwrap();

    for task in &mut *tasks
    {
        if task.id == id
        {
            task.status = match status.as_str()
            {
                "pending" => TaskStatus::Pending,
                "progress" => TaskStatus::InProgress,
                "done" => TaskStatus::Done,
                _ => panic!("Invalid value!"),
            };

            println!("Status of task {} set successfully.\n", task_id);
            print_tasks(tasks);
            return;
        }
    }
}