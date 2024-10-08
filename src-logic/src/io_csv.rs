use crate::types::*;
use csv;
use std::fs::File;

#[derive(Debug)]
struct CSVReader {
    file_path: String,
    delimiter: u8,
    reader: csv::Reader<File>,
    headers: Vec<String>,
    data: Vec<Vec<f32>>,
}

impl CSVReader {
    fn new(file_path: String, delimiter: Option<u8>) -> Self {
        let delimiter = match delimiter {
            Some(d) => d,
            _ => b';',
        };

        // open file
        let file = File::open(file_path.clone()).unwrap();

        // handle file with rust-csv
        let mut reader = csv::ReaderBuilder::new().delimiter(delimiter).from_reader(file);
        let headers =  reader.headers().unwrap();

        // convert into custom format
        let headers: Vec<String> = headers.iter().map(|header| header.to_string()).collect();

        let mut data: Vec<Vec<f32>> = vec![Vec::new(); headers.len()];
        for j in reader.records() {
            let j = j.unwrap();
            j.iter().enumerate().for_each(|(header, value)| {
                let value = value.parse::<f32>().unwrap();
                data[header].push(value);
            });
        }
        
        CSVReader {
            file_path,
            delimiter,
            reader,
            headers,
            data,
        }
    }

    fn get_data(&self, header: String) -> Vec<f32> {
        if !self.headers.contains(&header) {
            panic!("header does not exist");
        }
        self.data[self.headers.iter().position(|s| s == &header).unwrap()].clone()
    }
}

pub trait FromCSV {
    fn from_csv(file_path: String) -> Self;
}

impl FromCSV for Dem1D {
    fn from_csv(file_path: String) -> Self {
        let csv = CSVReader::new(file_path, None);
        let mut dem = Dem1D::default();
        dem.x = csv.get_data(String::from("x"));
        dem.surface.z = csv.get_data(String::from("z"));
        dem
    }
}

impl FromCSV for Surface1D {
    fn from_csv(file_path: String) -> Self {
        let csv = CSVReader::new(file_path, None);
        let mut surface = Surface1D::default();
        let _x = csv.get_data(String::from("x"));
        surface.z = csv.get_data(String::from("z"));
        surface
    }
}

impl FromCSV for DispData {
    fn from_csv(file_path: String) -> Self {
        let csv = CSVReader::new(file_path, None);
        let mut disp_data = DispData::default();
        disp_data.x = csv.get_data(String::from("x"));
        disp_data.amplitude = csv.get_data(String::from("disp"));
        disp_data
    }
}

impl FromCSV for DispProfile {
    fn from_csv(file_path: String) -> Self {
        let csv = CSVReader::new(file_path, None);
        let mut disp_profile = DispProfile::default();
        disp_profile.origin_x = csv.get_data(String::from("x"));
        disp_profile.origin_z = csv.get_data(String::from("z"));
        disp_profile.amplitude_vec = csv.get_data(String::from("amplitude"));
        disp_profile.slope_vec = csv.get_data(String::from("slope"));
        disp_profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv() {
        let path = String::from("./test_data/dem.csv");
        let csv = CSVReader::new(path, None);
        println!("all csv infos: {:?}", csv);
        println!("x data: {:?}", csv.get_data(String::from("x")));
        println!("z data: {:?}", csv.get_data(String::from("z")));
    }

}