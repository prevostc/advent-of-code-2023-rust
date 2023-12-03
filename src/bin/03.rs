use grid::*;
advent_of_code::solution!(3);

#[derive(Copy, Clone, Debug)]
enum Element {
    Digit(u32),
    Symbol(char),
    Void,
}

fn parse_input(input: &str) -> Grid<Element> {
    let line_len = input.lines().next().unwrap().len();
    let elems = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| {
            if c == '.' {
                Element::Void
            } else if c.is_digit(10) {
                Element::Digit(c.to_digit(10).unwrap().try_into().unwrap())
            } else {
                Element::Symbol(c)
            }
        })
        .collect();
    Grid::from_vec(elems, line_len)
}

fn adjacent<T: Copy>(grid: &Grid<T>, row: usize, col: usize) -> impl Iterator<Item = T> + '_ {
    let int_row: i32 = row.try_into().unwrap();
    let int_col: i32 = col.try_into().unwrap();

    let res = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        //(0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .map(move |(l, c)| (int_row + l, int_col + c))
    .filter(|(l, c)| *l >= 0 && *c >= 0)
    .map(|(l, c)| grid.get(l.try_into().unwrap(), c.try_into().unwrap()))
    .filter(|o| o.is_some())
    .map(|o| o.unwrap())
    .map(|o| *o);

    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let elems = parse_input(input);

    let mut c;
    let mut r = 0;

    let mut sum = 0;
    while r < elems.rows() {
        c = 0;
        while c < elems.cols() {
            match elems.get(r, c) {
                Some(Element::Digit(_)) => {
                    let mut n = 0;
                    let mut do_include = false;
                    while c < elems.cols() {
                        match elems.get(r, c) {
                            Some(Element::Digit(d)) => n = n * 10 + *d,
                            _ => break,
                        }
                        do_include = do_include
                            || adjacent(&elems, r, c).any(|e| match e {
                                Element::Symbol(_) => true,
                                _ => false,
                            });
                        c += 1;
                    }

                    if do_include {
                        sum += n;
                    }
                }
                _ => (),
            }
            c += 1;
        }
        r += 1;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elems = parse_input(input);

    let mut c;
    let mut r = 0;

    let mut sum = 0;
    while r < elems.rows() {
        c = 0;
        while c < elems.cols() {
            match elems.get(r, c) {
                Some(Element::Symbol('*')) => {
                    // detect adj numbers
                    let mut maybe_num_coord = vec![(r, c - 1), (r, c + 1)];
                    if let Some(Element::Digit(_)) = elems.get(r - 1, c) {
                        maybe_num_coord.push((r - 1, c));
                    } else {
                        maybe_num_coord.push((r - 1, c - 1));
                        maybe_num_coord.push((r - 1, c + 1));
                    }

                    if let Some(Element::Digit(_)) = elems.get(r + 1, c) {
                        maybe_num_coord.push((r + 1, c));
                    } else {
                        maybe_num_coord.push((r + 1, c - 1));
                        maybe_num_coord.push((r + 1, c + 1));
                    }

                    let adj_nums = maybe_num_coord
                        .into_iter()
                        .map(|(l, c)| ((l, c), elems.get(l, c)))
                        .filter(|(_, n)| n.is_some())
                        .map(|(co, n)| (co, *n.unwrap()))
                        .filter(|(_, e)| matches!(e, Element::Digit(_)))
                        .map(|(co, _)| co)
                        .collect::<Vec<_>>();

                    println!("({},{}): {}", r, c, adj_nums.len());

                    if adj_nums.len() == 2 {
                        let mut prod = 1;

                        for (n_l, n_c) in adj_nums {
                            let mut d_c = n_c;
                            // rewind
                            while d_c > 0
                                && d_c < elems.cols()
                                && elems
                                    .get(n_l, d_c - 1)
                                    .is_some_and(|e| matches!(e, Element::Digit(_)))
                            {
                                d_c -= 1;
                            }
                            // consume
                            let mut n = 0;
                            while d_c < elems.cols() {
                                match elems.get(n_l, d_c) {
                                    Some(Element::Digit(d)) => n = n * 10 + *d,
                                    _ => break,
                                }
                                d_c += 1;
                            }
                            prod *= n;
                        }

                        sum += prod;
                    }
                }
                _ => (),
            }
            c += 1;
        }
        r += 1;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_one_non_reg() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, 2));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, 1));
        assert_eq!(result, Some(467835));
    }
}
