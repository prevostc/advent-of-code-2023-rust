use std::collections::HashMap;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        let to = to.split(" ").map(|s| s.to_string()).collect::<Vec<_>>();

        let edges = to
            .iter()
            .map(|s| (from.to_string(), s.to_string()))
            .chain(to.iter().map(|s| (s.to_string(), from.to_string())))
            .collect::<Vec<_>>();
        for (from, to) in edges {
            let n = nodes.entry(from).or_insert(vec![]);
            if !n.contains(&to.to_string()) {
                n.push(to.to_string());
            }
        }
    }
    //println!("{:?}", nodes);

    // remove those
    // xft -> pzv
    // dqf -> cbx
    // sds -> hbr
    let edges_to_remove = vec![
        ("xft".to_string(), "pzv".to_string()),
        ("dqf".to_string(), "cbx".to_string()),
        ("sds".to_string(), "hbr".to_string()),
    ];
    for (from, to) in edges_to_remove {
        nodes.get_mut(&from).unwrap().retain(|s| s != &to);
        nodes.get_mut(&to).unwrap().retain(|s| s != &from);
    }

    let mut res = 1;
    let mut clusters = vec!["xft".to_string(), "pzv".to_string()];
    //let mut clusters = vec!["bvb".to_string(), "cmg".to_string()];
    while let Some(cluster) = clusters.pop() {
        let mut seen = vec![];
        let mut queue = vec![cluster];
        while let Some(node) = queue.pop() {
            if seen.contains(&node) {
                continue;
            }
            seen.push(node.clone());
            if let Some(children) = nodes.get(&node) {
                queue.extend(children.clone());
            }
        }
        res *= seen.len();
    }

    Some(res as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, None);
    }
}
