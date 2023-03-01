use rayon::prelude::*;
use std::iter::zip;
use std::ops::Index;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

// Heights are ints from [0, MAX_HEIGHT].
const MAX_HEIGHT: usize = 9;

#[derive(Debug, PartialEq)]
struct Grid<T> {
    data: Vec<T>,
    n_rows: usize,
    n_columns: usize,
}

#[derive(Debug)]
struct ColumnIterator<'a, T> {
    grid: &'a Grid<T>,
    front_row: usize,
    end_row: usize,
    column: usize,
}

#[derive(Debug)]
struct RowIterator<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    front_column: usize,
    end_column: usize,
}

struct ColumnsIterator<'a, T> {
    grid: &'a Grid<T>,
    column: usize,
}

struct RowsIterator<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, n_rows: usize, n_columns: usize) -> Grid<T> {
        assert_eq!(data.len(), n_rows * n_columns, "misshapen data.");
        Grid {
            data,
            n_rows,
            n_columns,
        }
    }

    pub fn rows(&self) -> RowsIterator<T> {
        RowsIterator { grid: self, row: 0 }
    }

    pub fn columns(&self) -> ColumnsIterator<T> {
        ColumnsIterator {
            grid: self,
            column: 0,
        }
    }

    pub fn row(&self, n_row: usize) -> RowIterator<T> {
        RowIterator {
            grid: self,
            row: n_row,
            front_column: 0,
            end_column: self.n_columns,
        }
    }

    pub fn column(&self, n_column: usize) -> ColumnIterator<T> {
        ColumnIterator {
            grid: self,
            front_row: 0,
            end_row: self.n_rows,
            column: n_column,
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        let idx = row * self.n_columns + column;
        &self.data[idx]
    }
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_row >= self.end_row {
            return None;
        }

        let row = self.front_row;
        self.front_row += 1;
        Some(&self.grid[(row, self.column)])
    }
}

impl<'a, T> DoubleEndedIterator for ColumnIterator<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        if self.end_row <= self.front_row {
            return None;
        }

        let row = self.end_row - 1;
        self.end_row -= 1;
        Some(&self.grid[(row, self.column)])
    }
}

impl<'a, T> DoubleEndedIterator for RowIterator<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        if self.end_column <= self.front_column {
            return None;
        }

        let column = self.end_column - 1;
        self.end_column -= 1;
        Some(&self.grid[(self.row, column)])
    }
}

impl<'a, T> Iterator for RowIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_column >= self.end_column {
            return None;
        }

        let column = self.front_column;
        self.front_column += 1;
        Some(&self.grid[(self.row, column)])
    }
}

impl<'a, T> Iterator for ColumnsIterator<'a, T> {
    type Item = ColumnIterator<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column == self.grid.n_columns {
            return None;
        }

        let column = self.column;
        self.column += 1;
        Some(self.grid.column(column))
    }
}

impl<'a, T> Iterator for RowsIterator<'a, T> {
    type Item = RowIterator<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.grid.n_rows {
            return None;
        }

        let row = self.row;
        self.row += 1;
        Some(self.grid.row(row))
    }
}

