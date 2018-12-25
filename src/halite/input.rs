pub fn tokenize(string: &mut String) -> Vec<String> {
    let tokens: Vec<String> = Vec::new();
    
    let tokens = string
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
    
    tokens
}


