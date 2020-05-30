use crate::types::UserData;

use std::io::prelude::*;

pub fn load_user_data_from_disk(path: &str) -> UserData
{
    let file_open_result = std::fs::OpenOptions::new()
                            .read(true)
                            .open(path);

    if file_open_result.is_ok()
    {
        let mut string = String::new();
        file_open_result.unwrap().read_to_string(&mut string).unwrap();

        return serde_json::from_str(&string).unwrap();
    }
    else
    {
        panic!("Error while opening user data file! {:?}", file_open_result.unwrap_err());
    }
}

pub fn save_user_data_to_disk(user_data: &UserData, path: &str)
{
    let file_open_result = std::fs::OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(path);

    if file_open_result.is_ok()
    {
        let mut file = file_open_result.unwrap();

        let data = serde_json::to_string(user_data).unwrap();

        file.write_all(data.as_bytes()).unwrap();
    }
    else
    {
        panic!("Error while opening user data file! {:?}", file_open_result.unwrap_err());
    }


}