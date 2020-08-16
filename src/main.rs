use std::error::Error;
use std::fs::File;
use std::str::FromStr;
use std::process;
use rand::prelude::*;
use rand_distr::StandardNormal;

fn main() {
    let mut data:Vec<Vec<f32>> = vec![];
    if let Err(err) = read_in_csv(&mut data, "../data/data.csv"){
        println!("error running example: {}", err);
        process::exit(1);
    }
    
    // println!("The data {:?}", data);
    let gauss_prob: GaussianProb = GaussianProb::new(4);
    println!("Random gauss {:?}", gauss_prob);
}

fn read_in_csv(data: &mut Vec<Vec<f32>>, csv_file_name: &str) -> Result<(),Box<dyn Error>> {
    // init data

    // init reader
    let file = File::open(csv_file_name)?;
    let mut rdr = csv::Reader::from_reader(file);
    
    // iterate over data
    for record in rdr.records() {
        let sample = record?;
        let eruptions = f32::from_str(&sample[0]).unwrap();
        let waiting = f32::from_str(&sample[1]).unwrap();
        let sample = vec![eruptions, waiting];
        data.push(sample);
    }
    Ok(())
}


/* pub struct GaussianMixture {
    /// gaussian probability distributions
    gaussian_probs: Vec<GaussionProb>,
    /// probs weights
    probs_weights: Vec<f32>,
}

*/
        

#[derive(Debug)]
pub struct GaussianProb {
    /// mean
    mean: Vec<f32>,
    /// variance
    variance: Vec<f32>,
    ///
    dim: u32,
}
impl GaussianProb {
    pub fn new(
        dim: u32
    ) -> Self {
        let mean: Vec<f32> = get_random_vector(dim);
        let variance: Vec<f32> = get_random_vector(dim);
        GaussianProb {
            mean,
            variance,
            dim
        }
    }
    pub fn from_mean_and_variance(mean: Vec<f32>, variance: Vec<f32>) -> Self {
        let dim = mean.len() as u32;
        GaussianProb {
            mean,
            variance,
            dim
        }
    }
}


fn get_random_vector(dim: u32) -> Vec<f32> {
    let random_vec: Vec<f32> = (0..dim).map(|_| {
        thread_rng().sample(StandardNormal)
        }).collect();
    random_vec
}
