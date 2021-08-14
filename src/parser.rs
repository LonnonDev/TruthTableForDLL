#[derive(Debug)]
struct Circuit {
    name: String,
    inputs: Vec<(String, u64)>,
    outputs: Vec<(String, u64)>,
    command: String,
    meaning: (Vec<String>, Vec<u64>),
    output_values: Vec<i32>
}

pub fn parse() {

}