fn full_auxiliary_grid_part1(grid: &Grid<i8>, auxiliary_grid: &Grid<AtomicBool>) {
    // Belowe we parallel iterate in each of the 4 directions to build the auxiliary grid. We could
    // further parallelize by spawning each of these par iters in parallel. This caused a lot more
    // indentation and made it harder to read. I suspect there is not a ton to be gained by this
    // since as the array grows it can be further parallelized, but also we are likely to outstrip
    // the number of threads the OS can offer.

    // ParIter over each row forwards.
    zip(grid.rows(), auxiliary_grid.rows())
        .par_bridge()
        .for_each(|(grid_row, auxiliary_row)| {
            let mut max_height = -1;
            zip(grid_row, auxiliary_row).for_each(|(&height, visible)| {
                if height > max_height {
                    visible.store(true, Ordering::Release)
                }
                max_height = std::cmp::max(max_height, height);
            });
        });

    // ParIter over each row backwards.
    zip(grid.rows(), auxiliary_grid.rows())
        .par_bridge()
        .for_each(|(grid_row, auxiliary_row)| {
            let mut max_height = -1;
            zip(grid_row.rev(), auxiliary_row.rev()).for_each(|(&height, visible)| {
                if height > max_height {
                    visible.store(true, Ordering::Release)
                }
                max_height = std::cmp::max(max_height, height);
            });
        });

    // ParIter over each column.
    zip(grid.columns(), auxiliary_grid.columns())
        .par_bridge()
        .for_each(|(grid_column, auxiliary_column)| {
            let mut max_height = -1;
            zip(grid_column, auxiliary_column).for_each(|(&height, visible)| {
                if height > max_height {
                    visible.store(true, Ordering::Release)
                }
                max_height = std::cmp::max(max_height, height);
            });
        });

    // ParIter over each column backwards.
    zip(grid.columns(), auxiliary_grid.columns())
        .par_bridge()
        .for_each(|(grid_column, auxiliary_column)| {
            let mut max_height = -1;
            zip(grid_column.rev(), auxiliary_column.rev()).for_each(|(&height, visible)| {
                if height > max_height {
                    visible.store(true, Ordering::Release)
                }
                max_height = std::cmp::max(max_height, height);
            });
        });
}

fn count_visible_trees(auxiliary_grid: &Grid<AtomicBool>) -> (u32, u32) {
    auxiliary_grid
        .data
        .par_iter()
        .fold(
            || (0_u32, 0_u32),
            |(visible, invisible), is_visible| {
                if is_visible.load(Ordering::Acquire) {
                    (visible + 1, invisible)
                } else {
                    (visible, invisible + 1)
                }
            },
        )
        .reduce(|| (0_u32, 0_u32), |(v1, i1), (v2, i2)| (v1 + v2, i1 + i2))
}

fn apply_part2_line<'a, G, GItem, A, AItem>(grid_iter: G, auxiliary_iter: A, reverse: bool)
where
    G: Iterator<Item = GItem> + Send,
    GItem: Iterator<Item = &'a i8> + DoubleEndedIterator + Send,
    A: Iterator<Item = AItem> + Send,
    AItem: Iterator<Item = &'a AtomicI64> + DoubleEndedIterator + Send,
{
    zip(grid_iter, auxiliary_iter)
        .par_bridge()
        .for_each(|(grid_line, auxiliary_line)| {
            let (giter, aiter): (Box<dyn Iterator<Item = _>>, Box<dyn Iterator<Item = _>>) =
                if reverse {
                    (Box::new(grid_line.rev()), Box::new(auxiliary_line.rev()))
                } else {
                    (Box::new(grid_line), Box::new(auxiliary_line))
                };

            // i16 assumes the len of the row/column is under 2^15.
            let mut height_to_index = [0 as i16; MAX_HEIGHT + 1];
            zip(giter, aiter)
                .enumerate()
                .for_each(|(index, (&height, scenic_score))| {
                    let closest = *height_to_index[(height as usize)..].iter().max().unwrap();
                    let distance = index as i16 - closest;
                    let res =
                        scenic_score.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |score| {
                            Some(score * distance as i64)
                        });
                    assert!(res.is_ok());
                    height_to_index[height as usize] = index as i16;
                });
        });
}

fn fill_auxiliary_grid_part2(grid: &Grid<i8>, auxiliary_grid: &Grid<AtomicI64>) {
    // Probably easier to use rayon::scope & spawn, but rayon recommends par_iter instead :P.
    let counters: [Box<dyn Fn() -> () + Send + Sync>; 4] = [
        // ParIter over each row &  column both forwards and backwards.
        Box::new(|| apply_part2_line(grid.rows(), auxiliary_grid.rows(), false)),
        Box::new(|| apply_part2_line(grid.rows(), auxiliary_grid.rows(), true)),
        Box::new(|| apply_part2_line(grid.columns(), auxiliary_grid.columns(), false)),
        Box::new(|| apply_part2_line(grid.columns(), auxiliary_grid.columns(), true)),
    ];
    counters.par_iter().for_each(|func| func())
}

