mod io;
mod types;
mod output;
mod utils;

#[macro_use]
extern crate clap;
extern crate colorsys;

use termion::{color, style};
use types::*;
use dirs;
use utils::*;

fn main()
{
    let mut path = dirs::home_dir().unwrap().into_os_string().to_str().unwrap().to_owned();
    path.push_str("/.cli-todo");

    let mut user_data = io::load_user_data_from_disk(&path);

    user_data.create_tag(Tag {
        name: "pending".to_owned(),
        style_and_color: format!("{}{}", color::Fg(color::LightYellow), style::Bold),
        is_status: true,
    });

    user_data.create_tag(Tag {
        name: "done".to_owned(),
        style_and_color: format!("{}{}", color::Fg(color::LightBlue), style::Bold),
        is_status: true,
    });

    let matches: clap::ArgMatches = clap_app!(cliTodo =>
        (@subcommand create =>
            (@arg title: * "Title of the task to create")
            (@arg tags: ...)
        )
        (@subcommand delete =>
            (@arg id: *)
        )
        (@subcommand tag =>
            (@arg name: *)
            (@arg color: )
            (@arg delete: --delete)
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

        user_data.add_task(new_task);
    }

    if let Some(delete_subcmd) = matches.subcommand_matches("delete")
    {
        let id = delete_subcmd.value_of("id").unwrap().parse::<usize>();
        if id.is_err()
        {
            panic!("Id arg not a number!");
        }

        let id = id.unwrap();

        user_data.delete_task(id);
    }

    if let Some(tag_subcmd) = matches.subcommand_matches("tag")
    {
        let name = tag_subcmd.value_of("name").unwrap();

        //create/update tag if setting color
        if let Some(color) = tag_subcmd.value_of("color")
        {
            let color = colorsys::Rgb::from_hex_str(color).unwrap();
    
            let color = color::Fg(color::AnsiValue::rgb(
                scale_range(0.0, 256.0, 0.0, 5.0,   color.get_red()     ) as u8,
                scale_range(0.0, 256.0, 0.0, 5.0,   color.get_green()   ) as u8,
                scale_range(0.0, 256.0, 0.0, 5.0,   color.get_blue()    ) as u8
            ));
    
            //create_tag overwrites an existing tag with the same name if there is one
            user_data.create_tag(Tag {
                name: name.to_owned(),
                style_and_color: format!("{}", color),
                is_status: false,
            });
        }

        if tag_subcmd.is_present("delete")
        {
            user_data.delete_tag(name);
        }
    }

    output::print_tasks(&user_data);

    io::save_user_data_to_disk(&user_data, &path);
}