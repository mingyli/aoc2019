use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

use super::answer::Answer;

type Vertex = String;
type Edge = (Vertex, Vertex);
type Graph = HashMap<Vertex, HashSet<Vertex>>;

fn get_edges<R: BufRead>(reader: &mut R) -> Vec<Edge> {
    let pattern = Regex::new(r"^(?P<planet>.+)\)(?P<satellite>.+)$").unwrap();
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let captures = pattern.captures(&line).unwrap();
            (
                captures["planet"].to_string(),
                captures["satellite"].to_string(),
            )
        })
        .collect()
}

fn get_vertices(edges: &[Edge]) -> HashSet<Vertex> {
    edges
        .iter()
        .flat_map(|(v1, v2)| vec![v1.to_owned(), v2.to_owned()])
        .collect()
}

fn get_parents(edges: &[Edge]) -> HashMap<Vertex, Vertex> {
    edges
        .iter()
        .cloned()
        .map(|(parent, child)| (child, parent))
        .collect()
}

fn num_orbits(
    vertices: &HashSet<Vertex>,
    parents: &HashMap<Vertex, Vertex>,
) -> HashMap<Vertex, u32> {
    fn num_orbits_impl(
        vertex: &str,
        parents: &HashMap<Vertex, Vertex>,
        res: &mut HashMap<Vertex, u32>,
    ) {
        if !res.contains_key(vertex) {
            if parents.contains_key(vertex) {
                let parent = &parents[vertex];
                num_orbits_impl(parent, parents, res);
                res.insert(vertex.to_string(), 1 + res[parent]);
            } else {
                res.insert(vertex.to_string(), 0);
            }
        }
    }
    vertices.iter().fold(HashMap::new(), |mut acc, v| {
        num_orbits_impl(v, parents, &mut acc);
        acc
    })
}

pub fn day6a<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let edges = get_edges(&mut reader);
    let vertices = get_vertices(&edges);
    let parents = get_parents(&edges);
    let orbits = num_orbits(&vertices, &parents);
    Ok(Answer::U(orbits.values().sum()))
}

fn get_graph(edges: &[Edge]) -> Graph {
    edges
        .iter()
        .fold(HashMap::new(), |mut graph: Graph, (v1, v2)| {
            graph
                .entry(v1.to_owned())
                .or_default()
                .insert(v2.to_owned());
            graph
                .entry(v2.to_owned())
                .or_default()
                .insert(v1.to_owned());
            graph
        })
}

fn bfs(graph: &Graph, start: Vertex, end: Vertex) -> Option<u32> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut queue = VecDeque::from(vec![(start, 0)]);
    while !queue.is_empty() {
        let (vertex, dist) = queue.pop_front().unwrap();
        if vertex == end {
            return Some(dist);
        } else if !visited.contains(&vertex) {
            visited.insert(vertex.to_owned());
            let neighbors = graph[&vertex.to_owned()].difference(&visited);
            for neighbor in neighbors {
                queue.push_back((neighbor.to_owned(), dist + 1));
            }
        }
    }
    None
}

pub fn day6b<R: BufRead>(mut reader: &mut R) -> io::Result<Answer> {
    let edges = get_edges(&mut reader);
    let graph = get_graph(&edges);
    let distance = bfs(&graph, "YOU".to_string(), "SAN".to_string()).unwrap();
    Ok(Answer::U(distance - 2))
}
