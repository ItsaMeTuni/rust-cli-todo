use serde::{Serialize, Deserialize};
use std::iter::Iterator;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task
{
    pub id: usize,
    pub title: String,
    pub tags: Vec<String>,
    pub deadline: u64,
    pub deadline_includes_time: bool,
    pub repeats: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag
{
    pub name: String,
    pub color: String,
    pub is_status: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData
{
    pub tasks: Vec<Task>,
    pub tags: Vec<Tag>,
}

impl Task
{
    /*pub fn get_status(&self, user_data: &UserData) -> &Tag
    {
        for tag in self.get_tags(user_data)
        {
            if tag.is_status
            {
                return tag;
            }
        }

        panic!("The task has no status tag! {:?}", self);
    }*/

    pub fn get_tags(&self, user_data: &UserData) -> Vec::<&Tag>
    {
        let mut tags = Vec::<&Tag>::new();

        for tag in &user_data.tags
        {
            match self.tags.iter().find(|tag_name: &String| tag.name == *tag_name)
            {
                Some(e) => tags.push(tag),
                None => { }
            }
        }

        return tags;
    }

    pub fn get_deadline_str() -> String
    {
        
    }
}
