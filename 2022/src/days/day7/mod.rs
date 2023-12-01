use std::fs::read_to_string;

#[derive(Debug)]
#[allow(dead_code)]
struct Node<'a> {
    name: String,
    value: u32,
    children: Vec<Node<'a>>,
    parent: Option<Box<&'a Node<'a>>>,
}

// impl Node {
// fn caluclate_value(&self) -> u32 {
// self.value
// + self
// .children
// .iter()
// .map(|c| c.caluclate_value())
// .sum::<u32>()
// }
// }

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day7/input.txt").unwrap();
    let raw_input: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();

    println!("{:?}", read_tree(&raw_input));

    "Day seven soluition"
}

#[allow(unused)]
fn read_tree<'a>(input: &'a Vec<&'a str>) -> Result<Node, &str> {
    let root = Node {
        name: "/".to_string(),
        value: 0,
        children: vec![],
        parent: None,
    };

    let mut current_node = &root;

    for line in input {
        if line.contains("cd") {
            if line.contains("/") {
                current_node = &root;
                println!("Going to root");
                continue;
            }

            let change_dir_name = line.chars().last().unwrap().to_string();
            let found_child = find_child(&current_node, &change_dir_name);

            if found_child.is_some() {
                current_node = found_child.unwrap();
                continue;
            }

            let new_child_node = create_node(change_dir_name.to_string(), current_node);

            let mut current_node_children = &current_node.children;
            // current_node_children.push(new_child_node); // TODO this doesn't work

            continue;
        }
    }

    Ok(root)
}

#[allow(dead_code)]
fn create_node<'a>(name: String, parent: &'a Node) -> Node<'a> {
    Node {
        name: name.to_string(),
        value: 0,
        children: Vec::new(),
        parent: Some(Box::new(parent)),
    }
}

fn find_child<'a>(input_node: &'a Node<'a>, search_name: &String) -> Option<&'a Node<'a>> {
    let mut child_iter = input_node.children.iter();
    let found_child_node = child_iter.find(|child| *child.name == *search_name);

    found_child_node
}
