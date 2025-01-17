use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MethodRecord {
    #[serde(rename = "Structure")]
    pub geometry: String,
    #[serde(rename = "Energy")]
    pub energy: f64,
    #[serde(rename = "Unit", default = "default_unit")]
    pub unit: String,
}

fn default_unit() -> String {
    String::from("Hartree")
}

pub struct Method {
    pub filepath: std::path::PathBuf,
    pub data: Vec<MethodRecord>,
}

impl Method {
    pub fn new(dbfile: &std::path::PathBuf) -> Result<Method, csv::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .comment(Some(b'#'))
            .has_headers(true)
            .from_path(&dbfile)?;

        let mut data: Vec<MethodRecord> = vec![];
        for result in reader.deserialize() {
            let dbrecord: MethodRecord = result?;
            data.push(dbrecord);
        }
        Ok(Method {
            filepath: dbfile.clone(),
            data: data,
        })
    }

    pub fn get_energy(&self, geometry: &String) -> f64 {
        let data = self
            .data
            .iter()
            .filter(|x| x.geometry.as_str() == geometry.as_str())
            .map(|x| (x.energy, &x.unit))
            .collect::<Vec<(f64, &String)>>();
        if data.len() != 1 {
            panic!("Unknown geometry '{}'", geometry);
        }
        data[0].0 * 627.5095
    }
}
