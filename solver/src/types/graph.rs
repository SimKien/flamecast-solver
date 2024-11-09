use std::collections::HashMap;

pub type Vertex = usize;
pub type DirectedEdge = (Vertex, Vertex);

#[derive(Debug, Clone)]
pub struct DirectedGraph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<DirectedEdge>,
}

impl DirectedGraph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<DirectedEdge>) -> Self {
        Self { vertices, edges }
    }

    pub fn get_sources(&self) -> Vec<Vertex> {
        self.vertices
            .iter()
            .filter(|vertex| self.edges.iter().all(|(_, v2)| **vertex != *v2))
            .map(|vertex| *vertex)
            .collect()
    }

    pub fn get_drains(&self) -> Vec<Vertex> {
        self.vertices
            .iter()
            .filter(|vertex| self.edges.iter().all(|(v1, _)| **vertex != *v1))
            .map(|vertex| *vertex)
            .collect()
    }

    pub fn get_layers(&self) -> Vec<Vec<Vertex>> {
        // get the layers of the graph
        let mut layers = Vec::new();
        let mut current_layer = self.get_sources();
        let mut next_layer = Vec::new();

        while !current_layer.is_empty() {
            layers.push(current_layer.clone());

            for vertex in &current_layer {
                let outgoing_edge = self.outgoing_edge(*vertex);
                match outgoing_edge {
                    Some(edge) => {
                        next_layer.push(edge.1);
                    }
                    None => {
                        break;
                    }
                }
            }
            next_layer.sort();
            next_layer.dedup();

            current_layer = next_layer;
            next_layer = Vec::new();
        }

        return layers;
    }

    pub fn outgoing_edge(&self, vertex: Vertex) -> Option<DirectedEdge> {
        let possible_edge = self.edges.iter().find(|(v1, _)| *v1 == vertex);
        match possible_edge {
            Some(edge) => Some(*edge),
            None => None,
        }
    }

    pub fn calculate_edge_flows(&self) -> HashMap<DirectedEdge, usize> {
        // calculate the flows of the edges of the graph
        // assertions: only one outgoing edge from each vertex, layered, fully connected graph
        let mut edge_flows = HashMap::new();
        let mut vertex_flows = HashMap::new();

        let mut current_level = self.get_sources();
        let mut next_level = Vec::new();
        for source in &current_level {
            vertex_flows.insert(*source, 1);
        }

        while !current_level.is_empty() {
            for vertex in &current_level {
                let outgoing_edge = self.outgoing_edge(*vertex);
                match outgoing_edge {
                    Some(edge) => {
                        let flow = *vertex_flows.get(vertex).unwrap();

                        edge_flows.insert(edge, flow);
                        *vertex_flows.entry(edge.1).or_insert(0) += flow;

                        next_level.push(edge.1);
                    }
                    None => {
                        break;
                    }
                }
            }
            next_level.sort();
            next_level.dedup();

            current_level = next_level;
            next_level = Vec::new();
        }

        return edge_flows;
    }
}
