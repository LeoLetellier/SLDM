use crate::data::vec_proj::Vector2Rep;
use crate::types::*;
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use csv;
use std::fs::File;
use std::vec;
use thiserror::Error;

#[derive(Debug)]
pub struct CsvReader {
    pub headers: Vec<String>,
    data: Vec<Vec<f32>>,
}

#[derive(Debug, Error)]
enum CsvReadError {
    #[error("Invalid header: {0}")]
    InvalidHeader(String),
}

impl CsvReader {
    pub fn read(file_path: String, delimiter: Option<u8>) -> Result<Self> {
        log::trace!("Read file at {file_path}");
        let delimiter = match delimiter {
            Some(d) => d,
            _ => b';',
        };

        // open file
        let file = File::open(file_path.clone())?;

        // handle file with csv
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .from_reader(file);
        let headers = reader.headers()?;

        // convert into custom format
        let headers: Vec<String> = headers.iter().map(|header| header.to_string()).collect();

        let mut data: Vec<Vec<f32>> = vec![Vec::new(); headers.len()];
        for j in reader.records() {
            let j = j?;
            for k in 0..j.len() {
                let value = j[k].parse::<f32>()?;
                data[k].push(value);
            }
        }

        Ok(CsvReader { headers, data })
    }

    pub fn get_data(&self, header: &String) -> Result<Vec<f32>> {
        if self.headers.contains(header) {
            Ok(self.data[self.headers.iter().position(|s| s == header).unwrap()].clone())
        } else {
            Err(anyhow!(CsvReadError::InvalidHeader(header.clone())))
        }
    }

    pub fn get_datas(&self, headers: &Vec<String>) -> Result<Vec<Vec<f32>>> {
        let mut results = vec![];
        for header in headers {
            results.push(self.get_data(header)?);
        }
        Ok(results)
    }
}

#[derive(Debug)]
pub struct CsvWriter {
    pub headers: Vec<String>,
    datas: Vec<Vec<f32>>,
}

impl CsvWriter {
    pub fn from_datas_headers(datas: Vec<Vec<f32>>, headers: Vec<String>) -> Result<Self> {
        if datas.len() != headers.len() {
            bail!("Datas and headers dos not match.");
        }
        let csv_writer = CsvWriter { headers, datas };
        Ok(csv_writer)
    }

    pub fn write(self, file_path: &String, delimiter: Option<u8>) -> Result<()> {
        log::trace!("Write file at {file_path}");
        let delimiter = match delimiter {
            Some(d) => d,
            _ => b';',
        };

        let mut writer = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_path(file_path)?;

        // Write headers
        writer.write_record(self.headers)?;

        // Write data as strings
        for k in 0..self.datas[0].len() {
            let mut line = vec![];
            for d in 0..self.datas.len() {
                line.push(self.datas[d][k].to_string());
            }
            writer.write_record(line)?;
        }
        writer.flush()?;
        Ok(())
    }
}

#[derive(Debug, Error)]
enum FromCsvError {
    #[error("The number of points in the sampling is different from the project's dem")]
    XInvalidLen,
    #[error("The values of sampling does not match with those of the project's dem")]
    XInconsistent,
    #[error("Two data have inconsistent vectors's length")]
    DataInconsistentLen,
}

impl Dem1D {
    pub fn from_csv_reader(
        csv_reader: &CsvReader,
        x_header: &mut String,
        z_header: &mut String,
    ) -> Result<Self> {
        let x_header = if x_header.is_empty() {
            &String::from("x")
        } else {
            x_header
        };
        let z_header = if z_header.is_empty() {
            &String::from("z")
        } else {
            z_header
        };

        let x_data = csv_reader.get_data(x_header)?;
        let z_data = csv_reader.get_data(z_header)?;
        Ok(Dem1D::new(x_data, z_data)?)
    }
}

