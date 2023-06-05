use std::collections::HashMap;

use regex::Regex;

fn main() {
    let java = include_str!("../data/GqlOperations.java");
    let regex = Regex::new(r#"(?m)([0-9a-fA-F]{12})", "(\w+)""#).unwrap();
    let result = regex.captures_iter(java);
    let mut hashmap = HashMap::new();
    for mat in result {
        let gql_id = mat.get(1).unwrap().as_str().to_string();
        let action_id = mat.get(2).unwrap().as_str().to_string();
        hashmap.insert(action_id, gql_id);
    }
    let serialized = nanoserde::SerJson::serialize_json(&hashmap);
    std::fs::write("data/GqlOperations.json", serialized).unwrap();
}