fn parse_row(src: &str, dst: &mut [i8]) {
    for (i, c) in src.chars().enumerate() {
        dst[i] = i8::from_str_radix(&c.to_string(), 10).unwrap();
    }
}

fn parse_input(input: &str) -> Grid<i8> {
    // Create the vec which will hold the data. This is done ahead of time to allow parsing in
    // parallel. By creating the vec ahead of time, this allows for only 1 reservation and avoids
    // the need to parse each row into its own vec which is then copied over into the main one.
    let mut lines = input.lines();
    let n_columns = lines.next().unwrap().len();
    let n_rows = lines.count() + 1;
    let mut data = vec![-1; n_columns * n_rows];

    let mut remainder = &mut data[..];
    rayon::scope(|s| {
        for line in input.lines() {
            let dst;
            (dst, remainder) = remainder.split_at_mut(n_columns);
            s.spawn(|_| parse_row(line, dst));
        }
    });
    Grid::new(data, n_rows, n_columns)
}

fn part1(input: &str) -> (u32, u32) {
    let grid = parse_input(input);

    let mut auxiliary_vec: Vec<AtomicBool> = Vec::new();
    auxiliary_vec.resize_with(grid.n_columns * grid.n_rows, AtomicBool::default);
    let auxiliary_grid = Grid::new(auxiliary_vec, grid.n_rows, grid.n_columns);
    full_auxiliary_grid_part1(&grid, &auxiliary_grid);

    count_visible_trees(&auxiliary_grid)
}

fn part2(input: &str) -> i64 {
    let grid = parse_input(input);

    let mut auxiliary_vec: Vec<AtomicI64> = Vec::new();
    auxiliary_vec.resize_with(grid.n_columns * grid.n_rows, || AtomicI64::new(1));
    let auxiliary_grid = Grid::new(auxiliary_vec, grid.n_rows, grid.n_columns);
    fill_auxiliary_grid_part2(&grid, &auxiliary_grid);

    auxiliary_grid
        .data
        .par_iter()
        .map(|a| a.load(Ordering::SeqCst))
        .max()
        .unwrap()
}

fn main() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(vec!["data", "day-08.txt"]);
    println!("{:?}", path);
    let txt = std::fs::read_to_string(path).unwrap();

    println!("8");
    println!("{:?}", part1(&txt));
    println!("{:?}", part2(&txt));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    #[test]
    fn test_grid() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(grid[(1, 2)], 6);
        assert_eq!(grid[(0, 0)], 1);
    }

    #[test]
    fn test_column() {
        let grid: Grid<i8> = Grid::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(grid.column(2).map(|i| *i).collect::<Vec<i8>>(), vec![3, 6]);
        assert_eq!(
            grid.column(2).rev().map(|i| *i).collect::<Vec<i8>>(),
            vec![6, 3]
        );
    }

    #[test]
    fn test_row() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(grid.row(0).map(|i| *i).collect::<Vec<i8>>(), vec![1, 2, 3]);
        assert_eq!(
            grid.row(0).rev().map(|i| *i).collect::<Vec<i8>>(),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn test_rows() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        for (a, b) in zip(grid.rows(), [[1, 2, 3], [4, 5, 6]]) {
            for (c, d) in zip(a, b) {
                assert_eq!(*c, d);
            }
        }
    }

    #[test]
    fn test_columns() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        for (a, b) in zip(grid.columns(), [[1, 4], [2, 5], [3, 6]]) {
            for (c, d) in zip(a, b) {
                assert_eq!(*c, d);
            }
        }
    }

    #[test]
    fn test_parse() {
        let grid = parse_input(
            "123
456",
        );
        assert_eq!(grid, Grid::new(vec![1, 2, 3, 4, 5, 6], 2, 3));
    }

    #[test]
    fn test_part1() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(part1(input), (21, 4))
    }

    #[test]
    fn test_part2() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(part2(input), 8)
    }
}
