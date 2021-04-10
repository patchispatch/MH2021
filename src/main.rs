mod par;
use par::Problem;
use par::ExecutionRecord;
use par::algorithms::{greedy, local_search};

use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::time::Instant;
use std::collections::HashMap;
use colored::*;
use std::io::{stdout, Write};


fn main() {
    // Initialize random seed
    let seeds: [u64; 5] = [4, 7, 2, 1, 3];

    // Map containing problem instances
    let mut instances = HashMap::new();

    // Zoo
    instances.insert("zoo10", Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_10.const", 7)); 
    instances.insert("zoo20", Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_20.const", 7));

    // Bupa
    instances.insert("bupa10", Problem::from_files("instances/bupa_set.dat", "instances/bupa_set_const_10.const", 16));
    instances.insert("bupa20", Problem::from_files("instances/bupa_set.dat", "instances/bupa_set_const_20.const", 16));

    // Glass
    instances.insert("glass10", Problem::from_files("instances/glass_set.dat", "instances/glass_set_const_10.const", 7));
    instances.insert("glass20", Problem::from_files("instances/glass_set.dat", "instances/glass_set_const_20.const", 7));

    // Execute greedy for every instance five times, saving each one in its respective csv file
    for (key, instance) in instances {
        println!("Executing greedy for instance {}", key);
        let mut wtr = csv::Writer::from_path(format!("results/greedy/{}.csv", key)).unwrap();
        for seed in seeds.iter() {
            print!("Seed {}: ", seed);
            stdout().flush().unwrap();
            let mut rng = Pcg64::seed_from_u64(*seed);
            let now = Instant::now();
            let (_partition, aggr, inf, dev) = greedy(&instance, &mut rng);
            let time = now.elapsed().as_millis();
        
            wtr.serialize(ExecutionRecord {
                instance: key,
                aggregate: aggr,
                infeasibility: inf,
                general_deviation: dev,
                time: time,
            }).unwrap();
            print!("{}\n", "OK".bold().green());
        }
        wtr.flush().unwrap();
    }

    // Execute local search for every instance five times, saving each one in its respective csv file
    for (key, instance) in instances {
        println!("Executing local search for instance {}", key);
        let mut wtr = csv::Writer::from_path(format!("results/local-search/{}.csv", key)).unwrap();
        for seed in seeds.iter() {
            print!("Seed {}: ", seed);
            let mut rng = Pcg64::seed_from_u64(*seed);
            let now = Instant::now();
            let (_partition, aggr, inf, dev) = local_search(&instance, &mut rng);
            let time = now.elapsed().as_millis();
        
            wtr.serialize(ExecutionRecord {
                instance: key,
                aggregate: aggr,
                infeasibility: inf,
                general_deviation: dev,
                time: time,
            }).unwrap();
            print!("{}\n", "OK".bold().green());
        }
        wtr.flush().unwrap();
    }
}
