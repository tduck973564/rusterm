fn push_string_to(string: &mut String, array: &mut Vec<String>) {
    if string != "" {
        array.push(string.to_string())
    }
    *string = "".to_string();
}

pub fn scan(input: String) -> Vec<String> {
    let mut current = String::new();
    let mut output: Vec<String> = Vec::new();
    let mut in_quote = false;
    
    for character in input.chars() {
        if in_quote {
            if character == '"' {
                in_quote = false;
                push_string_to(&mut current, &mut output);
            } else {
                current += &character.to_string();
            }
        } else {
            if character == '"' {
                in_quote = true;
                push_string_to(&mut current, &mut output);
            } else if character == ' ' {
                push_string_to(&mut current, &mut output);
            } else {
                current += &character.to_string();
            }
        }
    }
    push_string_to(&mut current, &mut output);
    
    output
}
