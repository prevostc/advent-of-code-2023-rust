use std::collections::HashSet;

use rstar::RTree;
use rstar::{primitives::Rectangle, AABB};

advent_of_code::solution!(22);

type Point = [f32; 3];

#[inline]
fn parse_line(s: &str) -> Rectangle<Point> {
    let (a, b) = s.split_once("~").unwrap();
    let (an, bn) = (
        a.split(',')
            .map(|n| n.parse::<f32>().unwrap())
            .collect::<Vec<_>>(),
        b.split(',')
            .map(|n| n.parse::<f32>().unwrap())
            .collect::<Vec<_>>(),
    );

    Rectangle::from_corners(
        [
            min_f32(an[0], bn[0]),
            min_f32(an[1], bn[1]),
            min_f32(an[2], bn[2]),
        ],
        [
            max_f32(an[0], bn[0]) + 1.0,
            max_f32(an[1], bn[1]) + 1.0,
            max_f32(an[2], bn[2]) + 1.0,
        ],
    )
}

#[inline]
fn parse(input: &str) -> Vec<Rectangle<Point>> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect::<Vec<_>>()
}

#[inline]
fn envelope_search_down(rect: &Rectangle<Point>) -> AABB<Point> {
    // create an envelope 0.5 block down, 0.5 unit inside x,y wise
    let lower = rect.lower();
    let upper = rect.upper();
    AABB::from_corners(
        [lower[0] + 0.5, lower[1] + 0.5, lower[2] - 0.5],
        [upper[0] - 0.5, upper[1] - 0.5, lower[2] - 0.5],
    )
}

#[inline]
fn envelope_search_up(rect: &Rectangle<Point>) -> AABB<Point> {
    // create an envelope 0.5 block down, 0.5 unit inside x,y wise
    let lower = rect.lower();
    let upper = rect.upper();
    AABB::from_corners(
        [lower[0] + 0.5, lower[1] + 0.5, upper[2] + 0.5],
        [upper[0] - 0.5, upper[1] - 0.5, upper[2] + 0.5],
    )
}

#[inline]
fn has_hit_floor(rect: &Rectangle<Point>) -> bool {
    rect.lower()[2] == 1.0
}

#[inline]
fn can_fall_down_one(rect: &Rectangle<Point>, tree: &RTree<Rectangle<Point>>) -> bool {
    // check if we can fall down
    let envelope = envelope_search_down(rect);
    tree.locate_in_envelope_intersecting(&envelope).count() == 0
}

#[inline]
fn rect_fall_down_one(rect: &mut Rectangle<Point>) {
    *rect = Rectangle::from_corners(
        [rect.lower()[0], rect.lower()[1], rect.lower()[2] - 1.0],
        [rect.upper()[0], rect.upper()[1], rect.upper()[2] - 1.0],
    )
}

#[inline]
fn tree_fall_down(blocks: &mut Vec<Rectangle<Point>>, tree: &mut RTree<Rectangle<Point>>) -> u32 {
    // move one down until we hit the floor or another block
    let mut moved = true;
    let mut moved_blocks = HashSet::new();
    while moved {
        moved = false;
        for (id, block) in blocks.iter_mut().enumerate() {
            if has_hit_floor(&block) {
                continue;
            }

            if !can_fall_down_one(&block, &tree) {
                continue;
            }

            tree.remove(&block);
            rect_fall_down_one(block);
            tree.insert(*block);

            moved_blocks.insert(id);

            moved = true;
        }
    }
    moved_blocks.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    // add a special floor block
    let mut blocks = parse(input);
    let mut tree = RTree::bulk_load(blocks.clone());

    //println!("tree: {:?}", tree);
    tree_fall_down(&mut blocks, &mut tree);

    let mut valid_desintegrate = 0;
    for block in blocks.iter() {
        let blocks_resting_on = tree.locate_in_envelope_intersecting(&envelope_search_up(&block));

        let mut can_desintegrate = true;
        for resting_block in blocks_resting_on {
            let resting_block_resting_on =
                tree.locate_in_envelope_intersecting(&envelope_search_down(&resting_block));

            if resting_block_resting_on.count() == 1 {
                can_desintegrate = false;
                break;
            }
        }

        if can_desintegrate {
            valid_desintegrate += 1;
        }
    }

    Some(valid_desintegrate)
}

pub fn part_two(input: &str) -> Option<u32> {
    // add a special floor block
    let mut blocks = parse(input);
    let mut tree = RTree::bulk_load(blocks.clone());

    tree_fall_down(&mut blocks, &mut tree);

    let mut total_moving = 0;
    for block in blocks.iter() {
        let mut blocks_copy = blocks.clone();
        let mut tree_copy = tree.clone();

        blocks_copy.retain(|b| b != block);
        tree_copy.remove(block);

        total_moving += tree_fall_down(&mut blocks_copy, &mut tree_copy);
    }

    Some(total_moving)
}

#[inline]
fn min_f32(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
#[inline]
fn max_f32(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_envelope_lookup_down() {
        let input = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
        ";
        let blocks = parse(input);
        let (a, b, c, d, e, f, g) = (
            blocks[0].clone(),
            blocks[1].clone(),
            blocks[2].clone(),
            blocks[3].clone(),
            blocks[4].clone(),
            blocks[5].clone(),
            blocks[6].clone(),
        );
        let tree = RTree::bulk_load(blocks);

        let test_cases = vec![
            (g, vec![]),
            (f, vec![&e]),
            (e, vec![]),
            (d, vec![&c]),
            (c, vec![]),
            (b, vec![&a]),
            (a, vec![]),
        ];
        for (test, expected) in test_cases {
            let intersecting = tree
                .locate_in_envelope_intersecting(&envelope_search_down(&test))
                .collect::<Vec<_>>();
            assert_eq!(intersecting, expected);
        }
    }

    #[test]
    fn test_tree_fall_down() {
        let input = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
    ";
        let mut blocks = parse(input);
        let mut tree = RTree::bulk_load(blocks.clone());

        tree_fall_down(&mut blocks, &mut tree);

        let (a, b, c, d, e, f, g) = (
            blocks[0].clone(),
            blocks[1].clone(),
            blocks[2].clone(),
            blocks[3].clone(),
            blocks[4].clone(),
            blocks[5].clone(),
            blocks[6].clone(),
        );

        assert_eq!((a.lower(), a.upper()), ([1.0, 0.0, 1.0], [2.0, 3.0, 2.0]));
        assert_eq!((b.lower(), b.upper()), ([0.0, 0.0, 2.0], [3.0, 1.0, 3.0]));
        assert_eq!((c.lower(), c.upper()), ([0.0, 2.0, 2.0], [3.0, 3.0, 3.0]));
        assert_eq!((d.lower(), d.upper()), ([0.0, 0.0, 3.0], [1.0, 3.0, 4.0]));
        assert_eq!((e.lower(), e.upper()), ([2.0, 0.0, 3.0], [3.0, 3.0, 4.0]));
        assert_eq!((f.lower(), f.upper()), ([0.0, 1.0, 4.0], [3.0, 2.0, 5.0]));
        assert_eq!((g.lower(), g.upper()), ([1.0, 1.0, 5.0], [2.0, 2.0, 7.0]));
    }
}
