fn iterator_example() {
    let lines_of_code: Vec<String> = [
        "MAIN:".to_string(),
        "STORE #d 887.0".to_string(),
        "STORE #e 888.0".to_string(),
        "ADD #d #e".to_string(),
        "PRINT #d".to_string(),
    ]
    .to_vec();

    let results_of_search: Vec<usize> = lines_of_code
        .iter()
        .enumerate()
        .filter(|&(_, content)| content == "MAIN:")
        .map(|(i, _)| i)
        .collect();

         println!("Result {:?}",results_of_search);
}






fn procedural_example() {
    let lines_of_code: Vec<String> = [
        "MAIN:".to_string(),
        "STORE #d 887.0".to_string(),
        "STORE #e 888.0".to_string(),
        "ADD #d #e".to_string(),
        "PRINT #d".to_string(),
    ]
    .to_vec();

    let mut line_counter = 0;
    let mut results_of_search: Vec<usize> = Vec::new();
    for line in lines_of_code.iter() {
        match line.find("MAIN:") {
            Some(_) => results_of_search.push(line_counter),
            None => (),
        }

        line_counter += 1;
    }
    println!("Result {:?}",results_of_search);
}

fn main() {
    println!("Hello, world!");
    iterator_example();
    procedural_example();
}
