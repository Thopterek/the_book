/*
 * Usage of lifetime to mark what string should return Vector have a ref to
 * and using lines() to let go with line by line content, returns Iterator
 * contains -> checks if the string has the query
*/
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // creating the Vector
    let mut result = Vec::new();
    // macro that tells to panic with message -> not implemented
    // unimplemented!();
    for line in content.lines() {
        if line.contains(query) {
            // typical style of handling
            result.push(line);
        }
    }
    result
}

pub fn insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // for full Unicode handling it would need to be reworked
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        // the \ is telling not to put the newline
        let content = "Well \
        the thing is:
        it can be
fast, safe or productive
        Pick two";
        assert_eq!(vec!["fast, safe or productive"], search(query, content));
    }
    #[test]
    fn case_insensitive() {
        let query = "RUST";
        let content = "TryinG sOme
rUsT
Trust me";
        assert_eq!(vec!["rUsT", "Trust me"], insensitive_search(query, content));
    }
}
