/*
A network of n nodes labeled 1 to n is provided
along with a list of travel times for directed edges represented as times[i]=(xi, yi, ti),
where xi is the source node, yi is the target node, and ti is the delay time from the source node to the target node.

Considering we have a starting node, k, we have to determine the minimum time required for all the remaining for n - 1 nodes to receive the signal.
Return −1 if it is not possible for all n - 1 nodes to receive the signal.

Example:  [[2, 1, 1], [3, 2, 1], [3, 4, 2]], n = 4, k = 3
*/

fn main() {
    let time = [[2, 1, 1], [3, 2, 1], [3, 4, 2]];
    let n = 4;
    let k = 3;

    let result = network_delay_time(&time, n, k);
    assert_eq!(result, Some(2));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Edge {
    target: usize,
    weight: usize,
}

impl Edge {
    fn new(target: usize, weight: usize) -> Self {
        Self { target, weight }
    }
}

fn network_delay_time(time: &[[usize; 3]], n: usize, k: usize) -> Option<usize> {
    use std::collections::{HashMap, BinaryHeap};

    let mut adj_map: HashMap<usize, Vec<Edge>> =
        time.iter().fold(HashMap::new(), |mut map, nodes| {
            let source = nodes[0];
            let target = nodes[1];
            let weight = nodes[2];
            let edge = Edge::new(target, weight);

            map.entry(source)
                .and_modify(|edges| edges.push(edge))
                .or_insert(vec![edge]);

            map
        });

    // let mut q: Edge = BinaryHeap::new();

    println!("{adj_map:?}");

    None
}
