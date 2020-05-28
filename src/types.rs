use serde::{Serialize, Deserialize};
use std::iter::Iterator;
use std::fmt::Debug;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task
{
    pub id: usize,
    pub title: String,
    pub tags: Vec<String>,
    pub deadline: i64,
    pub deadline_includes_time: bool,
    pub repeats: bool,
}

impl Task
{
    pub fn new() -> Task
    {
        return Task {
            id: 0,
            title: "".to_owned(),
            tags: vec![],
            deadline: 0,
            deadline_includes_time: false,
            repeats: false,
        }
    }

    pub fn get_tags(&self, user_data: &UserData) -> Vec::<Tag>
    {
        let mut tags = Vec::<Tag>::new();

        for tag_name in &self.tags
        {
            match user_data.tags.iter().find(| x | *tag_name == (*x).name)
            {
                Some(tag) => tags.push(tag.clone()),
                None => tags.push(Tag::from_str(&tag_name))
            }
        }

        return tags;
    }

    pub fn get_deadline_str(&self) -> String
    {
        let naive_date_time = NaiveDateTime::from_timestamp(self.deadline / 1000, 0);

        let now = Local::now();
        let timezone = now.timezone();
        let date_time = timezone.from_utc_datetime(&naive_date_time);

        let date_str = date_time.format("%b %e");

        let day = date_time.day();

        let mut day_suffix = "";

        if day == 1 || day == 21 || day == 31
        {
            day_suffix = "st";
        }
        else if day == 2 || day == 22
        {
            day_suffix = "nd";
        }
        else if day == 3 || day == 23
        {
            day_suffix = "rd";
        }
        else
        {
            day_suffix = "th";
        }

        let time_str = date_time.format("%R");

        return format!("{}{} {}", date_str, day_suffix, time_str);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag
{
    pub name: String,
    pub style_and_color: String,
    pub is_status: bool,
}

impl Tag
{
    pub fn from_str(string: &String) -> Tag
    {
        return Tag {
            name: string.clone(),
            style_and_color: "".to_owned(),
            is_status: false,
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData
{
    pub tasks: Vec<Task>,
    pub tags: Vec<Tag>,
}

impl UserData
{
    pub fn new() -> UserData
    {
        return UserData {
            tasks: Vec::<Task>::new(),
            tags: Vec::<Tag>::new(),
        }
    }

    pub fn addTask(&mut self, mut task: Task)
    {
        task.id = self.tasks.len();
        self.tasks.push(task);
    }
}