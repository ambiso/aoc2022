use std::collections::HashMap;

use crate::error::Result;

fn solve(graph: Vec<Vec<usize>>, node_values: Vec<i64>) -> i64 {
    let mut cache = HashMap::new();
    solve_internal(&mut cache, &graph, 0, 30, node_values)
}

fn solve_internal(
    cache: &mut HashMap<(usize, i64, Vec<i64>), i64>,
    graph: &Vec<Vec<usize>>,
    node: usize,
    time_left: i64,
    node_values: Vec<i64>,
) -> i64 {
    if time_left == 0 {
        return 0;
    }
    if cache.contains_key(&(node, time_left, node_values.clone())) {
        cache[&(node, time_left, node_values)]
    } else {
        let mut max_val = 0;
        for neigh in &graph[node] {
            // move to that node and solve subproblem
            max_val = max_val.max(solve_internal(
                cache,
                graph,
                *neigh,
                time_left - 1,
                node_values.clone(),
            ));
        }
        // open current node
        let mut node_values_prime = node_values.clone();
        node_values_prime[node] = 0;
        max_val = max_val.max(
            solve_internal(cache, graph, node, time_left - 1, node_values_prime)
                + node_values[node] * (time_left - 1),
        );

        cache.insert((node, time_left, node_values), max_val);
        max_val
    }
}

fn parse_input() -> Result<(Vec<Vec<usize>>, Vec<i64>)> {
    let f = String::from_utf8(std::fs::read("inputs/day16a")?)?;
    let mut name_map = HashMap::new();

    let mut graph = Vec::new();
    let mut node_values = Vec::new();

    for (i, l) in f.lines().enumerate() {
        let split: Vec<_> = l.split(" ").collect();
        let x = split[4].split("=").collect::<Vec<_>>();
        node_values.push(x[1].split(";").next().unwrap().parse::<i64>().unwrap());
        name_map.insert(split[1], i);
        // if nid >= node_values.len() {
        //     node_values.push(nid);
        //     assert_eq!(node_count, node_values.len());
        // }
    }
    for l in f.lines() {
        let split: Vec<_> = l.split(" ").collect();
        graph.push(
            split[9..]
                .join(" ")
                .split(", ")
                .map(|neighbor| name_map[neighbor])
                .collect::<Vec<_>>(),
        );
    }

    Ok((graph, node_values))
}

pub fn solve_a() -> Result<i64> {
    // substructure: f(where I am, how much time is left, values of the (modified) graph)
    let (graph, node_values) = parse_input()?;
    let s = solve(graph, node_values);

    Ok(s)
}

pub fn solve_b() -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert!(solve_a().unwrap() < 2173);
        // assert_eq!(solve_a().unwrap(), 5256611);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 26686);
    }
}
