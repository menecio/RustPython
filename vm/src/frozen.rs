use std::collections::hash_map::HashMap;

const HELLO: &str = "initialized = True
print(\"Hello world!\")
";

pub fn get_module_inits() -> HashMap<String, &'static str> {
    let mut modules = HashMap::new();
    modules.insert("__hello__".to_string(), HELLO);
    modules
}
