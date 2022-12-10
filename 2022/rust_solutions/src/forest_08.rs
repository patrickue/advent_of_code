use std::error;

pub fn calculate_scenic_score(x_pos: usize, y_pos: usize, forest: &mut Forest) -> usize {
    let tree_height = forest.rows[y_pos][x_pos].height;
    if is_on_edge(x_pos, y_pos, forest.width, forest.height) {
        return 0;
    }
    let mut view_left_idx = 0;
    let mut view_right_idx = 0;
    let mut view_top_idx = 0;
    let mut view_bottom_idx = 0;
    for x in (0..x_pos).rev() {
        view_left_idx += 1;
        if forest.rows[y_pos][x].height >= tree_height
        {
            break;
        }
    }
    for x in x_pos + 1..forest.width {
        view_right_idx += 1;
        if forest.rows[y_pos][x].height >= tree_height
        {
            break;
        }
    }
    for y in (0..y_pos).rev(){
        view_top_idx += 1;
        if forest.rows[y][x_pos].height >= tree_height
        {
            break;
        }
    }
    for y in y_pos + 1..forest.height {
        view_bottom_idx += 1;
        if forest.rows[y][x_pos].height >= tree_height
        {
            break;
        }
    }
    println!("{:?}", forest.rows[y_pos][x_pos]);
    println!("{:?}{:?}{:?}{:?}", view_left_idx, view_right_idx, view_top_idx, view_bottom_idx);
    return view_left_idx *
        view_right_idx *
        view_top_idx *
        view_bottom_idx;
}

pub fn parse_forest<'a>(terminal_output: Vec<String>) -> Result<Forest, Box<dyn error::Error>> {
    let mut forest = Forest::new();

    for line in terminal_output.into_iter()
    {
        let mut result_row: Result<Vec<usize>, std::num::ParseIntError> = line.chars().map(|p| p.to_string().parse::<usize>())
            .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
            //Map a possible ParseIntError onto Box Error
            .map_err(|e| e.into());
        let mut row = result_row.unwrap()
            .into_iter()
            .map(|h| Tree { height: h, scenic_score: 0 })
            .collect::<Vec<Tree>>();
        forest.rows.push(row);
    }
    forest.height = forest.rows.len();
    forest.width = forest.rows[0].len();
    return Ok(forest);
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Tree {
    height: usize,
    scenic_score: usize,
}

// A free standing private function.
pub fn make_tree_row() -> Vec<Vec<Tree>> { return Vec::new(); }

pub fn calculate_scenic_view(mut forest: Forest) -> usize {
    mark_trees_visible(&mut forest);
    println!("Forest after marking {:?} visible trees.", forest);
    let mut max_view_score = 0;
    for x in 0..forest.width {
        for y in 0..forest.height {
            let tree_scenic_score = forest.rows[y][x].scenic_score;
            if tree_scenic_score > max_view_score {
                max_view_score = tree_scenic_score;
            }
        }
    }
    return max_view_score;
}

fn mark_trees_visible(forest: &mut Forest) -> () {
    for x in 0..forest.width {
        for y in 0..forest.height {
            forest.rows[y][x].scenic_score = calculate_scenic_score(x, y, forest)
        }
    }
}

fn is_on_edge(x: usize, y: usize, max_x: usize, max_y: usize) -> bool {
    x == 0 ||
        y == 0 ||
        x == max_x - 1 ||
        y == max_y - 1
}

#[cfg(test)]
mod tests {
    use crate::forest_08::calculate_scenic_score;
    use crate::forest_08::Forest;
    use crate::forest_08::Tree;

    #[test]
    fn minimal() {
        let mut forest = Forest::new();
        forest.rows.push(vec![Tree { height: 0, scenic_score: 0 }, Tree { height: 0, scenic_score: 0 }, Tree { height: 0, scenic_score: 0 }]);
        forest.rows.push(vec![Tree { height: 0, scenic_score: 0 }, Tree { height: 1, scenic_score: 0 }, Tree { height: 0, scenic_score: 0 }]);
        forest.rows.push(vec![Tree { height: 0, scenic_score: 0 }, Tree { height: 0, scenic_score: 0 }, Tree { height: 0, scenic_score: 0 }]);
        forest.width = 3;
        forest.height = 3;
        assert_eq!(calculate_scenic_score(1, 1, &mut forest), 1);
    }
}

#[derive(Debug)]
pub struct Forest {
    rows: Vec<Vec<Tree>>,
    height: usize,
    width: usize,
}

impl Forest {
    pub fn new() -> Forest { Forest { rows: make_tree_row(), height: 0, width: 0 } }
}
