use std::collections::{
    HashMap, 
    HashSet
};

use super::BoW;

#[derive(Debug)]
pub struct DF {
    pub frequencies: HashMap<String, usize>,
    pub documents: usize,
}

impl DF {
    pub fn build(bags: &Vec<BoW>) -> Self {
        let mut df: HashMap<String, usize> = HashMap::new();
        for bag in bags {
            let mut seen: HashSet<String> = HashSet::new();
            for token in bag.tokens() {
                if seen.insert(token.clone()) {
                    *df.entry(token.clone())
                        .or_insert(0) += 1;
                }
            }
        }

        return Self {
            frequencies: df,
            documents: bags.len(),
        };
    }

    pub fn idf(&self) -> HashMap<String, f64> {
        let mut idf_map: HashMap<String, f64> = HashMap::new();
        let documents: f64 = self.documents as f64;

        for (token, &df_count) in &self.frequencies {
            let idf: f64 = (documents / (1.0 + df_count as f64)).ln();
            idf_map.insert(token.clone(), idf);
        }

        return idf_map;
    }
}

