extern crate genetic;
extern crate time;

#[cfg(test)]
mod tests {

    use time::PreciseTime;    
    use genetic::*;

    #[test]
    fn test_duplicate_string() {
        let start = PreciseTime::now();
        let gene_set = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!.";
        let target = "Not all those who wander are lost.";

        let wrapped_display = |candidate: &String| {display_duplicate_string(&candidate, target, start);};
        let wrapped_get_fitness = |candidate: &String| -> usize {get_duplicate_string_fitness(&candidate, target)};
    
        let best = genetic::get_best(wrapped_get_fitness, wrapped_display, target.len(), target.len(), gene_set);
        println!("{}", best);
        println!("Total time: {}", start.to(PreciseTime::now()));
        assert_eq!(best,target);
    }
    
    fn display_duplicate_string(candidate: &String, target: &str, start: PreciseTime) {
        let now = PreciseTime::now();
        let elapsed = start.to(now);
        println!("{}\t{}\t{}", candidate, get_duplicate_string_fitness(&candidate, target),elapsed);
    }
    
    fn get_duplicate_string_fitness(candidate: &String, target: &str) -> usize {
        let different_count = target.chars()
            .zip(candidate.chars())
            .filter(|&(a, b)| a != b)
            .count();

        target.len() - different_count
    }
}
