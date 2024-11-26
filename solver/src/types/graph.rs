use std::collections::HashMap;

pub type Vertex = usize;
pub type DirectedEdge = (Vertex, Vertex);

#[derive(Debug, Clone)]
pub struct Layer {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<DirectedEdge>,
}

impl Layer {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<DirectedEdge>) -> Self {
        Self { vertices, edges }
    }
}

#[derive(Debug, Clone)]
pub struct LayeredGraph {
    pub layers: Vec<Layer>,
    pub next_vertex: Vertex,
    pub removed_vertices: Vec<Vertex>,
}

impl LayeredGraph {
    pub fn new(layers: Vec<Layer>, next_vertex: Vertex, removed_vertices: Vec<Vertex>) -> Self {
        Self {
            layers,
            next_vertex,
            removed_vertices,
        }
    }

    pub fn cumulate_vertices(&self) -> Vec<Vertex> {
        // cumulate the vertices of the graph
        let mut vertices = Vec::new();

        for layer in &self.layers {
            vertices.extend(layer.vertices.clone());
        }

        return vertices;
    }

    pub fn cumulate_edges(&self) -> Vec<DirectedEdge> {
        // cumulate the edges of the graph
        let mut edges = Vec::new();

        for layer in &self.layers {
            edges.extend(layer.edges.clone());
        }

        return edges;
    }

    pub fn get_sources(&self) -> Vec<Vertex> {
        self.layers[0]
            .vertices
            .iter()
            .map(|vertex| *vertex)
            .collect()
    }

    pub fn get_drains(&self) -> Vec<Vertex> {
        self.layers[self.layers.len() - 1]
            .vertices
            .iter()
            .map(|vertex| *vertex)
            .collect()
    }

    pub fn get_vertex_layers(&self) -> Vec<Vec<Vertex>> {
        // get the layers of the graph
        let mut layers = Vec::new();

        for layer in &self.layers {
            layers.push(layer.vertices.clone());
        }

        return layers;
    }

    pub fn calculate_edge_flows(&self) -> HashMap<DirectedEdge, usize> {
        // calculate the flows of the edges of the graph
        let mut edge_flows = HashMap::new();
        let mut vertex_flows = HashMap::new();

        self.layers[0].vertices.iter().for_each(|vertex| {
            vertex_flows.insert(*vertex, 1);
        });

        for layer in &self.layers {
            layer.edges.iter().for_each(|(source, target)| {
                let flow = *vertex_flows.get(source).unwrap();
                edge_flows.insert((*source, *target), flow);
                *vertex_flows.entry(*target).or_insert(0) += flow;
            });
        }

        return edge_flows;
    }
}
