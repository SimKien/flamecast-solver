use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VertexID {
    pub layer: usize,
    pub index: usize,
}

impl VertexID {
    pub fn new(layer: usize, index: usize) -> Self {
        Self { layer, index }
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.layer, self.index)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub parent_index: Option<usize>,
    pub children_indices: Option<Vec<usize>>,
}

impl Vertex {
    pub fn new(parent_index: Option<usize>, children_indices: Option<Vec<usize>>) -> Self {
        Self {
            parent_index,
            children_indices,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            parent_index: None,
            children_indices: None,
        }
    }

    pub fn set_parent(&mut self, parent_index: Option<usize>) {
        self.parent_index = parent_index;
    }

    pub fn remove_parent(&mut self) {
        self.parent_index = None;
    }

    pub fn set_children(&mut self, children_indices: Option<Vec<usize>>) {
        self.children_indices = children_indices;
    }

    pub fn add_child(&mut self, child_index: usize) {
        self.children_indices
            .get_or_insert_with(|| Vec::new())
            .push(child_index);
    }

    pub fn append_children(&mut self, new_children: &mut Vec<usize>) {
        self.children_indices
            .get_or_insert_with(|| Vec::new())
            .append(new_children);
    }

    pub fn truncate_children(&mut self, children_len: usize) {
        if let Some(children) = &mut self.children_indices {
            children.truncate(children_len);
        }
    }

    pub fn remove_child(&mut self, child_index: usize) {
        if let Some(children_indices) = &mut self.children_indices {
            children_indices.swap_remove(
                children_indices
                    .iter()
                    .position(|x| *x == child_index)
                    .unwrap(),
            );
            if children_indices.is_empty() {
                self.children_indices = None;
            }
        }
    }

