#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, time::{Duration, Instant}};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

mod plotting;

const TARGET: &str = "methinks it is like a weasel";
const VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyz ";
const DEBUG: bool = true;

#[derive(Debug)]
struct Config {
    population_size: usize,
    mutation_rate: f64,
    tournament_size: usize,
}

impl Config {
    fn new(population_size: usize, mutation_rate: f64, tournament_size: usize) -> Self {
        Config {
            population_size,
            mutation_rate,
            tournament_size,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            population_size: 100,
            mutation_rate: 1.0 / (TARGET.len() as f64),
            tournament_size: 10,
        }
    }
}

#[derive(Debug)]
struct OutputData {
    iterations: usize,
    evaluations: usize,
    time: Duration,
}

fn main() {

    let config = Config::new(500, 1.0 / (TARGET.len() as f64), 2);

    let _ = ga_with_crossover(&config);

    // question1();        
    // question2();
}

fn question1() {
    // population data for question 1
    let x_data = vec![10.0, 50.0, 100.0, 250.0, 500.0, 1000.0];

    let crossover_y_data = x_data.clone().iter()
        .map(|&p| Config { population_size: p as usize, ..Default::default()})
        .map(|config| ga_with_crossover(&config))
        .map(|data| data.evaluations)
        .collect::<Vec<_>>();

    let tournament_y_data = x_data.clone().iter()
        .map(|&p| Config { population_size: p as usize, ..Default::default()})
        .map(|config| ga_with_tournament_selection(&config))
        .map(|data| data.evaluations)
        .collect::<Vec<_>>();

    plotting::plot_bar("Crossover - Population Size Vs. Evaluations", &x_data, &crossover_y_data);
    plotting::plot_bar("No Crossover - Population Size Vs. Evaluations", &x_data, &tournament_y_data);

    // It is clear that crossover reduces the number of evaluations needed

    // Uniform crossover may have problems if the fitnes function is not per character
}

fn question2() {
    // varying mutation rate
    let x_data = vec![0.1, 0.2, 0.5, 1.0, 2.0, 5.0];

    let crossover_y_data = x_data.clone().iter()
        .map(|&m| Config { mutation_rate: m / (TARGET.len() as f64), ..Default::default()})
        .map(|config| ga_with_crossover(&config))
        .map(|data| data.evaluations)
        .collect::<Vec<_>>();

    let tournament_y_data = x_data.clone().iter()
        .map(|&m| Config { mutation_rate: m / (TARGET.len() as f64), ..Default::default()})
        .map(|config| ga_with_tournament_selection(&config))
        .map(|data| data.evaluations)
        .collect::<Vec<_>>();
    

    plotting::plot_bar("Crossover - Mutation Rate Vs. Evaluations", &x_data, &crossover_y_data);
    plotting::plot_bar("No Crossover - Mutation Rate Vs. Evaluations", &x_data, &tournament_y_data);
}

fn generate_random_character() -> char {
    let index = rand::thread_rng().gen_range(0..VALID_CHARS.len());
    VALID_CHARS.chars().nth(index).unwrap()
}

fn generate_random_string() -> String {
    (0..TARGET.len())
        .map(|_| generate_random_character())
        .collect()
}

fn fitness_score(individual: &str) -> usize {
    individual
        .chars()
        .zip(TARGET.chars())
        .filter(|(a, b)| a == b)
        .count()
}

fn generate_mutated(individual: &str, mutation_rate: f64) -> String {
    individual
        .chars()
        .map(|c| {
            if rand::random::<f64>() < mutation_rate {
                generate_random_character()
            } else {
                c
            }
        })
        .collect()
}

fn mutation_hill_climber(config: &Config) -> OutputData {
    let mut individual = generate_random_string();

    let mut fitness_evaluations: HashMap<String, usize> = HashMap::new();
    let mut iterations = 0;
    let mut fitness = fitness_evaluations.entry(individual.clone()).or_insert(fitness_score(&individual)).clone();

    let timer = Instant::now();

    while fitness < TARGET.len() {
        let mutated = generate_mutated(&individual, config.mutation_rate);
        let mutated_fitness = fitness_evaluations.entry(mutated.clone()).or_insert(fitness_score(&mutated)).clone();
        if mutated_fitness > fitness {
            individual = mutated;
            fitness = mutated_fitness;
        }
        iterations += 1;
        if DEBUG && iterations % 1000 == 0 {
            println!("{} with fitness {} after {} evaluations", individual, fitness, fitness_evaluations.len());
        }
    }

    let out = OutputData { 
        iterations,
        evaluations: fitness_evaluations.len(),
        time: timer.elapsed()
    };
    if DEBUG { println!("{:?}", out); }
    out 
}

