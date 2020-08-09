use std::error::Error;
use std::fs::File;
use std::str::FromStr;
use std::process;

fn main() {
    let mut data:Vec<Vec<f32>> = vec![];
    if let Err(err) = read_in_csv(&mut data, "../data/data.csv"){
        println!("error running example: {}", err);
        process::exit(1);
    }
    
    println!("The data {:?}", data)
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

        


