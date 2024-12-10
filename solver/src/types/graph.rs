use std::collections::HashMap;

pub type Vertex = usize;
pub type DirectedEdge = (Vertex, Vertex);

#[derive(Debug, Clone)]
pub struct Layer {
    pub vertices: Vec<Vertex>,
    pub edges: HashMap<Vertex, DirectedEdge>, // Each Vertex only has one edge
}

impl Layer {
    pub fn new(vertices: Vec<Vertex>, edges: HashMap<Vertex, DirectedEdge>) -> Self {
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

    pub fn to_string(&self) -> String {
        // convert the graph to a string
        let mut graph_string = String::new();
        for layer in &self.layers {
            graph_string = format!("{}\n{:?}", graph_string, layer.vertices);
            graph_string = format!("{}\n{:?}", graph_string, layer.edges);
        }
        return graph_string;
    }

    pub fn new_vertex(&mut self) -> Vertex {
        // create a new vertex
        if !self.removed_vertices.is_empty() {
            let vertex = self.removed_vertices.pop().unwrap();
            return vertex;
        }

        let vertex = self.next_vertex;
        self.next_vertex += 1;
        return vertex;
    }

    pub fn add_vertex_to_layer(&mut self, vertex: Vertex, layer_index: usize) {
        // add a vertex to a layer
        self.layers[layer_index].vertices.push(vertex);
    }

    pub fn remove_vertex_from_layer(&mut self, vertex: Vertex, layer_index: usize) {
        // remove a vertex from a layer
        let vertex_position = self.layers[layer_index]
            .vertices
            .iter()
            .position(|v| *v == vertex)
            .unwrap();
        self.layers[layer_index].vertices.remove(vertex_position);
    }

    pub fn add_edge_to_layer(&mut self, edge: DirectedEdge, layer_index: usize) {
        // add an edge to a layer
        self.layers[layer_index].edges.insert(edge.0, edge);
    }

    pub fn remove_edge_from_layer(&mut self, edge: DirectedEdge, layer_index: usize) {
        // remove an edge from a layer
        self.layers[layer_index].edges.remove(&edge.0);
    }

    pub fn get_parent(&self, layer_index: usize, vertex: Vertex) -> Vertex {
        // get the parent of a vertex
        self.layers[layer_index].edges.get(&vertex).unwrap().1
    }

    pub fn get_children(&self, layer_index: usize, vertex: Vertex) -> Vec<Vertex> {
        // get the children of a vertex
        let mut children = Vec::new();

        for edge in self.layers[layer_index].edges.values() {
            if edge.1 == vertex {
                children.push(edge.0);
            }
        }

        return children;
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
            if layer.edges.is_empty() {
                break;
            }
            edges.extend(
                layer
                    .vertices
                    .iter()
                    .map(|vertex| layer.edges.get(vertex).unwrap()),
            );
        }

        return edges;
    }

    pub fn get_sources(&self) -> Vec<Vertex> {
        // get the sources of the graph
        self.layers[0]
            .vertices
            .iter()
            .map(|vertex| *vertex)
            .collect()
    }

    pub fn get_drains(&self) -> Vec<Vertex> {
        // get the drains of the graph
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

    pub fn get_neighbours(&self, vertex: Vertex) -> Vec<Vertex> {
        // get the neighbours of a vertex
        let mut neighbours = Vec::new();

        let vertex_layer = self
            .layers
            .iter()
            .position(|layer| layer.vertices.contains(&vertex))
            .unwrap();

        neighbours.append(&mut self.get_children(vertex_layer - 1, vertex));
        neighbours.push(self.get_parent(vertex_layer, vertex));

        return neighbours;
    }

    pub fn calculate_edge_flows(&self) -> HashMap<DirectedEdge, usize> {
        // calculate the flows of the edges of the graph
        let mut edge_flows = HashMap::new();
        let mut vertex_flows = HashMap::new();

        self.layers[0].vertices.iter().for_each(|vertex| {
            vertex_flows.insert(*vertex, 1);
        });

        for layer in &self.layers {
            layer.edges.values().for_each(|(source, target)| {
                let flow = *vertex_flows.get(source).unwrap();
                edge_flows.insert((*source, *target), flow);
                *vertex_flows.entry(*target).or_insert(0) += flow;
            });
        }

        return edge_flows;
    }
}