impl Surface1D {
    pub fn from_csv_reader(
        csv_reader: &CsvReader,
        dem: &Dem1D,
        x_header: &mut String,
        surface_header: &mut String,
    ) -> Result<Self> {
        let x_header = if x_header.is_empty() {
            &String::from("x")
        } else {
            x_header
        };
        let surface_header = if surface_header.is_empty() {
            &String::from("z")
        } else {
            surface_header
        };

        let x_data = csv_reader.get_data(x_header)?;
        let surface_data = csv_reader.get_data(surface_header)?;

        if dem.x.len() != x_data.len() {
            return Err(anyhow!(FromCsvError::XInvalidLen));
        }

        for k in 0..x_data.len() {
            if dem.x[k] != x_data[k] {
                return Err(anyhow!(FromCsvError::XInconsistent));
            }
        }

        Ok(Surface1D::new(surface_data))
    }
}

impl DispProfile {
    pub fn from_csv_reader(
        csv_reader: &CsvReader,
        x_header: &mut String,
        z_header: &mut String,
        vx_header: &mut String,
        vz_header: &mut String,
    ) -> Result<Self> {
        let x_header = if x_header.is_empty() {
            &String::from("x")
        } else {
            x_header
        };
        let z_header = if z_header.is_empty() {
            &String::from("z")
        } else {
            z_header
        };
        let vx_header = if vx_header.is_empty() {
            &String::from("vx")
        } else {
            vx_header
        };
        let vz_header = if vz_header.is_empty() {
            &String::from("vz")
        } else {
            vz_header
        };

        let x_data = csv_reader.get_data(x_header)?;
        let z_data = csv_reader.get_data(z_header)?;
        let vx_data = csv_reader.get_data(vx_header)?;
        let vz_data = csv_reader.get_data(vz_header)?;

        if (x_data.len() != z_data.len())
            | (vx_data.len() != vz_data.len())
            | (x_data.len() != vx_data.len())
        {
            Err(anyhow!(FromCsvError::DataInconsistentLen))
        } else {
            let mut vecs: Vec<Vector2Rep> = vec![];
            let mut origins: Vec<[f32; 2]> = vec![];

            for k in 0..x_data.len() {
                vecs.push(Vector2Rep::new(vx_data[k], vz_data[k]));
                origins.push([x_data[k], z_data[k]]);
            }

            Ok(DispProfile::new(vecs, origins)?)
        }
    }
}

impl DispData {
    pub fn from_csv_reader(
        csv_reader: &CsvReader,
        x_header: &mut String,
        amp_header: &mut String,
    ) -> Result<Self> {
        let x_header = if x_header.is_empty() {
            &String::from("x")
        } else {
            x_header
        };
        let amp_header = if amp_header.is_empty() {
            &String::from("disp")
        } else {
            amp_header
        };

        let x_data = csv_reader.get_data(x_header)?;
        let amp_data = csv_reader.get_data(amp_header)?;

        if x_data.len() != amp_data.len() {
            Err(anyhow!(FromCsvError::DataInconsistentLen))
        } else {
            Ok(DispData::new(x_data, amp_data)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv() {
        let path = String::from("./test_data/dem.csv");
        let csv = CsvReader::read(path, None).unwrap();
        println!("all csv infos: {:?}", csv);
        println!("x data: {:?}", csv.get_data(&String::from("x")).unwrap());
        println!("z data: {:?}", csv.get_data(&String::from("z")).unwrap());
    }

    #[test]
    fn test_writing() {
        let x: Vec<f32> = vec![0., 1., 2., 3., 4., 5.];
        let z: Vec<f32> = vec![12., 56.56, 48., 0., 1.2, 7.];
        let writer =
            CsvWriter::from_datas_headers(vec![x, z], vec!["x".to_string(), "z".to_string()])
                .unwrap();
        writer
            .write(&"./test_data/project_files/test_csv.csv".to_string(), None)
            .unwrap();
    }

    #[test]
    fn test_writing_reading() {
        test_writing();
        let z: Vec<f32> = vec![12., 56.56, 48., 0., 1.2, 7.];
        let reader =
            CsvReader::read("./test_data/project_files/test_csv.csv".to_string(), None).unwrap();
        let get_z = reader.get_data(&"z".to_string()).unwrap();
        for k in 0..z.len() {
            assert_eq!(z[k], get_z[k]);
        }
    }
}
