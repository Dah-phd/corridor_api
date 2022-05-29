use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

pub trait PathGenerator {
    fn generate_paths(&self, from_position: (usize, usize)) -> Vec<(usize, usize)>;
}

enum NextNodeResult<T> {
    Ok(T),
    Finished,
    Err,
}

pub struct AStar {
    que: Vec<Node>,
    closed: Vec<Node>,
}

impl AStar {
    pub fn new(start: (usize, usize), target: usize) -> Self {
        Self {
            que: vec![Node::new(start, target)],
            closed: Vec::new(),
        }
    }

    pub fn run<T: PathGenerator>(&mut self, from_struct: Box<&T>) -> Result<Vec<(usize, usize)>, String> {
        // validators confirms that the move is possible
        let exposed_struct = *from_struct;
        loop {
            if self.que.is_empty() {
                return Err("no path found".to_owned()); // no elements left therefor no fast way out
            }
            self.que.sort();
            let top = self.que.remove(0);
            let possible_paths = exposed_struct.generate_paths(top.position);
            if possible_paths.len() != 0 {
                for possible_path in possible_paths {
                    if self.pull_from_closed_by_position(possible_path).is_some() {
                        continue;
                    }
                    match top.next_node(possible_path) {
                        NextNodeResult::Ok(v) => self.que.push(v),
                        NextNodeResult::Err => return Result::Err("Impossible path passed!".to_owned()),
                        NextNodeResult::Finished => return Ok(self.reconstruct_path(top)),
                    }
                }
            }
            self.closed.push(top);
        }
    }

    fn reconstruct_path(&self, opt: Node) -> Vec<(usize, usize)> {
        let mut fastest_path = vec![opt.position];
        let mut opt = self.pull_previous_position(&opt);
        loop {
            if opt.is_some() {
                fastest_path.push(opt.unwrap().position);
            } else {
                return fastest_path;
            }
            opt = self.pull_previous_position(opt.unwrap());
        }
    }

    fn pull_previous_position(&self, node: &Node) -> Option<&Node> {
        if node.comes_from.is_some() {
            let result = self.pull_from_closed_by_position(node.comes_from.unwrap());
        }
        None
    }

    fn pull_from_closed_by_position(&self, position: (usize, usize)) -> Option<&Node> {
        for closed_node in self.closed.iter() {
            if closed_node.position == position {
                return Some(closed_node);
            }
        }
        None
    }
}

#[derive(Eq)]
struct Node {
    position: (usize, usize),
    target: usize,
    comes_from: Option<(usize, usize)>,
    cost: usize,
    heuristic_cost: usize,
}

impl Node {
    fn new(position: (usize, usize), target: usize) -> Self {
        Self {
            position,
            target,
            comes_from: Option::None,
            cost: 0,
            heuristic_cost: Self::usize_difference(position.0, target),
        }
    }

    fn same_position(&self, position: (usize, usize)) -> bool {
        self.position == position
    }

    fn next_node(&self, position: (usize, usize)) -> NextNodeResult<Self> {
        if Self::usize_difference(position.0 + position.1, self.position.0 + self.position.1) != 1 {
            return NextNodeResult::Err;
        }
        if position.0 == self.target {
            return NextNodeResult::Finished;
        }
        NextNodeResult::Ok(Self {
            position,
            target: self.target,
            comes_from: Some(self.position),
            cost: self.cost + 1,
            heuristic_cost: Self::usize_difference(position.0, self.target),
        })
    }

    fn usize_difference(x: usize, y: usize) -> usize {
        if x > y {
            return x - y;
        }
        y - x
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic_cost == other.heuristic_cost
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic_cost.cmp(&other.heuristic_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }

    fn ge(&self, other: &Self) -> bool {
        self.heuristic_cost >= other.heuristic_cost
    }
    fn le(&self, other: &Self) -> bool {
        self.heuristic_cost <= other.heuristic_cost
    }
    fn gt(&self, other: &Self) -> bool {
        self.heuristic_cost > other.heuristic_cost
    }
    fn lt(&self, other: &Self) -> bool {
        self.heuristic_cost < other.heuristic_cost
    }
}
