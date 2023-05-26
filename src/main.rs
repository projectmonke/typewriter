use atty;
use regex::Regex;
use std::fs::File;
use std::str::FromStr;
use std::error::Error;
use clap::{ArgGroup, Parser};
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Parser)]
#[command(author = "Monke (https://hackerone.com/monke)", version = "1.0.0", about, long_about = None)]
#[clap(group = ArgGroup::new("input").required(true).args(&["stdin", "filename", "domain"]))]
struct Typewriter {
    /// The depth to generate subdomains at.
    #[clap(short = 'd', long = "depth", default_value = "1")]
    depth: i32,

    /// Ingest subdomains from stdin
    #[clap(short = 's', long = "stdin")]
    stdin: bool,

    /// Generate permutations for one domain or subdomain
    #[clap(short = 'i', long = "input")]
    domain: Option<String>,

    /// Ingest subdomains from a file
    #[clap(short = 'f', long = "file")]
    filename: Option<String>,

    /// The wordlist to generate permutations with
    #[clap(short = 'w', long = "wordlist")]
    wordlist: String,
}

fn main() {
    let typewriter = Typewriter::parse();
    let generated_permutations = generate_permutations(typewriter.wordlist, typewriter.depth).expect("Failed to generate permutations");
    
    if typewriter.stdin {
        if atty::is(atty::Stream::Stdin) {
            println!("No stdin found.");
        } else {
            let generated_domains: HashSet<String> = io::stdin()
                .lock()
                .lines()
                .filter_map(Result::ok)
                .map(|line| line.replace(' ', ""))
                .filter(|line| line.split('.').count() >= 2)
                .collect::<HashSet<String>>();

            for domain in generated_domains.iter(){
                permutator(domain, &generated_domains, &generated_permutations, typewriter.depth, true);
            }
        }
    } else if let Some(file) = typewriter.filename {
        let generated_domains = generate_domains_from_file(file).expect("Failed to generate domains.");
        for domain in generated_domains.iter(){
            permutator(domain, &generated_domains, &generated_permutations, typewriter.depth, true);
        }
    } else if let Some(domain) = typewriter.domain{
        let mut generated_domains: HashSet<String> = HashSet::new();
        generated_domains.insert(domain.clone());
        permutator(&domain, &generated_domains , &generated_permutations, typewriter.depth, true);
    }
}

fn permutator(domain: &str, domains: &HashSet<String>, permutations: &HashSet<String>, depth: i32, first_time: bool){
    if depth < 1 {
        return
    }

    for permutation in permutations.iter(){
        let joins = get_joins(domain, permutation, first_time);
        for join in joins.iter(){
            let new_subdomain = format!("{}{}{}", permutation, join, domain);
            if new_subdomain.split('.').count() > 2{
                if !domains.contains(&new_subdomain){
                    println!("{}", new_subdomain);
                    permutator(&new_subdomain, domains, permutations, depth - 1, false);
                }
            }
        }
        if depth == 1 && first_time{
            if domain.len() > 2 {
                let domain_split: Vec<&str> = domain.split('.').collect();
                let first_item = domain_split[0];
                let subdomain_first_item = first_item
                    .chars()
                    .filter(|c| !c.is_digit(10))
                    .collect::<String>();
                let new_permutation = permutation
                    .chars()
                    .filter(|c| !c
                        .is_digit(10))
                        .collect::<String>();
                
                if subdomain_first_item != new_permutation && !subdomain_first_item.ends_with(&new_permutation) {
                        let new_subdomain = format!("{}{}{}", first_item, permutation, domain_split[1..].join("."));
                        if new_subdomain.split('.').count() > 2{
                            if !domains.contains(&new_subdomain) {
                                println!("{}", new_subdomain);
                            }
                        }

                        let new_subdomain = format!("{}-{}{}", first_item, permutation, domain_split[1..].join("."));
                        if new_subdomain.split('.').count() > 2 {
                            if !domains.contains(&new_subdomain) {
                                println!("{}", new_subdomain);
                            }
                        }
                        
                    }
                }
        }  
    }
}