    pub fn change_child(&mut self, old_index: usize, new_index: usize) {
        if let Some(children_indices) = &mut self.children_indices {
            let index = children_indices
                .iter()
                .position(|x| *x == old_index)
                .unwrap();
            children_indices[index] = new_index;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub vertices: Vec<Vertex>,
}

impl Layer {
    pub fn new_with_size(size: usize) -> Self {
        let vertices = vec![Vertex::new_empty(); size];
        Self { vertices }
    }

    pub fn new_empty() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn from(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        // add a vertex to the layer and returns the index of the vertex
        let index = self.vertices.len();
        self.vertices.push(vertex);
        return index;
    }

    pub fn remove_vertex(&mut self, vertex: &VertexID) -> usize {
        // remove a vertex from the layer and returns the the old index of the vertex which was swapped with the removed vertex
        self.vertices.swap_remove(vertex.index);

        return self.vertices.len();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayeredGraph {
    pub layers: Vec<Layer>,
}

impl LayeredGraph {
    pub fn new_empty() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn new_with_size(num_layers: usize) -> Self {
        Self {
            layers: vec![Layer::new_empty(); num_layers],
        }
    }

    pub fn from(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn from_sources_drains(
        sources: Vec<Vertex>,
        drains: Vec<Vertex>,
        num_layers: usize,
    ) -> Self {
        // create an almost empty graph with only sources and drains
        let mut graph = LayeredGraph::new_empty();

        graph.add_layer(Layer::from(sources));

        for _ in 1..num_layers - 1 {
            graph.add_layer(Layer::new_empty());
        }

        graph.add_layer(Layer::from(drains));

        return graph;
    }

    pub fn add_layer(&mut self, layer: Layer) {
        // add a layer to the graph
        self.layers.push(layer);
    }

    pub fn to_string(&self) -> String {
        // convert the graph to a string
        let mut graph_string = String::new();
        for layer in &self.layers {
            graph_string = format!("{}\n{:?}", graph_string, layer);
        }
        return graph_string;
    }

    pub fn add_vertex_to_layer(&mut self, layer_index: usize, vertex: Vertex) -> VertexID {
        // add a vertex to a layer
        let (first, last) = self.layers.split_at_mut(layer_index);

        let vertex_index = last[0].add_vertex(vertex);
        let vertex = &last[0].vertices[vertex_index];

        if let Some(children_indices) = &vertex.children_indices {
            let children_layer = &mut first[layer_index - 1];
            children_indices.iter().for_each(|child_index| {
                let child = &mut children_layer.vertices[*child_index];
                child.set_parent(Some(vertex_index));
            });
        }

        if let Some(parent_index) = vertex.parent_index {
            let parent = &mut last[1].vertices[parent_index];
            parent.add_child(vertex_index);
        }

        return VertexID::new(layer_index, vertex_index);
    }

    pub fn add_vertex_at_position(&mut self, vertex: Vertex, vertex_id: &VertexID) {
        // add a vertex at a specific position of the graph, used for merge and split operations
        let layer_index = vertex_id.layer;

        self.add_vertex_to_layer(layer_index, vertex);
        self.swap_vertices_position(
            vertex_id,
            &VertexID::new(layer_index, self.layers[layer_index].vertices.len() - 1),
        );
    }

    pub fn set_vertex_at_position(&mut self, vertex_id: &VertexID, vertex: Vertex) {
        // set a vertex at a specific position of the graph, assuming no vertex exists at that position
        let (first, last) = self.layers.split_at_mut(vertex_id.layer);

        if let Some(children_indices) = &vertex.children_indices {
            let children_layer = &mut first[vertex_id.layer - 1];
            children_indices.iter().for_each(|child_index| {
                let child = &mut children_layer.vertices[*child_index];
                child.set_parent(Some(vertex_id.index));
            });
        }

        if let Some(parent_index) = vertex.parent_index {
            let parent = &mut last[1].vertices[parent_index];
            parent.add_child(vertex_id.index);
        }

        self.layers[vertex_id.layer].vertices[vertex_id.index] = vertex;
    }

    pub fn remove_vertex(&mut self, vertex: &VertexID) {
        // remove a vertex from the graph
        if vertex.index == self.layers[vertex.layer].vertices.len() {
            return;
        }

        let last_vertex_id =
            VertexID::new(vertex.layer, self.layers[vertex.layer].vertices.len() - 1);

        self.swap_vertices_position(vertex, &last_vertex_id);
        self.pop_vertex(vertex.layer);
    }

    pub fn pop_vertex(&mut self, vertex_layer: usize) -> Option<Vertex> {
        // pops the last vertex of a layer from the graph
        return self.layers[vertex_layer].vertices.pop();
    }

    pub fn swap_vertices_position(&mut self, vertex1: &VertexID, vertex2: &VertexID) {
        // swap the position of two vertices in the same layer of the graph
        if vertex1 == vertex2 {
            return;
        }

        let vertex1_parent = self.get_parent(vertex1);
        let vertex2_parent = self.get_parent(vertex2);

        let vertex1_children = self.get_children(vertex1);
        let vertex2_children = self.get_children(vertex2);

        if let Some(vertex1_children) = vertex1_children {
            vertex1_children.iter().for_each(|child| {
                self.get_vertex_mut(child).set_parent(Some(vertex2.index));
            });
        }

        if let Some(vertex2_children) = vertex2_children {
            vertex2_children.iter().for_each(|child| {
                self.get_vertex_mut(child).set_parent(Some(vertex1.index));
            });
        }

        if let Some(vertex1_parent) = vertex1_parent {
            self.get_vertex_mut(&vertex1_parent)
                .change_child(vertex1.index, vertex2.index);
        }

        if let Some(vertex2_parent) = vertex2_parent {
            self.get_vertex_mut(&vertex2_parent)
                .change_child(vertex2.index, vertex1.index);
        }

        self.layers[vertex1.layer]
            .vertices
            .swap(vertex1.index, vertex2.index);
    }

    pub fn add_edge(&mut self, source: &VertexID, target: &VertexID) {
        // add an edge to the graph
        let source_vertex = self.get_vertex_mut(source);
        source_vertex.set_parent(Some(target.index));
        let target_vertex = self.get_vertex_mut(target);
        target_vertex.add_child(source.index);
    }

    pub fn remove_edge(&mut self, source: &VertexID) {
        // remove an edge from the graph
        let target = self.get_parent(source).unwrap();
        let source_vertex = self.get_vertex_mut(source);
        source_vertex.remove_parent();
        let target_vertex = self.get_vertex_mut(&target);
        target_vertex.remove_child(source.index);
    }

    pub fn get_number_of_vertices(&self) -> usize {
        // get the number of vertices of the graph
        return self
            .layers
            .iter()
            .fold(0, |acc, layer| acc + layer.vertices.len());
    }

    pub fn get_number_of_edges(&self) -> usize {
        // get the number of edges of the graph
        let number_of_layers = self.layers.len();

        return self
            .layers
            .iter()
            .enumerate()
            .fold(0, |acc, (layer_index, layer)| {
                if layer_index == number_of_layers - 1 {
                    return acc;
                }
                return acc + layer.vertices.len();
            });
    }

    pub fn get_vertex(&self, vertex: &VertexID) -> &Vertex {
        // get a vertex from the graph
        &self.layers[vertex.layer].vertices[vertex.index]
    }

    pub fn get_vertex_mut(&mut self, vertex: &VertexID) -> &mut Vertex {
        // get a mutable vertex from the graph
        &mut self.layers[vertex.layer].vertices[vertex.index]
    }

    pub fn get_parent(&self, vertex: &VertexID) -> Option<VertexID> {
        // get the parent of a vertex
        let regarded_vertex = self.get_vertex(vertex);

        return match regarded_vertex.parent_index {
            Some(parent_index) => Some(VertexID::new(vertex.layer + 1, parent_index)),
            None => None,
        };
    }

    pub fn get_children(&self, vertex: &VertexID) -> Option<Vec<VertexID>> {
        // get the children of a vertex
        let regarded_vertex = self.get_vertex(vertex);

        return match &regarded_vertex.children_indices {
            Some(children_indices) => Some(
                children_indices
                    .iter()
                    .map(|child_index| VertexID::new(vertex.layer - 1, *child_index))
                    .collect(),
            ),
            None => None,
        };
    }

    pub fn get_sources_indexes(&self) -> Vec<VertexID> {
        // get the sources of the graph
        self.layers[0]
            .vertices
            .iter()
            .enumerate()
            .map(|(index, _)| VertexID::new(0, index))
            .collect()
    }

    pub fn get_drains_indexes(&self) -> Vec<VertexID> {
        // get the drains of the graph
        self.layers[self.layers.len() - 1]
            .vertices
            .iter()
            .enumerate()
            .map(|(index, _)| VertexID::new(self.layers.len() - 1, index))
            .collect()
    }

    pub fn get_vertex_layers_with_indexes(&self) -> Vec<Vec<VertexID>> {
        // get the layers of the graph
        return self
            .layers
            .iter()
            .enumerate()
            .map(|(layer_index, layer)| {
                layer
                    .vertices
                    .iter()
                    .enumerate()
                    .map(|(vertex_index, _)| VertexID::new(layer_index, vertex_index))
                    .collect()
            })
            .collect();
    }

    pub fn get_layer_structure(&self) -> Vec<usize> {
        // get the structure of the layers
        self.layers
            .iter()
            .map(|layer| layer.vertices.len())
            .collect()
    }

    pub fn get_neighbours(&self, vertex: &VertexID) -> Vec<VertexID> {
        // get the neighbours of a vertex
        let mut neighbours = Vec::new();

        if let Some(parent) = self.get_parent(vertex) {
            neighbours.push(parent);
        }
        if let Some(children) = self.get_children(vertex) {
            neighbours.append(&mut children.clone());
        }

        return neighbours;
    }

    pub fn get_sorted_random_vertices(&self, amount: usize) -> Vec<VertexID> {
        // returns rendom vertices which are sorted by layer_index and index in a layer
        let mut rng = rand::thread_rng();

        // dont pick vertices of the last layer because they cant have neighbors
        let max_amount_regarded_vertices =
            self.get_number_of_vertices() - self.layers[self.layers.len() - 1].vertices.len();

        let mut indices: Vec<usize> = (0..max_amount_regarded_vertices).collect();
        indices.shuffle(&mut rng);

        indices.truncate(amount);
        indices.sort();

        let mut current_index = 0;
        let mut layer_index = 0;
        let layer_structure = self.get_layer_structure();

        return indices
            .iter()
            .map(|index| {
                while *index >= current_index + layer_structure[layer_index] {
                    current_index += layer_structure[layer_index];
                    layer_index += 1;
                }
                return VertexID::new(layer_index, index - current_index);
            })
            .collect::<Vec<VertexID>>();
    }

    pub fn calculate_vertex_flows(&self) -> Vec<Vec<usize>> {
        // calculate the flows of the vertices of the graph, assumes a valid flamecast graph
        let mut vertex_flows = self
            .layers
            .iter()
            .map(|layer| vec![0; layer.vertices.len()])
            .collect::<Vec<Vec<usize>>>();

        for (vertex_index, mut vertex) in self.layers[0].vertices.iter().enumerate() {
            let mut layer_index = 0;
            vertex_flows[0][vertex_index] = 1;
            while vertex.parent_index.is_some() {
                let parent_index = vertex.parent_index.unwrap();
                vertex = &self.layers[layer_index + 1].vertices[parent_index];
                vertex_flows[layer_index + 1][parent_index] += 1;
                layer_index += 1;
            }
        }

        return vertex_flows;
    }

    pub fn calculate_edge_flows(&self) -> Vec<Vec<usize>> {
        // calculate the flows of the edges of the graph, assumes a valid flamecast graph
        let mut edge_flows = Vec::new();

        edge_flows.push(vec![1; self.layers[0].vertices.len()]);

        for (layer_index, layer) in self.layers.iter().enumerate().skip(1) {
            let mut current_layer_flows = Vec::new();

            if layer_index == self.layers.len() - 1 {
                break;
            }

            layer.vertices.iter().for_each(|vertex| {
                let flow = vertex
                    .children_indices
                    .as_ref()
                    .unwrap()
                    .iter()
                    .fold(0, |acc, child_index| {
                        acc + edge_flows[layer_index - 1][*child_index]
                    });
                current_layer_flows.push(flow);
            });
            edge_flows.push(current_layer_flows);
        }

        return edge_flows;
    }

    pub fn is_valid_flamecast_topology_check_all(
        &self,
        capacities: &Vec<usize>,
        number_of_sources: usize,
        number_of_drains: usize,
        num_layers: usize,
    ) -> bool {
        if self.layers.len() != num_layers {
            return false;
        }

        if self.layers[0].vertices.len() != number_of_sources {
            return false;
        }
        if self.layers[num_layers - 1].vertices.len() != number_of_drains {
            return false;
        }

        return self.is_valid_flamecast_topology(capacities);
    }

    pub fn is_valid_flamecast_topology(&self, capacities: &Vec<usize>) -> bool {
        let num_layers = self.layers.len();

        let mut visited_vertices = self
            .get_layer_structure()
            .iter()
            .map(|size| vec![false; *size])
            .collect::<Vec<Vec<bool>>>();
        let mut vertices_flows = self
            .layers
            .iter()
            .map(|layer| vec![0; layer.vertices.len()])
            .collect::<Vec<Vec<usize>>>();

        for (vertex_index, mut vertex) in self.layers[0].vertices.iter().enumerate() {
            visited_vertices[0][vertex_index] = true;
            let mut layer_index = 0;
            vertices_flows[0][vertex_index] = 1;
            while vertex.parent_index.is_some() {
                let parent_index = vertex.parent_index.unwrap();
                vertex = &self.layers[layer_index + 1].vertices[parent_index];
                visited_vertices[layer_index + 1][parent_index] = true;
                vertices_flows[layer_index + 1][parent_index] += 1;
                layer_index += 1;
            }
            if layer_index != num_layers - 1 {
                return false;
            }
        }

        if visited_vertices
            .iter()
            .take(num_layers - 1)
            .any(|layer| layer.iter().any(|vertex| !vertex))
        {
            return false;
        }

        for (layer_flows, capacity) in vertices_flows.iter().zip(capacities.iter()) {
            if layer_flows.iter().any(|flow| *flow > *capacity) {
                return false;
            }
        }

        return true;
    }

    pub fn is_valid_flamecast_topology_check_capacities(&self, capacities: &Vec<usize>) -> bool {
        let mut vertices_flows = self
            .layers
            .iter()
            .map(|layer| vec![0; layer.vertices.len()])
            .collect::<Vec<Vec<usize>>>();

        for (vertex_index, mut vertex) in self.layers[0].vertices.iter().enumerate() {
            let mut layer_index = 0;
            vertices_flows[0][vertex_index] = 1;
            while vertex.parent_index.is_some() {
                let parent_index = vertex.parent_index.unwrap();
                vertex = &self.layers[layer_index + 1].vertices[parent_index];
                vertices_flows[layer_index + 1][parent_index] += 1;
                layer_index += 1;
            }
        }

        for (layer_flows, capacity) in vertices_flows.iter().zip(capacities.iter()) {
            if layer_flows.iter().any(|flow| *flow > *capacity) {
                return false;
            }
        }
        return true;
    }
}
