#![allow(dead_code)]

use crate::reaction::Reaction;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DBRecord {
    #[serde(rename = "ReactionName")]
    pub data_id: String,
    #[serde(rename = "ReferenceValue")]
    pub reference_value: f64,
    #[serde(rename = "Weight", default = "default_weight")]
    pub weight: f64,
    #[serde(rename = "Uncertainty", default = "default_uncertainty")]
    pub uncertainty: f64,
    #[serde(rename = "Unit", default = "default_unit")]
    pub unit: String,
    #[serde(rename = "Reaction")]
    reaction: String,
    #[serde(skip_deserializing)]
    reaction_internal: Reaction,
}

fn default_weight() -> f64 {
    1.0
}
fn default_uncertainty() -> f64 {
    0.0
}
fn default_unit() -> String {
    String::from("Hartree")
}

pub struct Database {
    pub name: String,
    pub description: String,
    pub doi: String,
    pub reference_method: String,
    pub data: Vec<DBRecord>,
}

impl Database {
    pub fn new(
        dbfile: &std::path::Path,
        name: &String,
        description: &String,
        doi: &String,
        reference_method: &String,
    ) -> Result<Self, csv::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .comment(Some(b'#'))
            .has_headers(true)
            .from_path(dbfile)?;
        let mut data: Vec<DBRecord> = vec![];
        for result in reader.deserialize() {
            let mut dbrecord: DBRecord = result?;
            dbrecord.reaction_internal = Reaction::new(&dbrecord.reaction)
                .expect(format!("DB {} has a bug!", dbfile.display()).as_str());
            data.push(dbrecord);
        }
        return Ok(Database {
            name: name.clone(),
            description: description.clone(),
            doi: doi.clone(),
            reference_method: reference_method.clone(),
            data: data,
        });
    }

    pub fn compute<CB: Fn(&String) -> f64>(self, compute: CB) -> Vec<(String, f64)> {
        let mut results: Vec<(String, f64)> = vec![];
        for dbrecord in self.data {
            let val: f64 = dbrecord
                .reaction_internal
                .compounds
                .iter()
                .map(|pair| pair.1 * compute(&pair.0))
                .sum();
            results.push((dbrecord.data_id, val));
        }
        return results;
    }

    pub fn update(&mut self, compute: fn(&String) -> f64, uncertainty: Option<f64>) {
        //todo!("Implement type conversion");
        self.data.iter_mut().for_each(|dbrecord| {
            dbrecord.reference_value = dbrecord
                .reaction_internal
                .compounds
                .iter()
                .map(|pair| pair.1 * compute(&pair.0))
                .sum();
            if uncertainty.is_some() {
                dbrecord.uncertainty = uncertainty.unwrap();
            }
        });
    }

    pub fn compute_diff<CB: Fn(&String) -> f64>(
        self,
        compute: CB,
        with_uncertainty: Option<bool>,
    ) -> Vec<(String, f64)> {
        let mut results: Vec<(String, f64)> = vec![];
        for dbrecord in self.data {
            let mut val: f64 = dbrecord
                .reaction_internal
                .compounds
                .iter()
                .map(|pair| pair.1 * compute(&pair.0))
                .sum();
            //todo!("Implement type conversion");
            if with_uncertainty.is_some() && with_uncertainty.unwrap() {
                let diff = val - dbrecord.reference_value;
                if f64::abs(diff) < dbrecord.uncertainty {
                    val = 0.0
                } else if diff > 0.0 {
                    val = diff - dbrecord.uncertainty;
                } else {
                    val = diff + dbrecord.uncertainty;
                }
            } else {
                val = val - dbrecord.reference_value;
            }
            results.push((dbrecord.data_id, val));
        }
        return results;
    }

    pub fn compute_stat<CB: Fn(&String) -> f64>(
        self,
        compute: CB,
        with_uncertainty: Option<bool>,
    ) -> Vec<(String, f64)> {
        let mut results: Vec<(String, f64)> = vec![];
        let data = self.compute_diff(compute, with_uncertainty);
        results.push((
            String::from("MD"),
            data.iter().map(|x| x.1).sum::<f64>() / data.len() as f64,
        ));
        results.push((
            String::from("MAE"),
            data.iter().map(|x| f64::abs(x.1)).sum::<f64>() / data.len() as f64,
        ));
        results.push((
            String::from("RMSE"),
            f64::sqrt(data.iter().map(|x| x.1 * x.1).sum::<f64>() / data.len() as f64),
        ));
        results.push((
            String::from("MAX"),
            data.iter()
                .map(|x| f64::abs(x.1))
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
        ));
        return results;
    }
}
