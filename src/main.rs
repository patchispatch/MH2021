mod par;
use par::Problem;
use par::ExecutionRecord;
use par::algorithms::{greedy, local_search, generational_genetic};

use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::time::Instant;
use std::collections::HashMap;
use colored::*;
use std::io::{stdout, Write};
use std::env;


fn main() {
    // Command-line arguments
    let args: Vec<String> = env::args().collect();

    // Map containing problem instances
    let mut instances = HashMap::new();
    let mut seeds = Vec::new();

    if args.len() == 2 && args[1] == "all" {
        // Initialize random seed
        seeds = vec![4, 7, 2, 1, 3];

        // Zoo
        instances.insert("zoo10", Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_10.const", 7));
        instances.insert("zoo20", Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_20.const", 7));

        // Bupa
        instances.insert("bupa10", Problem::from_files("instances/bupa_set.dat", "instances/bupa_set_const_10.const", 16));
        instances.insert("bupa20", Problem::from_files("instances/bupa_set.dat", "instances/bupa_set_const_20.const", 16));

        // Glass
        instances.insert("glass10", Problem::from_files("instances/glass_set.dat", "instances/glass_set_const_10.const", 7));
        instances.insert("glass20", Problem::from_files("instances/glass_set.dat", "instances/glass_set_const_20.const", 7));
    }
    else if args.len() == 6 {
        let data_file = &args[1];
        let constraints_file = &args[2];
        let results_file = &args[3];
        let number_of_clusters = args[4].parse::<usize>().unwrap();
        seeds = vec![args[5].parse::<u64>().unwrap()];

        instances.insert(results_file, Problem::from_files(data_file, constraints_file, number_of_clusters));
    }
    // Test purposes
    else {
        instances.insert("zoo10", Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_10.const", 7));
        seeds = vec![4, 7, 2, 1, 3];

        for (key, instance) in instances.iter() {
            println!("Executing generational genetic for instance {}", key);
            let mut wtr = csv::Writer::from_path(format!("results/generational-genetic/{}.csv", key)).unwrap();
            for seed in seeds.iter() {
                print!("Seed {}: ", seed);
                stdout().flush().unwrap();
                let mut rng = Pcg64::seed_from_u64(*seed);
                let now = Instant::now();
                let (_partition, aggr, inf, dev) = generational_genetic(&instance, 50, &mut rng);
                let time = now.elapsed().as_millis();
            
                wtr.serialize(ExecutionRecord {
                    instance: *seed as usize,
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

    
    /*
    // Execute greedy for every instance five times, saving each one in its respective csv file
    for (key, instance) in instances.iter() {
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
                instance: *seed as usize,
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
    for (key, instance) in instances.iter() {
        println!("Executing local search for instance {}", key);
        let mut wtr = csv::Writer::from_path(format!("results/local-search/{}.csv", key)).unwrap();
        for seed in seeds.iter() {
            print!("Seed {}: ", seed);
            stdout().flush().unwrap();
            let mut rng = Pcg64::seed_from_u64(*seed);
            let now = Instant::now();
            let (_partition, aggr, inf, dev) = local_search(&instance, &mut rng);
            let time = now.elapsed().as_millis();
        
            wtr.serialize(ExecutionRecord {
                instance: *seed as usize,
                aggregate: aggr,
                infeasibility: inf,
                general_deviation: dev,
                time: time,
            }).unwrap();
            print!("{}\n", "OK".bold().green());
        }
        wtr.flush().unwrap();
    }
    */
}