fn ga_with_tournament_selection(config: &Config) -> OutputData {
    let mut population = (0..config.population_size)
        .map(|_| generate_random_string())
        .collect::<Vec<String>>();

    let mut fitness_evaluations: HashMap<String, usize> = HashMap::new();
    let mut best_fitness = 0;
    let mut iterations = 0;

    let timer = Instant::now();

    while best_fitness < TARGET.len() {
        // randomly choose two individuals and select the best to be the parent
        let parent = population.choose_multiple(&mut rand::thread_rng(), config.tournament_size)
            .max_by_key(|&i| fitness_evaluations.entry(i.clone()).or_insert(fitness_score(i)).clone())
            .unwrap();
        
        // mutate the parent to create the child
        let child = generate_mutated(parent, config.mutation_rate);

        // replace the worst individual from 2 random individuals
        let worst = population.choose_multiple(&mut rand::thread_rng(), config.tournament_size)
            .min_by_key(|&i| fitness_evaluations.entry(i.clone()).or_insert(fitness_score(i)).clone())
            .unwrap();

        // Would be nice to just change the value of worst, however, there is no choose_multiple_mut
        let worst_index = population.iter().position(|i| i == worst).unwrap();
        population[worst_index] = child;

        iterations += 1;
        best_fitness = *fitness_evaluations.values().clone().max().unwrap_or(&0);

        if DEBUG && iterations % 1000 == 0 {
            println!("Best fitness {} after {} evaluations", best_fitness, fitness_evaluations.len());
        }
    }   

    let out = OutputData {
        iterations,
        evaluations: fitness_evaluations.len(),
        time: timer.elapsed()
    };
    if DEBUG { println!("{:?}", out); }
    out
}

fn generate_crossover(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .map(|(a, b)| if rand::random::<bool>() { a } else { b })
        .collect()
}

fn ga_with_crossover(config: &Config) -> OutputData {
    println!("Running crossover with {:?}", config);
    let mut population = (0..config.population_size)
        .map(|_| generate_random_string())
        .collect::<Vec<String>>();
    
    let mut fitness_evaluations: HashMap<String, usize> = HashMap::new();
    let mut best_fitness = 0;
    let mut iterations = 0;

    let timer = Instant::now();

    while best_fitness < TARGET.len() // && iterations < 20
    {
        // randomly choose two individuals and select the best to be the parent
        let parent_a = population.choose_multiple(&mut rand::thread_rng(), config.tournament_size)
            .max_by_key(|&i| fitness_evaluations.entry(i.clone()).or_insert(fitness_score(i)).clone())
            .unwrap();
        let parent_b = population.choose_multiple(&mut rand::thread_rng(), config.tournament_size)
            .max_by_key(|&i| fitness_evaluations.entry(i.clone()).or_insert(fitness_score(i)).clone())
            .unwrap();
        
        // crossover the parents to create the child
        let mut child = generate_crossover(parent_a, parent_b);
        child = generate_mutated(&child, config.mutation_rate);

        // replace the worst individual from 2 random individuals
        let worst = population.choose_multiple(&mut rand::thread_rng(), config.tournament_size)
            .min_by_key(|&i| fitness_evaluations.entry(i.clone()).or_insert(fitness_score(i)).clone())
            .unwrap();

        // Would be nice to just change the value of worst, however, there is no choose_multiple_mut
        let worst_index = population.iter().position(|i| i == worst).unwrap();
        population[worst_index] = child;

        iterations += 1;
        best_fitness = *fitness_evaluations.values().max().clone().unwrap_or(&0);

        if DEBUG && iterations % 1000 == 0 {
            println!("Best fitness {} after {} evaluations", best_fitness, fitness_evaluations.len());
        }
    }

    let out = OutputData {
        iterations,
        evaluations: fitness_evaluations.len(),
        time: timer.elapsed(),
    };
    if DEBUG { println!("{:?}", out); }
    out
}
