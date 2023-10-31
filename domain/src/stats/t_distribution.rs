use std::env;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct TDistribution {
    df: f64,
    p20: f64,
    p10: f64,
    p05: f64,
    p02: f64,
    p01: f64,
    p005: f64,
    p002: f64,
    p001: f64,
}

impl TDistribution {
    pub fn t_for_p(&self, p: f64) -> f64 {
        if p >= 0.20 {
            return self.p20;
        } else if p >= 0.10 {
            return self.p10;
        } else if p >= 0.05 {
            return self.p05;
        } else if p >= 0.02 {
            return self.p02;
        } else if p >= 0.01 {
            return self.p01;
        } else if p >= 0.005 {
            return self.p005;
        } else if p >= 0.002 {
            return self.p002;
        } else {
            return self.p001;
        }
    }

    pub fn p_for_t(&self, t: f64) -> f64 {
        if t > self.p001 {
            return 0.001;
        } else if t > self.p002 {
            return 0.002;
        } else if t > self.p005 {
            return 0.005;
        } else if t > self.p01 {
            return 0.01;
        } else if t > self.p02 {
            return 0.02;
        } else if t > self.p05 {
            return 0.05;
        } else if t > self.p10 {
            return 0.10;
        } else if t > self.p20 {
            return 0.20;
        } else {
            return 1.0;
        }
    }
}

pub struct TTable {
    lut: Vec<TDistribution>,
}

impl TTable {
    pub fn new() -> TTable {
        let path = match env::var("CSV_PATH") {
            Ok(var) => var,
            Err(_) => {
                panic!("CSV_PATH is not configured. Configure it before launching the program.")
            }
        };
        let mut reader =
            csv::Reader::from_path(path).expect("Could not find degrees of freedom lookup table");
        let mut collector = Vec::new();
        for result in reader.deserialize() {
            let record: TDistribution = result.unwrap();
            collector.push(record);
        }
        TTable { lut: collector }
    }

    pub fn get_t_for(&self, df: f64, p: f64) -> f64 {
        let df = TTable::nearest_df(df);
        for t_dist in self.lut.clone() {
            if t_dist.df == df {
                return t_dist.t_for_p(p);
            }
        }
        return 1000.0;
    }

    pub fn get_p_for(&self, df: f64, t: f64) -> f64 {
        let df = TTable::nearest_df(df);
        for t_dist in self.lut.clone() {
            if t_dist.df == df {
                return t_dist.p_for_t(t);
            }
        }
        return 1.0;
    }

    fn nearest_df(df: f64) -> f64 {
        if df <= 40.0 {
            return df;
        } else if df <= 50.0 {
            if df % 2.0 == 0.0 {
                return df;
            } else {
                return df - 1.0;
            }
        } else if df <= 100.0 {
            return df - (df % 10.0);
        } else if df < 120.0 {
            return 100.0;
        } else if df < 150.0 {
            return 120.0;
        } else if df < 200.0 {
            return 150.0;
        } else if df < 300.0 {
            return 200.0;
        } else if df < 500.0 {
            return 300.0;
        } else if df < 750.0 {
            return 500.0;
        } else {
            return 1000.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_appropriate_max_p_for_t() {
        let ttable = TTable::new();

        assert_eq!(ttable.get_p_for(9.0, 5.0), 0.001);
        assert_eq!(ttable.get_p_for(9.0, 4.3), 0.002);
        assert_eq!(ttable.get_p_for(9.0, 3.8), 0.005);
        assert_eq!(ttable.get_p_for(9.0, 3.3), 0.01);
        assert_eq!(ttable.get_p_for(9.0, 2.9), 0.02);
        assert_eq!(ttable.get_p_for(9.0, 2.3), 0.05);
        assert_eq!(ttable.get_p_for(9.0, 1.9), 0.10);
        assert_eq!(ttable.get_p_for(9.0, 1.4), 0.20);
        assert_eq!(ttable.get_p_for(9.0, 1.1), 1.0);
        assert_eq!(ttable.get_p_for(-9.0, 1.4), 1.0);
    }

    #[test]
    fn nearest_df_is_rounded_down_for_given_df_not_on_table() {
        assert_eq!(TTable::nearest_df(41.0), 40.0);
        assert_eq!(TTable::nearest_df(42.0), 42.0);
        assert_eq!(TTable::nearest_df(110.0), 100.0);
        assert_eq!(TTable::nearest_df(130.0), 120.0);
        assert_eq!(TTable::nearest_df(170.0), 150.0);
        assert_eq!(TTable::nearest_df(270.0), 200.0);
        assert_eq!(TTable::nearest_df(670.0), 500.0);
        assert_eq!(TTable::nearest_df(1001.0), 1000.0);
        assert_eq!(TTable::nearest_df(52.0), 50.0);
        assert_eq!(TTable::nearest_df(57.0), 50.0);
    }

    #[test]
    fn ttable_new_reads_csv_and_returns_ttable_struct() {
        let ttable = TTable::new();

        assert_eq!(ttable.lut.len(), 56);
    }

    #[test]
    fn ttable_returns_expected_t_value_for_given_query() {
        let ttable = TTable::new();

        let t = ttable.get_t_for(17.0, 0.05);
        let t2 = ttable.get_t_for(300.0, 0.001);
        let t3 = ttable.get_t_for(-1.0, 0.05);

        assert_eq!(t, 2.11);
        assert_eq!(t2, 3.323);
        assert_eq!(t3, 1000.0);
        assert_eq!(ttable.get_t_for(14.0, 0.0011), 4.14);
        assert_eq!(ttable.get_t_for(14.0, 0.0021), 3.787);
        assert_eq!(ttable.get_t_for(14.0, 0.0051), 3.326);
        assert_eq!(ttable.get_t_for(14.0, 0.011), 2.977);
        assert_eq!(ttable.get_t_for(14.0, 0.022), 2.625);
        assert_eq!(ttable.get_t_for(14.0, 0.11), 1.761);
        assert_eq!(ttable.get_t_for(14.0, 0.21), 1.345);
    }
}
