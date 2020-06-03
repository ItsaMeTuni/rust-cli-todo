use crate::types::*;
use std::iter::FromIterator;
use termion::{color, style};

pub fn print_tasks(user_data: &UserData)
{
    let reset_styles_and_colors = format!("{}{}{}", color::Fg(color::Reset), color::Bg(color::Reset), style::Reset);

    let mut table = Table::new();

    let id_col          = table.new_col("ID"          .to_owned());
    let status_col      = table.new_col("Status"      .to_owned());
    let title_col       = table.new_col("Title"       .to_owned());
    let deadline_col    = table.new_col("Deadline"    .to_owned());
    let tags_col        = table.new_col("Tags"        .to_owned());
    let repeat_col      = table.new_col("Repeat"      .to_owned());


    for task in &user_data.tasks
    {
        let mut is_done = false;

        //Id
        let id_str = task.id.to_string();
        table.cols[id_col].add_row(
            format!("{}{}{}", color::Fg(color::AnsiValue::grayscale(14)), id_str, reset_styles_and_colors),
            id_str.len()
        );

        //Tags
        let mut status_str = String::new();
        let mut status_str_display_len = 0;
        let mut tags_str = String::new();
        let mut tags_str_display_len = 0;
        
        for tag in task.get_tags(user_data)
        {
            if tag.is_status
            {
                status_str = format!("{}{}{}", tag.style_and_color, tag.name, reset_styles_and_colors);
                status_str_display_len = tag.name.len();

                if tag.name == "done"
                {
                    is_done = true;
                }
            }
            else
            {
                if tags_str.len() > 0
                {
                    tags_str.push_str(", ");
                    tags_str_display_len += 2;
                }

                tags_str.push_str(&tag.style_and_color);
                tags_str.push_str(&tag.name);
                tags_str.push_str(&reset_styles_and_colors);

                tags_str_display_len += tag.name.len();
            }
        }

        table.cols[tags_col].add_row(tags_str, tags_str_display_len);

        //Status
        table.cols[status_col].add_row(status_str, status_str_display_len);

        //Title
        let mut title_style = "".to_owned();
        if is_done
        {
            title_style = format!("{}", style::CrossedOut);
        }
        
        table.cols[title_col].add_row(
            format!("{}{}{}", title_style, task.title, reset_styles_and_colors),
            task.title.len()
        );


        //Deadline
        let deadline = task.get_deadline_str();

        let mut deadline_style = "".to_owned();
        if is_done
        {
            deadline_style = format!("{}", style::CrossedOut);
        }

        table.cols[deadline_col].add_row(
            format!("{}{}{}", deadline_style, deadline.clone(), reset_styles_and_colors),
            deadline.len()
        );
        

        //Repeats
        let repeats_str = if task.repeats { "yes" } else { "no" };
        table.cols[repeat_col].add_row(repeats_str.to_owned(), repeats_str.len());
    }
    

    let spacing = String::from_iter(vec![' '; 2].into_iter());


    for col in &table.cols
    {
        let space_fill = get_filler_str(col.min_len - col.title.len());
        print!("{}{}{}{}{}", style::Bold, col.title, space_fill, spacing, style::Reset);
    }

    print!("\n");

    for row_i in 0..user_data.tasks.len()
    {
        for col in &table.cols
        {
            //This is a string of spaces that gets appended to the row's content
            //so that the table is aligned (all cells have the same width)
            let space_fill = get_filler_str(col.min_len - col.rows_display_len[row_i]);

            print!("{}{}{}", col.rows[row_i], space_fill, spacing);
        }

        print!("\n");
    }
}

fn get_filler_str(len: usize) -> String
{
    return String::from_iter(vec![' '; len].into_iter())
}

#[derive(Debug)]
pub struct Table
{
    cols: Vec<Box<Col>>
}

impl Table
{
    pub fn new() -> Table
    {
        return Table { cols: Vec::<Box<Col>>::new() };
    }

    pub fn new_col(&mut self, title: String) -> usize
    {
        let col_box = Box::new(Col::new(title));
        self.cols.push(col_box);

        return self.cols.len() - 1;
    }
}

#[derive(Debug)]
pub struct Col
{
    rows: Vec<String>,
    rows_display_len: Vec<usize>,
    min_len: usize,
    pub title: String,
}

impl Col
{
    fn add_row(&mut self, text: String, display_len: usize)
    {
        self.rows.push(text);
        self.rows_display_len.push(display_len);

        if display_len > self.min_len
        {
            self.min_len = display_len;
        }
    }

    fn new(title: String) -> Col
    {
        return Col {
            rows: vec![],
            rows_display_len: vec![],
            min_len: title.len(),
            title: title,
        }
    }
}