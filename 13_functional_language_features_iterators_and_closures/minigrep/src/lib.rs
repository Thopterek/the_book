/*
 * Usage of lifetime to mark what string should return Vector have a ref to
 * and using lines() to let go with line by line content, returns Iterator
 * contains -> checks if the string has the query
*/
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // updated version with iterrators
    content.lines().filter(|line| line.contains(query)).collect()
}

pub fn insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // for full Unicode handling it would need to be reworked
    // but just updating with iterators
    let query = query.to_lowercase();
    content.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
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

