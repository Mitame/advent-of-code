use std::io::{Read, BufReader, BufRead};
use std::collections::HashMap;
use crate::Aoc;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Vec3(u64, u64, u64);

impl Vec3 {
    fn distance(&self, other: &Vec3) -> f64 {
        Vec3(other.0.abs_diff(self.0), other.1.abs_diff(self.1), other.2.abs_diff(self.2)).absolute()
    }

    fn absolute(&self) -> f64 {
        ((self.0.pow(2) + self.1.pow(2) + self.2.pow(2)) as f64).sqrt()
    }
}

fn parse(buf: &mut dyn Read) -> Vec<Vec3> {
    let buf_reader = BufReader::new(buf);
    buf_reader.lines().map(|line| {
        let line = line.unwrap();
        let values: Vec<u64> = line.split(",").map(|v| v.parse::<u64>().unwrap()).collect();
        Vec3(values[0], values[1], values[2])
    }).collect()
}

fn part1(buf: &mut dyn Read) {
    let junction_boxes = parse(buf);
    let mut parent_groups: Vec<_> = junction_boxes.iter().map(|junction_box| vec![junction_box]).collect();
    let mut group_link: HashMap<&Vec3, usize> = junction_boxes.iter().enumerate().map(|(i, junction_box)| (junction_box, i)).collect();
    
    // Get all distances
    let distances: HashMap<(&Vec3, &Vec3), f64> = junction_boxes.iter().enumerate()
        .flat_map(|(i, box_a)| junction_boxes[(i + 1)..].iter().map(move |box_b| (box_a, box_b)))
        .map(|(box_a, box_b)| ((box_a, box_b), box_a.distance(box_b)))
        .collect();

    let mut distances_by_length: Vec<((&Vec3, &Vec3), f64)> = distances.iter().map(|(a, b)| (*a, *b)).collect();
    distances_by_length.sort_by(|(_, distance_a), (_, distance_b)| distance_a.total_cmp(distance_b));

    dbg!(distances_by_length.len());

    // Connect shortest until we get to 1000
    for ((box_a, box_b), distance) in distances_by_length.iter().take(1000) {
        // If they're in the same group, do nothing
        let group_a = *group_link.get(box_a).unwrap();
        let group_b = *group_link.get(box_b).unwrap();
        if group_a == group_b {
            continue;
        } 
        
        // Link them
        let linked_group = std::mem::replace(&mut parent_groups[group_b], vec![]);
        let target_group = &mut parent_groups[group_a];
        for linked_box in &linked_group {
            group_link.insert(linked_box, group_a);
        }
        target_group.extend(linked_group);
    }
    let mut group_sizes: Vec<_> = parent_groups.iter().map(|group| group.len()).collect();
    group_sizes.sort();
    let result: usize = group_sizes.iter().rev().take(3).product();
    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let junction_boxes = parse(buf);
    let mut parent_groups: Vec<_> = junction_boxes.iter().map(|junction_box| vec![junction_box]).collect();
    let mut group_link: HashMap<&Vec3, usize> = junction_boxes.iter().enumerate().map(|(i, junction_box)| (junction_box, i)).collect();
    
    // Get all distances
    let distances: HashMap<(&Vec3, &Vec3), f64> = junction_boxes.iter().enumerate()
        .flat_map(|(i, box_a)| junction_boxes[(i + 1)..].iter().map(move |box_b| (box_a, box_b)))
        .map(|(box_a, box_b)| ((box_a, box_b), box_a.distance(box_b)))
        .collect();

    let mut distances_by_length: Vec<((&Vec3, &Vec3), f64)> = distances.iter().map(|(a, b)| (*a, *b)).collect();
    distances_by_length.sort_by(|(_, distance_a), (_, distance_b)| distance_a.total_cmp(distance_b));

    dbg!(distances_by_length.len());

    let mut last_con = None;
    // Connect shortest until we get to 1000
    for ((box_a, box_b), distance) in distances_by_length.iter() {
        // If they're in the same group, do nothing
        let group_a = *group_link.get(box_a).unwrap();
        let group_b = *group_link.get(box_b).unwrap();
        if group_a == group_b {
            continue;
        } 
        
        last_con = Some((box_a, box_b));
        // Link them
        let linked_group = std::mem::replace(&mut parent_groups[group_b], vec![]);
        let target_group = &mut parent_groups[group_a];
        for linked_box in &linked_group {
            group_link.insert(linked_box, group_a);
        }
        target_group.extend(linked_group);
    }
    
    let last_con = last_con.unwrap();

    println!("Part 2: {}", last_con.0.0 * last_con.1.0);
}

inventory::submit!(Aoc::new(
    2025,
    8,
    part1,
    part2,
));
