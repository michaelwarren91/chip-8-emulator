use std::fs;

#[derive(Default)]
pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn from_file_path(file_path: String) -> Result<Rom, String> {
        let data = fs::read(&file_path).map_err(|error| {
            String::from("Failed to read file '") + &file_path + "': " + &error.to_string()
        })?;

        let rom = Rom { data };
        Ok(rom)
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}
