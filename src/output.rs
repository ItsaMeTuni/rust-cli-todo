use crate::types::*;

pub fn print_tasks(user_data: &UserData)
{

    let mut id_col: Col = Col::new(String::from("ID"));
    let mut status_col: Col = Col::new(String::from("Status"));
    let mut title_col: Col = Col::new(String::from("Title"));
    let mut deadline_col: Col = Col::new(String::from("Deadline"));
    let mut tags_col: Col = Col::new(String::from("Tags"));
    let mut repeat_col: Col = Col::new(String::from("Repeat"));

    for task in &user_data.tasks
    {
        id_col.add_row(task.title.clone(), task.title.len());

        for tag in task.get_tags(user_data)
        {
            if tag.is_status
            {
                status_col.add_row(tag.name, tag.name.len());
            }
            else
            {
                tags_col.add_row(tag.name, tag.name.len());
            }
        }

        let deadline = task.get_deadline();
        deadline_col.add_row(deadline, deadline.len())
    }

}

pub struct Col
{
    rows: Vec<String>,
    min_len: usize,
    pub title: String,
}

impl Col
{
    fn add_row(&self, text: String, display_len: usize)
    {
        self.rows.push(text);

        if display_len > self.min_len
        {
            self.min_len = display_len;
        }
    }

    fn new(title: String) -> Col
    {
        return Col {
            rows: Vec::<String>::new(),
            min_len: title.len(),
            title: title,
        }
    }
}