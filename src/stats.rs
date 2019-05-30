pub fn summarize(v: &Vec<f64>) -> Vec<Vec<f64>> {
    let mut v_copy: Vec<f64> = v.to_vec();
    v_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut val: f64;
    let mut freq = 1;
    let mut table: Vec<Vec<f64>> = Vec::new();
    let mut i = 0;

    while i < v_copy.len() {
        val = v_copy[i + freq - 1];
        let mut a = i + 1;
        while a < v_copy.len() {
            if val == v_copy[a] {
                freq += 1;
            } else {
                break;
            }
            a += 1;
        }
        let val_freq = vec![val, freq as f64];
        table.push(val_freq);

        i += freq;
        freq = 1;
    }
    table
}

pub fn count(v: &Vec<f64>) -> i32 {
    v.len() as i32
}

pub fn sum(v: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in v {
        sum += *i;
    }
    sum
}

// REQUIRES: v is not empty
pub fn mean(v: &Vec<f64>) -> f64 {
    sum(v) / (v.len() as f64)
}

pub fn median(v: &Vec<f64>) -> f64 {
    // NOTE: making copy to get around creating mut ref arg.
    //       sacrificing performance slightly
    let mut v_copy: Vec<f64> = v.to_vec();
    v_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = v_copy.len();
    // if even
    if v_copy.len() % 2 == 1 {
        return v_copy[len / 2];
    } else {
        return (v_copy[len / 2] + v_copy[(len / 2) - 1]) / 2.0;
    }
}

pub fn mode(v: &Vec<f64>) -> f64 {
    let mut v_copy: Vec<f64> = v.to_vec();
    v_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut freq = 1;
    let mut most_freq = 0;
    let mut mode: f64 = 0.0;
    let mut val: f64;
    let mut i = 0;
    // [1, [2, 3, 3] ]
    while i < v_copy.len() {
        val = v[i + freq - 1];
        let mut a = i + 1;
        while a < v_copy.len() {
            if val == v[a] {
                freq += 1;
            } else {
                break;
            }
            a += 1;
        }
        if freq > most_freq {
            mode = val;
            most_freq = freq;
        }
        i += freq;
        freq = 1;
    }
    mode
}

pub fn min(v: &Vec<f64>) -> f64 {
    let mut v_copy: Vec<f64> = v.to_vec();
    v_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v_copy[0]
}

pub fn max(v: &Vec<f64>) -> f64 {
    let mut v_copy: Vec<f64> = v.to_vec();
    v_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v_copy[v_copy.len() - 1]
}

pub fn stdev(v: &Vec<f64>) -> f64 {
    let mut v_copy: Vec<f64> = v.to_vec();
    v_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut diff_sq: f64 = 0.0;
    let mut i = 0;
    let mean = mean(&v_copy);
    let n = v_copy.len();
    while i < n {
        let num = v_copy[i] - mean;
        diff_sq += num.powf(2.0);
        i += 1;
    }

    let stdev: f64 = ((1.0 / (n as f64 - 1.0)) * diff_sq).sqrt();
    stdev
}

pub fn percentile(v: &Vec<f64>, p: f64) -> f64 {
    let mut v_copy: Vec<f64> = v.to_vec();
    v_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let rank: f64;
    let intpart: usize;
    let fractpart: f64;
    let percentile: f64;

    if v_copy.len() == 1 {
        return v_copy[0];
    } else if p < 1.0 {
        rank = p * (v_copy.len() as f64 - 1.0) + 1.0;
        intpart = rank.floor() as usize;
        fractpart = rank - intpart as f64;
        percentile = v_copy[intpart - 1] + (fractpart * (v_copy[intpart] - v_copy[intpart - 1]));
        return percentile;
    } else {
        percentile = match v_copy.last().clone() {
            None => -1.0,
            Some(i) => i.clone(),
        };
        return percentile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_basic() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(3, count(&v));
    }

    #[test]
    fn test_count_empty() {
        let v: Vec<f64> = Vec::new();
        assert_eq!(0, count(&v));
    }

    #[test]
    fn test_sum_basic() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(6.0, sum(&v));
    }

    #[test]
    fn test_mean_basic() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(2.0, mean(&v));
    }

    #[test]
    fn test_median_basic() {
        let v = vec![2.0, 1.0, 3.0];
        assert_eq!(2.0, median(&v));
        assert_eq!(1.0, v[1]);
    }

    #[test]
    fn test_min_basic() {
        let v = vec![3.0, 2.0, 1.0];
        assert_eq!(1.0, min(&v));
    }

    #[test]
    fn test_max_basic() {
        let v = vec![3.0, 2.0, 1.0];
        assert_eq!(3.0, max(&v));
    }

    #[test]
    fn test_mode_basic() {
        let v = vec![3.0, 1.0, 1.0];
        assert_eq!(1.0, mode(&v));
    }

    #[test]
    fn test_mode_basic_longer() {
        let v = vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0];
        assert_eq!(4.0, mode(&v));
    }

    #[test]
    fn test_mode_single_val() {
        let v = vec![1.0];
        assert_eq!(1.0, mode(&v));
    }

    #[test]
    fn test_mode_two_vals() {
        let v = vec![1.0, 2.0, 2.0];
        assert_eq!(2.0, mode(&v));
    }

    #[test]
    fn test_mode_three_vals() {
        let v = vec![1.0, 1.0, 2.0, 2.0, 3.0];
        assert_eq!(1.0, mode(&v));
    }

    #[test]
    fn test_stdev_basic() {
        let v = vec![1.0, 4.0];
        let stdev: f64 = stdev(&v);
        let prec = (stdev - 2.12132034).abs();
        assert!(prec < 0.00001);
    }

    #[test]
    fn test_stdev_same_val() {
        let v = vec![1.0, 1.0];
        let stdev: f64 = stdev(&v);
        assert_eq!(0.0, stdev);
    }

    #[test]
    fn test_percentile_basic() {
        let v = vec![1.0];
        let perc: f64 = percentile(&v, 0.9999);
        assert_eq!(1.0, perc);
    }

    #[test]
    fn test_percentile_two_val() {
        let v = vec![1.0, 1.0];
        let perc: f64 = percentile(&v, 0.47);
        assert_eq!(1.0, perc);
    }
}
