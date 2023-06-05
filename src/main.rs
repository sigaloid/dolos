use std::collections::BTreeMap;

use regex::Regex;

fn main() {
    let java = include_str!("../data/GqlOperations.java");
    let regex = Regex::new(r#"(?m)([0-9a-fA-F]{12})", "(\w+)""#).unwrap();
    let result = regex.captures_iter(java);
    let mut map = BTreeMap::new();
    for mat in result {
        let gql_id = mat.get(1).unwrap().as_str().to_string();
        let action_id = mat.get(2).unwrap().as_str().to_string();
        map.insert(action_id, gql_id);
    }
    let serialized = serde_json::to_string(&map).unwrap();
    std::fs::write("data/GqlOperations.json", serialized).unwrap();
    let mut actions = String::new();
    actions.push_str("enum Actions {\n");
    for (key, _value) in map {
        actions.push_str(&format!("\t{key},\n"));
    }
    actions.push_str("}");
    std::fs::write("src/actions.rs", actions).unwrap();
}
