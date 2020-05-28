mod io;
mod types;
mod output;

use termion::{color, style};
use types::*;

#[macro_use]
extern crate clap;

fn main()
{
    let mut user_data = types::UserData::new();

    user_data.tags.push(Tag {
        name: "pending".to_owned(),
        style_and_color: format!("{}{}", color::Fg(color::LightYellow), style::Bold),
        is_status: true,
    });

    user_data.tags.push(Tag {
        name: "done".to_owned(),
        style_and_color: format!("{}{}", color::Fg(color::LightGreen), style::Bold),
        is_status: true,
    });

    let matches: clap::ArgMatches = clap_app!(cliTodo =>
        (@subcommand create =>
            (@arg title: * "Title of the task to create")
            (@arg tags: ...)
        )
    ).get_matches();

    if let Some(create_subcmd) = matches.subcommand_matches("create")
    {
        let mut new_task = Task::new();

        new_task.title = create_subcmd.value_of("title").unwrap().to_owned();
        if let Some(tags) = create_subcmd.values_of("tags")
        {
            for tag in tags
            {
                new_task.tags.push(tag.to_owned());
            }
        }

        user_data.addTask(new_task);
    }

    output::print_tasks(&user_data);
}