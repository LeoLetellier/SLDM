use crate::types::*;
use csv;
use std::fs::File;
use anyhow::anyhow;
use anyhow::Result;
use thiserror::Error;

#[derive(Debug)]
struct CSVReader {
    file_path: String,
    delimiter: u8,
    reader: csv::Reader<File>,
    headers: Vec<String>,
    data: Vec<Vec<f32>>,
}

#[derive(Debug, Error)]
enum CsvError{
    #[error("Invalid header: {0}")]
    InvalidHeader(String),
}

impl CSVReader {
    fn new(file_path: String, delimiter: Option<u8>) -> Result<Self>  {
        let delimiter = match delimiter {
            Some(d) => d,
            _ => b';',
        };

        // open file
        let file = File::open(file_path.clone())?;

        // handle file with rust-csv
        let mut reader = csv::ReaderBuilder::new().delimiter(delimiter).from_reader(file);
        let headers =  reader.headers()?;

        // convert into custom format
        let headers: Vec<String> = headers.iter().map(|header| header.to_string()).collect();

        let mut data: Vec<Vec<f32>> = vec![Vec::new(); headers.len()];
        for j in reader.records() {
            let j = j?;
            for k in 0..j.len() {
                let value = j[k].parse::<f32>()?;
                data[k].push(value);
            };
        }
        
        Ok(CSVReader {
            file_path,
            delimiter,
            reader,
            headers,
            data,
        })
    }

    fn get_data(&self, header: String) -> Result<Vec<f32>> {
        match !self.headers.contains(&header) {
            true => Err(anyhow!(CsvError::InvalidHeader(header))),
            false => Ok(self.data[self.headers.iter().position(|s| s == &header).unwrap()].clone()),
        }
    }
}

impl Dem1D {
    pub fn from_csv(file_path: String) -> Result<Self> {
        let csv = CSVReader::new(file_path, None)?;
        let mut dem = Dem1D::default();
        dem.x = csv.get_data(String::from("x"))?;
        dem.surface.z = csv.get_data(String::from("z"))?;
        Ok(dem)
    }
}

impl Surface1D {
    pub fn from_csv(file_path: String) -> Result<Self> {
        let csv = CSVReader::new(file_path, None)?;
        let mut surface = Surface1D::default();
        let _x = csv.get_data(String::from("x"))?;
        surface.z = csv.get_data(String::from("z"))?;
        Ok(surface)
    }
}

impl DispData {
    pub fn from_csv(file_path: String) -> Result<Self> {
        let csv = CSVReader::new(file_path, None)?;
        let mut disp_data = DispData::default();
        disp_data.x = csv.get_data(String::from("x"))?;
        disp_data.amplitude = csv.get_data(String::from("disp"))?;
        Ok(disp_data)
    }
}

impl DispProfile {
    pub fn from_csv(file_path: String) -> Result<Self> {
        let csv = CSVReader::new(file_path, None)?;
        let mut disp_profile = DispProfile::default();
        disp_profile.origin_x = csv.get_data(String::from("x"))?;
        disp_profile.origin_z = csv.get_data(String::from("z"))?;
        disp_profile.amplitude_vec = csv.get_data(String::from("amplitude"))?;
        disp_profile.slope_vec = csv.get_data(String::from("slope"))?;
        Ok(disp_profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv() {
        let path = String::from("./test_data/dem.csv");
        let csv = CSVReader::new(path, None).unwrap();
        println!("all csv infos: {:?}", csv);
        println!("x data: {:?}", csv.get_data(String::from("x")).unwrap());
        println!("z data: {:?}", csv.get_data(String::from("z")).unwrap());
    }

}