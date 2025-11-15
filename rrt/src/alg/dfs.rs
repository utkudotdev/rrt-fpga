pub fn dfs(
    tree: &Vec<Vec<usize>>,
    start_idx: usize,
    end_idx: usize,
    path: &mut Vec<usize>,
) -> bool {
    path.push(start_idx);
    if start_idx == end_idx {
        return true;
    }
    if let Some(children) = tree.get(start_idx) {
        for &child_idx in children {
            if dfs(tree, child_idx, end_idx, path) {
                return true;
            }
        }
    }
    path.pop();
    false
}
