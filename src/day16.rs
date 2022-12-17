use std::collections::HashMap;

use crate::error::Result;

fn solve(graph: Vec<Vec<usize>>, node_values: Vec<i64>, node: usize) -> i64 {
    let mut cache = HashMap::new();
    solve_internal(&mut cache, &graph, node, 30, node_values)
}

fn solve_internal(
    cache: &mut HashMap<(usize, i64, Vec<i64>), i64>,
    graph: &Vec<Vec<usize>>,
    node: usize,
    time_left: i64,
    node_values: Vec<i64>,
) -> i64 {
    if time_left == 0 || node_values.iter().all(|x| *x == 0) {
        return 0;
    }
    if cache.contains_key(&(node, time_left, node_values.clone())) {
        cache[&(node, time_left, node_values)]
    } else {
        // substructure: visit neighbors with 1 less time, or open valve of current node
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
        if node_values[node] != 0 {
            let mut node_values_prime = node_values.clone();
            node_values_prime[node] = 0;
            max_val = max_val.max(
                solve_internal(cache, graph, node, time_left - 1, node_values_prime)
                    + node_values[node] * (time_left - 1),
            );
        }

        cache.insert((node, time_left, node_values), max_val);
        max_val
    }
}

fn parse_input() -> Result<(Vec<Vec<usize>>, Vec<i64>, HashMap<String, usize>)> {
    let f = String::from_utf8(std::fs::read("inputs/day16a")?)?;
    let mut name_map = HashMap::new();

    let mut graph = Vec::new();
    let mut node_values = Vec::new();

    for (i, l) in f.lines().enumerate() {
        let split: Vec<_> = l.split(" ").collect();
        let x = split[4].split("=").collect::<Vec<_>>();
        node_values.push(x[1].split(";").next().unwrap().parse::<i64>().unwrap());
        name_map.insert(split[1].to_string(), i);
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

    Ok((graph, node_values, name_map))
}

pub fn solve_a() -> Result<i64> {
    let (graph, node_values, name_map) = parse_input()?;
    let s = solve(graph, node_values, name_map[&"AA".to_string()]);

    Ok(s)
}

fn solve_for_b(graph: Vec<Vec<usize>>, node_values: Vec<i64>, node: usize) -> i64 {
    let mut cache = HashMap::new();
    solve_internal_b(&mut cache, &graph, (node, node), 26, node_values)
}

fn solve_internal_b(
    cache: &mut HashMap<((usize, usize), i64, Vec<i64>), i64>,
    graph: &Vec<Vec<usize>>,
    node: (usize, usize),
    time_left: i64,
    node_values: Vec<i64>,
) -> i64 {
    if time_left == 0 || node_values.iter().all(|x| *x == 0) {
        return 0;
    }
    if cache.contains_key(&(node, time_left, node_values.clone())) {
        cache[&(node, time_left, node_values)]
    } else {
        // substructure: visit neighbors with 1 less time, or open valve of current node
        let mut max_val = 0;
        let mut node_values_open_0 = node_values.clone();
        node_values_open_0[node.0] = 0;

        let mut node_values_open_1 = node_values.clone();
        node_values_open_1[node.1] = 0;

        let mut node_values_open_both = node_values.clone();
        node_values_open_both[node.0] = 0;
        node_values_open_both[node.1] = 0;

        // 0 opens, 1 travels
        if node_values[node.0] != 0 {
            for neigh in &graph[node.1] {
                // move to that node and solve subproblem
                max_val = max_val.max(
                    solve_internal_b(
                        cache,
                        graph,
                        (node.0, *neigh),
                        time_left - 1,
                        node_values_open_0.clone(),
                    ) + node_values[node.0] * (time_left - 1),
                );
            }
        }

        // 0 travels, 1 opens
        if node_values[node.1] != 0 {
            for neigh in &graph[node.0] {
                // move to that node and solve subproblem
                max_val = max_val.max(
                    solve_internal_b(
                        cache,
                        graph,
                        (*neigh, node.1),
                        time_left - 1,
                        node_values_open_1.clone(),
                    ) + node_values[node.1] * (time_left - 1),
                );
            }
        }
        // both open
        if node.0 != node.1 && node_values[node.0] != 0 && node_values[node.1] != 0 {
            max_val = max_val.max(
                solve_internal_b(
                    cache,
                    graph,
                    node,
                    time_left - 1,
                    node_values_open_both.clone(),
                ) + node_values[node.1] * (time_left - 1)
                    + node_values[node.0] * (time_left - 1),
            );
        }

        // both travel
        for neigh0 in &graph[node.0] {
            for neigh1 in &graph[node.1] {
                max_val = max_val.max(solve_internal_b(
                    cache,
                    graph,
                    (*neigh0, *neigh1),
                    time_left - 1,
                    node_values.clone(),
                ));
            }
        }

        cache.insert((node, time_left, node_values), max_val);
        max_val
    }
}

pub fn solve_b() -> Result<i64> {
    let (graph, node_values, name_map) = parse_input()?;
    let s = solve_for_b(graph, node_values, name_map[&"AA".to_string()]);
    Ok(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a().unwrap(), 1751);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b().unwrap(), 26686);
    }
}
