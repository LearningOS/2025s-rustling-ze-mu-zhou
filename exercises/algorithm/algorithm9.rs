use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "访问了不在图中的节点")
    }
}

pub struct UndirectedGraph<N, W> {
    adjacency_table: HashMap<N, Vec<(N, W)>>,
}

impl<N, W> Graph<N, W> for UndirectedGraph<N, W>
where
    N: Eq + std::hash::Hash + Clone,
    W: Clone,
{
    fn new() -> Self {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<N, Vec<(N, W)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<N, Vec<(N, W)>> {
        &self.adjacency_table
    }
}

pub trait Graph<N, W>
where
    N: Eq + std::hash::Hash + Clone,
    W: Clone,
{
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<N, Vec<(N, W)>>;
    fn adjacency_table(&self) -> &HashMap<N, Vec<(N, W)>>;

    fn add_node(&mut self, node: N) -> bool {
        let adjacency_table = self.adjacency_table_mutable();
        if !adjacency_table.contains_key(&node) {
            adjacency_table.insert(node, Vec::new());
            true
        } else {
            false
        }
    }

    fn add_edge(&mut self, edge: (N, N, W)) {
        let (node1, node2, weight) = edge;
        self.add_node(node1.clone());
        self.add_node(node2.clone());
        let adjacency_table = self.adjacency_table_mutable();
        adjacency_table
            .entry(node1.clone()) // 使用 clone 避免移动 node1
            .or_default()
            .push((node2.clone(), weight.clone()));
        adjacency_table
            .entry(node2.clone()) // 使用 clone 避免移动 node2
            .or_default()
            .push((node1, weight)); // 这里 node1 未被移动，可以直接使用
    }

    fn contains(&self, node: &N) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes<'a>(&'a self) -> HashSet<&'a N> // 添加生命周期 'a
    where
        W: 'a, // 添加 W 的生命周期约束
    {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&N, &N, &W)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::{Graph, UndirectedGraph};

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&"a", &"b", &5),
            (&"b", &"a", &5),
            (&"c", &"a", &7),
            (&"a", &"c", &7),
            (&"b", &"c", &10),
            (&"c", &"b", &10),
        ];

        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}