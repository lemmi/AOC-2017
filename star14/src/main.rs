use std::io;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone,Debug,Default)]
struct Graph {
    root: String,
    nodes: HashMap<String,Node>,
    edges: HashMap<String,Vec<String>>,
}

#[derive(Clone,Default,Debug)]
struct Node {
    name: String,
    value: i32,
    parent: String,
}

impl Node {
    fn new(name: &str, value: i32) -> Node {
        Node{
            name: name.to_string(),
            value: value,
            parent: String::new(),
        }
    }
}

impl Graph {
    fn new() -> Graph {
        Graph::default()
    }
    
    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.clone(), node);
    }
    
    fn add_edge(&mut self, from: &str, to: &str) {
        let values = self.edges.entry(from.to_string()).or_insert(Vec::new());
        values.push(to.to_string());
    }

    fn parse(&mut self, line: &str) {
        let v: Vec<&str> = line.split(" -> ").collect();

        let nodestr: Vec<&str> = v[0].split_whitespace().collect();
        if nodestr.len() != 2 {
            panic!("Wrong number of elements {:?}", nodestr);
        }

        let nodename = nodestr[0];
        let nodevalue: i32 = nodestr[1]
            .trim_left_matches('(')
            .trim_right_matches(')')
            .parse()
            .expect("Not a number!");
        
        self.add_node(Node::new(nodename, nodevalue));

        if v.len() == 1 {
            return
        }

        let edgestr: Vec<&str> = v[1].split(',')
            .map(|s| s.trim())
            .collect();

        for to in edgestr {
            self.add_edge(nodename, to);
        }
    }

    fn rebuild(&mut self) {
        for (parent, children) in self.edges.iter() {
            for child in children {
                match self.nodes.get_mut(child) {
                    Some(n) => n.parent = parent.clone(),
                    None => panic!("Child does not exist"),
                }
            }
        }
        
        let root = self.nodes.keys().next().unwrap();
        let mut root = &self.nodes[root];

        while root.parent.len() > 0 {
            root = &self.nodes[&root.parent];
        }

        self.root = root.name.clone();
    }

    fn children(&self, subtree: &str) -> Vec<String> {
        if let Some(e) = self.edges.get(subtree) {
            e.clone()
        } else {
            Vec::new()
        }
    }
    fn subtreeweights(&mut self, subtree: &str) -> Vec<i32> {
        self.children(subtree)
            .iter()
            .map(|e| self.weight(e))
            .collect()
    }

    fn weight(&mut self, subtree: &str) -> i32 {
        let nodeval = self.nodes[subtree].value;
        let mut subtreeweights = self.subtreeweights(subtree);
        if let Some((pos, expect, got)) = Graph::find_unique(&subtreeweights) {
            println!("DETECTED UNIQUE VALUE {} at pos {}, expected {}", got, pos, expect);
            let child = &self.children(subtree)[pos];
            self.fix_subtree(child, expect);
            subtreeweights = self.subtreeweights(subtree);
        }
        let subtreeval = subtreeweights.iter().sum::<i32>();
        let sum = nodeval + subtreeval;
        println!("Subtree {}: {} + {} = {} ({:?})", subtree, nodeval, subtreeval, sum, subtreeweights);
        sum
    }

    fn fix_subtree(&mut self, subtree: &str, expect: i32) {
        println!("Fixing {}", subtree);
        let subtreeweights = self.subtreeweights(subtree);

        if subtreeweights.len() < 3 {
            let node = self.nodes.get_mut(subtree).unwrap();
            node.value = expect - subtreeweights.iter().sum::<i32>();
            println!("Fixin node value. Now: {}", node.value);
        } else if let Some((pos, expect, _)) = Graph::find_unique(&subtreeweights) {
            let child = &self.children(subtree)[pos];
            self.fix_subtree(child, expect);
        }
        
    }

    fn find_unique(weights: &Vec<i32>) -> Option<(usize, i32, i32)> {
        if weights.len() < 3 {
            return None
        }

        for (i, gr) in weights.windows(3).enumerate() {
            if gr[0] == gr[1] && gr[1] == gr[2] {
                //
            } else if gr[0] != gr[1] && gr[1] == gr[2] {
                return Some((i + 0, gr[1], gr[0]));
            } else if gr[0] != gr[1] && gr[1] != gr[2] {
                return Some((i + 1, gr[0], gr[1]));
            } else if gr[0] == gr[1] && gr[1] != gr[2] {
                return Some((i + 2, gr[0], gr[1]));
            } else {
                unreachable!();
            }
        }
        None
    }
}


fn main() {
    let mut g = Graph::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        g.parse(&line);
    }
    g.rebuild();
    let root = g.root.clone();
    println!("root: {}", g.root);
    g.weight(&root);
}
