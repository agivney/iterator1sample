 #![allow(dead_code)]

fn example1() {
    let a = [1, 2, 3];
    let _result: Vec<i32> = a.iter().map(|x| 2 * x).collect();
}

fn example2() {
    let layer_value = 0.034 as f32;
    let weights_to_process = [0.1, 0.3, 0.5]
        .to_vec()
        .iter()
        .map(|x| x * layer_value)
        .collect::<Vec<f32>>();

    let _result: f32 = weights_to_process.iter().sum();
}

fn example3(words: Vec<String>) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .filter(|&(i, _)| i > 0)
        .map(|(_, y)| y.to_string())
        .collect()
}

fn example4_monster_map(line_of_words: Vec<String>) {
    let _filtered_words = line_of_words
        .iter()
        .map(|x| x.to_string() + &" ".to_string())
        .map(|x| x.replace("\"", ""))
        .collect::<Vec<String>>()
        .iter()
        .map(|s| s.chars())
        .flatten()
        .collect::<String>();
}

fn example_flatten_from_rust_docs() {
    let words = ["alpha", "beta", "gamma"];
    // chars() returns an iterator
    let merged: String = words.iter().map(|s| s.chars()).flatten().collect();
    assert_eq!(merged, "alphabetagamma");
}

fn main() {}
