use std::error::Error;
use std::fs;

pub struct Inputs<'a> {
    pub query: &'a String,
    pub filename: &'a String,
    pub case_sensitive: bool
}
impl<'a> Inputs<'a> {
    pub fn new(args: &Vec<String>) -> Result<Inputs,&str> {
        if args.len() < 3 {
            return Err("Not enough arguments.")
        }
        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = args[3].parse::<bool>()
            .map_err(|_| "Invalid boolean value for case_sensitive")?;
        
        Ok(Inputs {query, filename, case_sensitive})
    }
}
pub fn run (inputs: Inputs) -> Result<(),Box<dyn Error>> {
    println!("Query: {}",inputs.query);
    println!("Filename: {}",inputs.filename);
    
    let file_content = fs::read_to_string(inputs.filename)?;
    search(inputs.query, &file_content, inputs.case_sensitive);
    Ok(())
}

pub fn search(query: & str, content: &str, case_sens: bool) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    if case_sens {
        for line in content.lines() {
            if line.contains(query) {
                result.push(line.to_owned());
            }
        }
    } else {
        for line in content.lines() {
            if line.to_lowercase().contains(query) {
                result.push(line.to_lowercase());
            }
        }
    }

    println!("{:?}",result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_grep() {
        let query = "golden";
        let content = "its golden network from the green waves \nto be like the golden sun that slowly expires";
        let case = true;
        assert_eq!(vec!["its golden network from the green waves ",
                        "to be like the golden sun that slowly expires"],search(&query,&content,case));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "golden";
        let content = "its Golden network from the green waves \nto be like the Golden sun that slowly expires";
        let case = false;
        assert_eq!(vec!["its golden network from the green waves ",
                        "to be like the golden sun that slowly expires"],search(&query,&content,case));
        
    }
}