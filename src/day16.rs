use std::collections::HashMap;

use crate::error::Result;

fn solve(graph: Vec<Vec<usize>>, node_values: Vec<i64>, node: usize, players: i64) -> i64 {
    let mut cache = HashMap::new();
    solve_internal(
        &mut cache,
        &graph,
        node,
        node,
        if players > 0 { 26 } else { 30 },
        node_values,
        players,
    )
}

fn solve_internal(
    cache: &mut HashMap<(usize, i64, Vec<i64>, i64), i64>,
    graph: &Vec<Vec<usize>>,
    initial_node: usize,
    node: usize,
    time_left: i64,
    node_values: Vec<i64>,
    players: i64,
) -> i64 {
    if time_left == 0 {
        return if players > 0 {
            solve_internal(
                cache,
                graph,
                initial_node,
                initial_node,
                26,
                node_values,
                players - 1,
            )
        } else {
            0
        };
    }
    if let Some(x) = cache.get(&(node, time_left, node_values.clone(), players)) {
        *x
    } else {
        // substructure: visit neighbors with 1 less time, or open valve of current node
        let mut max_val = 0;
        for neigh in &graph[node] {
            // move to that node and solve subproblem
            max_val = max_val.max(solve_internal(
                cache,
                graph,
                initial_node,
                *neigh,
                time_left - 1,
                node_values.clone(),
                players,
            ));
        }
        // open current node
        if node_values[node] != 0 {
            let mut node_values_prime = node_values.clone();
            node_values_prime[node] = 0;
            max_val = max_val.max(
                solve_internal(
                    cache,
                    graph,
                    initial_node,
                    node,
                    time_left - 1,
                    node_values_prime,
                    players,
                ) + node_values[node] * (time_left - 1),
            );
        }

        cache.insert((node, time_left, node_values, players), max_val);
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
    let s = solve(graph, node_values, name_map[&"AA".to_string()], 0);

    Ok(s)
}

pub fn solve_b() -> Result<i64> {
    let (graph, node_values, name_map) = parse_input()?;
    let s = solve(graph, node_values, name_map[&"AA".to_string()], 1);
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
