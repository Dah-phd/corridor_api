use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

enum NextNodeResult<T> {
    Ok(T),
    Finished,
    Err,
}

struct AstarQue {
    max_xy_coordinates: (usize, usize),
    que: Vec<Node>,
    closed: Vec<Node>,
}

impl AstarQue {
    pub fn new(start: (usize, usize), target: usize, max_xy_coordinates: (usize, usize)) -> Self {
        Self {
            max_xy_coordinates,
            que: vec![Node::new(start, target)],
            closed: Vec::new(),
        }
    }

    pub fn run(&mut self, validator: fn((usize, usize), (usize, usize)) -> bool) -> Result<Vec<(usize, usize)>, ()> {
        // validators confirms that the move is possible
        loop {
            self.que.sort();
            let top = self.que.remove(0);
            let possible_paths = Self::get_all_directions(&top);
            if possible_paths.len() != 0 {
                for possible_path in possible_paths {
                    match top.next_node(possible_path) {
                        NextNodeResult::Ok(v) => self.que.push(v),
                        NextNodeResult::Err => return Result::Err(()),
                        NextNodeResult::Finished => return Ok(self.reconstruct_path(top)),
                    }
                }
            }
            self.closed.push(top);
        }
    }

    fn get_all_directions(from_node: &Node) -> Vec<(usize, usize)> {
        let result = Vec::new();
        if from_node.position.0 != 0 {}
        if from_node.position.1 != 0 {}
        result
    }

    fn reconstruct_path(&self, opt: Node) -> Vec<(usize, usize)> {
        let mut fastest_path = vec![opt.position];
        let mut opt = self.pull_previous_position(&opt);
        loop {
            fastest_path.push(opt.position);
            if opt.comes_from.is_none() {
                return fastest_path;
            }
            opt = self.pull_previous_position(opt);
        }
    }

    fn pull_previous_position(&self, node: &Node) -> &Node {
        for closed_node in self.closed.iter() {
            if closed_node.position == node.comes_from.unwrap() {
                return closed_node;
            }
        }
        panic!("MISSING NODE IN CLOSED QUE VEC!")
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
