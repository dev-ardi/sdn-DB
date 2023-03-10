use std::collections::HashMap;

fn main() {
    let s_pets = Schema {
        name: "pets".to_string(),
        values: HashMap::from([("name".to_string(), 0), ("age".to_string(), 1), ("breed".to_string(), 2)])
    };
    let s_owner = Schema {
        name: "owner".to_string(),
        values: HashMap::from([("name".to_string(), 0), ("age".to_string(), 1), ("address".to_string(), 2)])
    };
    let s_root = Schema {
        name: "root".to_string(),
        values: HashMap::new()
    };

    let root = instance_builder(&s_root, vec![]);
    let mut  owners = vec![
        instance_builder(&s_owner, vec![("name", "John".to_string()), ("age", "30".to_string()), ("address", "123 Main St".to_string())]),
        instance_builder(&s_owner, vec![("name", "Jane".to_string()), ("age", "25".to_string()), ("address", "456 Main St".to_string())]),
    ];
    let mut pets = vec![
        instance_builder(&s_pets, vec![("name", "Fido".to_string()), ("age", "5".to_string()), ("breed", "Labrador".to_string())]),
        instance_builder(&s_pets, vec![("name", "Spot".to_string()), ("age", "3".to_string()), ("breed", "Poodle".to_string())]),
        instance_builder(&s_pets, vec![("name", "Rover".to_string()), ("age", "7".to_string()), ("breed", "Golden Retriever".to_string())]),
    ];

    owners[0].add_child(pets.swap_remove(0));
    owners[0].add_child(pets.swap_remove(1));
    owners[1].add_child(pets.swap_remove(2));
    root.add_child(owners.swap_remove(0));
    root.add_child(owners.swap_remove(1));



}

struct Schema {
    name: String,
    values: HashMap<String, usize>,
}
struct Instance<'schema> {
    schema: &'schema Schema,
    value: Vec<String>,
    children: Option<Vec<Instance<'schema>>>,
}
impl<'schema> Instance<'schema> {
    fn get(&self, key: &str) -> Option<&str> {
        self.schema.values.get(key)
        .map(|index| self.value.get(*index)
        .map(|r| r.as_str()))
        .flatten()
    }
    fn add_child(&mut self, child: Instance<'schema>) { // the schema for the child should live at least as long as the parent.
        match &mut self.children {
            Some(children) => children.push(child),
            None => self.children = Some(vec![child]),
        }
    }
    fn recurse(&self, condition: ) {
        
    }
}


fn instance_builder<'a>(schema: &'a Schema, input: Vec<(&str, String)>) -> Instance<'a> {
   let mut value = Vec::new();
   for (k, v) in input.into_iter() {
        let i = schema.values.get(k).unwrap();
        value[*i] = v;
   }

    Instance {
        schema,
        value,
        children: None,
    }
}