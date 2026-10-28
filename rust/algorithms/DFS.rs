fn dfs(graph: &Vec<Vec<usize>>, start: usize) {
    let mut visited = vec![false; graph.len()];
    let mut stack = Vec::new();
    
    visited[start] = true;
    stack.push(start);

    while let Some(node) = stack.pop() {
        println!("{}", node); 
        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                stack.push(neighbor);
            }
        }
    } 
}

fn main() {
    let graph = vec![
        vec![1, 2],
        vec![3, 4],
        vec![5],
        vec![],
        vec![],
        vec![],
    ];
    dfs(&graph, 0);
}