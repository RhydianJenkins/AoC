use std::fs::read_to_string;

#[derive(Debug)]
#[allow(dead_code)]
#[allow(dead_code)]
struct Node<'a> {
    name: &'a str,
    value: usize,
    children: Vec<&'a Node<'a>>,
    parent: Box<Option<Node<'a>>>,
}

pub fn solve() -> &'static str {
    let input = read_to_string("./src/days/day7/input.txt").unwrap();
    let raw_input: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();

    println!("{:?}", read_tree(&raw_input));

    "Day seven soluition"
}

#[allow(unused)]
fn read_tree<'a>(input: &'a Vec<&'a str>) -> Result<Node<'a>, &str> {
    let mut root = Node {
        name: "/",
        value: 0,
        children: vec![],
        parent: Box::new(None),
    };

    let mut current_node: &mut Node = &mut root;

    for line in input {
        if line.contains("cd") {
            if line.contains("/") {
                current_node = &mut root;
                println!("Going to root");
                continue;
            }

            let change_dir_name = line.chars().last().unwrap().to_string();
            let found_node = current_node
                .children
                .iter()
                .find(|child| *child.name == change_dir_name);

            if found_node.is_some() {
                // TODO fetch/save child node

                // current_node = *found_node.unwrap();
                continue;
            }

            // TODO create/save child node

            // let mut new_child_node = create_child_node(&change_dir_name.clone().trim()).unwrap();
            // current_node.children.push(&new_child_node);
            // current_node = &new_child_node;
            continue;
        }
    }

    Ok(root)
}

#[allow(dead_code)]
fn create_child_node<'a>(name: &'a str) -> Result<Node<'a>, &'a str> {
    Ok(Node {
        name,
        value: 0,
        children: vec![],
        parent: Box::new(None),
    })
}