fn generate_domains_from_file(filename: String) -> io::Result<HashSet<String>> {
    let domains_file = File::open(filename)?;
    let mut domains = HashSet::new();
    
    for line in io::BufReader::new(domains_file).lines() {
        let line = line?;
        let cleaned_line = line.replace(' ', "");
        if cleaned_line.split('.').count() < 2 || domains.contains(&cleaned_line) {
            continue;
        }
        domains.insert(cleaned_line);
    }
    Ok(domains.into_iter().collect())
}

fn generate_permutations(filename: String, permutation_number: i32) -> io::Result<HashSet<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut new_permutations: HashSet<String> = HashSet::new();
    let numbers_pattern = Regex::new("\\d+").unwrap();

    for line in reader.lines() {
        let line = line?;
        let cleaned_line = line.replace(' ', "");
        if !new_permutations.insert(cleaned_line.clone()) {
            continue;
        }
        go_bananas(&mut new_permutations, &numbers_pattern, &cleaned_line, permutation_number).expect("Advanced permutation error.");

        let data: Vec<String> = numbers_pattern.find_iter(&line)
            .map(|mat| mat.as_str().to_string())
            .collect();
        if !data.is_empty() {
            permutator_numbers(&mut new_permutations, &line, &data, permutation_number)
                .expect("Number permutation failed.");
        }
    }

    Ok(new_permutations.into_iter().collect())
}

fn permutator_numbers(
    permutations: &mut HashSet<String>,
    permutation: &str,
    data_to_replace: &Vec<String>,
    permutator_number: i32
) -> Result<(), Box<dyn Error>> {
    for num_to_replace in data_to_replace {
        let num = i32::from_str(num_to_replace)?;
        for i in 1..=permutator_number {
            let new_permutation = permutation.replace(num_to_replace, &(num + i).to_string());
            permutations.insert(new_permutation);
            if num - i >= 0 {
                let new_permutation = permutation.replace(num_to_replace, &(num - i).to_string());
                permutations.insert(new_permutation);
            }
        }
    }
    Ok(())
}

fn go_bananas(permutations: &mut HashSet<String>, num_pat: &Regex, data: &str, permutator_number: i32) -> Result<(), Box<dyn Error>> {
    for new_split in data.split('-').filter(|s| !s.is_empty()) {
        let new_split = new_split.to_string();
        if !permutations.contains(&new_split) {
            permutations.insert(new_split.clone());
            let data: Vec<_> = num_pat
                .find_iter(&new_split)
                .map(|m| m.as_str().to_string())
                .collect();
            if !data.is_empty() && permutator_number > 0 {
                permutator_numbers(permutations, &new_split, &data, permutator_number)?;
            }
        }
    }
    Ok(())
}


fn get_joins(domain: &str, permutation: &str, first_time: bool) -> Vec<String> {
    let mut joins = vec![".".to_string(), "-".to_string()];
    let number_prefix = domain.chars().next().map(|c| c.is_digit(10)).unwrap_or(false);
    
    if number_prefix && permutation.chars().last().map(|c| c.is_digit(10)).unwrap_or(false) {
        return joins;
    }

    if domain.split('.').count() == 2 && first_time{
        joins.clear();
        joins.push(".".to_string());  
    }

    let first_element = domain.split('.').next().unwrap_or("");
    match first_element {
        _ if first_element == permutation => {
            joins.clear();
        },
        _ if permutation.len() >= 4 && first_element.starts_with(permutation) => {
            joins.clear();
            joins.extend(vec![".".to_string(), "-".to_string()]);        },
        _ => {
            let subdomain_first_element = first_element.chars().filter(|c| !c.is_digit(10)).collect::<String>();
            let new_permutation = permutation.chars().filter(|c| !c.is_digit(10)).collect::<String>();
            if subdomain_first_element == new_permutation {
                joins.clear();
            } else if subdomain_first_element.ends_with(&new_permutation) {
                joins.clear();
                joins.push(".".to_string());            
            }
        }
    }
    joins
}
