use super::Project;
use serde::{Deserialize, Serialize};
use src_logic::prelude::*;
use anyhow::Result;
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct ProjectFile {
    project: ProjectRelated,
    surface: Vec<SurfaceRelated>,
    unit_model: Vec<UnitModelRelated>,
    combined_model: Vec<CombinedModelRelated>,
    disp_data: Vec<DispDataRelated>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectRelated {
    name: String,
    note: String,
    dem_file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SurfaceRelated {
    file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnitModelRelated {
    file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CombinedModelRelated {
    file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DispDataRelated {
    file_name: String,
}

impl Project {
    fn save(&self) -> Result<()> {
        todo!()
    }

    fn load(&mut self) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml() {
        let proj = ProjectFile {
            project: ProjectRelated { name: String::from("My dear little project"), 
                note: String::from("Usefull data I want to remember about my project"), 
                dem_file_name: String::from("dem.csv") 
            },
            surface: vec![SurfaceRelated{file_name: String::from("surf1.csv")},
            SurfaceRelated{file_name: String::from("surf2.csv")}],
            unit_model: vec![UnitModelRelated{file_name: String::from("unit1.csv")},
            UnitModelRelated{file_name: String::from("unit2.csv")}],
            combined_model: vec![CombinedModelRelated{file_name: String::from("model1.csv")},
            CombinedModelRelated{file_name: String::from("model2.csv")},
            CombinedModelRelated{file_name: String::from("model3.csv")}],
            disp_data: vec![DispDataRelated{file_name: String::from("disp.csv")}],
        };
        let toml = toml::to_string(&proj).unwrap();
        println!("toml:\n{}", toml);
    }
}