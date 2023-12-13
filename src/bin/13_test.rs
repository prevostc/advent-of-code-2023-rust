use super::*;

#[test]
fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
    assert_eq!(result, Some(400));
}

#[test]
fn test_best_mirror_position_1() {
    let s = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
    let lake = Lake::new(s);
    assert_eq!(detect_best_mirror_position(&lake.cols, 1), None);
    assert_eq!(detect_best_mirror_position(&lake.rows, 1), Some((3, 6)));
}

#[test]
fn test_part_one_1() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
    assert_eq!(result, Some(405));
}

#[test]
fn test_part_one_2() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY, 2));
    assert_eq!(result, Some(810));
}

#[test]
fn test_solve_1_0() {
    let s = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
    let lake = Lake::new(s);

    let (horizontal, vertical) = lake.solve(0);
    assert_eq!(horizontal, Some(5));
    assert_eq!(vertical, None);
}

#[test]
fn test_solve_2_0() {
    let s = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
    let lake = Lake::new(s);

    let (horizontal, vertical) = lake.solve(0);
    assert_eq!(horizontal, None);
    assert_eq!(vertical, Some(4));
}

#[test]
fn test_best_mirror_position_0() {
    let s = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
    let lake = Lake::new(s);
    assert_eq!(detect_best_mirror_position(&lake.cols, 0), Some((5, 8)));
    assert_eq!(detect_best_mirror_position(&lake.rows, 0), None);
}

#[test]
fn test_solve_for_first_column() {
    let s = "..###.#..#.##\n#####.####.##\n...##.#..#.##\n...##.####.##\n###.#......#.\n###..######..\n###..........\n..##..##.#..#\n...###.##.###";
    let lake = Lake::new(s);
    println!("{}", lake);
    assert_eq!(detect_best_mirror_position(&lake.cols, 0), Some((1, 2)));
}

#[test]
fn test_only_detect_full_reflections_up_to_a_border() {
    let s = "#.#.##..##.#.\n....#.##.#.#.\n####......#.#\n####......#.#\n....#.##.#.#.\n....##..##.#.\n#.#.#.##.#.#.";
    let lake = Lake::new(s);
    assert_eq!(detect_best_mirror_position(&lake.cols, 0), None);
    assert_eq!(detect_best_mirror_position(&lake.rows, 0), None);
}

#[test]
fn test_detect_regression() {
    let s = ".###..###\n.##....##\n#.#....#.\n.########\n##.#..#.#\n##......#\n##..###.#\n#.#.##.#.\n.#......#\n..#....#.\n####..###\n...#..#..\n...#..#..\n####..###\n..#....#.";
    let lake = Lake::new(s);
    assert_eq!(detect_best_mirror_position(&lake.rows, 0), Some((12, 6)));
}

#[test]
fn test_lake_new() {
    let s = "..#\n#..\n...";
    let lake = Lake::new(s);
    assert_eq!(lake.rows.len(), 3);
    assert_eq!(lake.rows.len(), 3);
    println!("{}", lake);
}
