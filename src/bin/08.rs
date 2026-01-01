use std::collections::{BTreeSet, HashSet};

advent_of_code::solution!(8);

type Vertex = (u64, u64, u64);
type Edge = (Vertex, Vertex, f64);
type Tree = BTreeSet<Vertex>;

fn distance(p1: &Vertex, p2: &Vertex) -> f64 {
    // Assuming p1 and p2 are (u64, u64, u64)
    let dx = (p2.0 as f64) - (p1.0 as f64);
    let dy = (p2.1 as f64) - (p1.1 as f64);
    let dz = (p2.2 as f64) - (p1.2 as f64);

    return (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
}

fn creates_circle(edge: &Edge, forest: &HashSet<BTreeSet<Vertex>>) -> bool {
    let (v_a, v_b, _) = edge;

    let t_a = forest.iter().find(|v| v.contains(v_a)).unwrap();
    let t_b = forest.iter().find(|v| v.contains(v_b)).unwrap();
    let is_equal = t_a == t_b;

    return is_equal;
}

pub fn part_one(input: &str) -> Option<u64> {
    return sol_part_one(1000, input);
}

pub fn sol_part_one(iterations: u32, input: &str) -> Option<u64> {
    let vertices: Vec<Vertex> = input
        .lines()
        .map(|line| {
            let coords: Vec<u64> = line.split(",").map(|num| num.parse().unwrap()).collect();
            return (coords[0], coords[1], coords[2]);
        })
        .collect();

    // TODO: As graph is undirected, we need to get rid of duplicates
    let mut edges: Vec<Edge> = vec![];

    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let p1 = vertices[i];
            let p2 = vertices[j];
            let d = (p1, p2, distance(&p1, &p2));
            edges.push(d);
        }
    }
    edges.sort_by(|p1, p2| p1.2.partial_cmp(&p2.2).unwrap());

    let mut forest: HashSet<BTreeSet<Vertex>> = vertices
        .into_iter() // Notice into_iter() to take ownership
        .map(|vertex| {
            let mut set = BTreeSet::new();
            set.insert(vertex);
            set
        })
        .collect();

    let mut connections_left = iterations;
    for edge in edges.iter() {
        if !creates_circle(edge, &forest) {
            let (v_a, v_b, _) = edge;

            let t_a = forest
                .iter()
                .find(|tree| tree.contains(v_a))
                .cloned()
                .unwrap();

            let t_b = forest
                .iter()
                .find(|tree| tree.contains(v_b))
                .cloned()
                .unwrap();

            let t_a_b: Tree = t_b.union(&t_a).map(|v| *v).collect();

            forest.remove(&t_a);
            forest.remove(&t_b);
            forest.insert(t_a_b);
        }
        connections_left -= 1;
        if connections_left == 0 {
            break;
        }
    }

    let mut size: Vec<usize> = forest.iter().map(|t| t.len()).collect();
    size.sort();

    let result = size.iter().rev().take(3).fold(1, |acc, x| acc * x);

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let vertices: Vec<Vertex> = input
        .lines()
        .map(|line| {
            let coords: Vec<u64> = line.split(",").map(|num| num.parse().unwrap()).collect();
            return (coords[0], coords[1], coords[2]);
        })
        .collect();

    // TODO: As graph is undirected, we need to get rid of duplicates
    let mut edges: Vec<Edge> = vec![];

    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let p1 = vertices[i];
            let p2 = vertices[j];
            let d = (p1, p2, distance(&p1, &p2));
            edges.push(d);
        }
    }
    edges.sort_by(|p1, p2| p1.2.partial_cmp(&p2.2).unwrap());

    let mut forest: HashSet<BTreeSet<Vertex>> = vertices
        .into_iter() // Notice into_iter() to take ownership
        .map(|vertex| {
            let mut set = BTreeSet::new();
            set.insert(vertex);
            set
        })
        .collect();

    let mut last_point: Option<&Edge> = None;

    for edge in edges.iter() {
        if !creates_circle(edge, &forest) {
            let (v_a, v_b, _) = edge;

            let t_a = forest
                .iter()
                .find(|tree| tree.contains(v_a))
                .cloned()
                .unwrap();

            let t_b = forest
                .iter()
                .find(|tree| tree.contains(v_b))
                .cloned()
                .unwrap();

            let t_a_b: Tree = t_b.union(&t_a).map(|v| *v).collect();

            forest.remove(&t_a);
            forest.remove(&t_b);
            forest.insert(t_a_b);
            last_point = Some(edge);
        }
    }

    let (a, b, _) = last_point.unwrap();

    Some(a.0 * b.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = sol_part_one(10, &advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
