#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexID {
    pub layer: usize,
    pub index: usize,
}

impl VertexID {
    pub fn new(layer: usize, index: usize) -> Self {
        Self { layer, index }
    }
}

#[derive(Debug, Clone)]
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

    pub fn set_parent(&mut self, parent_index: usize) {
        self.parent_index = Some(parent_index);
    }

    pub fn remove_parent(&mut self) {
        self.parent_index = None;
    }

    pub fn add_child(&mut self, child_index: usize) {
        self.children_indices
            .get_or_insert_with(|| Vec::new())
            .push(child_index);
    }

    pub fn remove_child(&mut self, child_index: usize) {
        if let Some(children_indices) = &mut self.children_indices {
            children_indices.swap_remove(
                children_indices
                    .iter()
                    .position(|x| *x == child_index)
                    .unwrap(),
            );
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
                child.set_parent(vertex_index);
            });
        }

        if let Some(parent_index) = vertex.parent_index {
            let parent = &mut last[1].vertices[parent_index];
            parent.add_child(vertex_index);
        }

        return VertexID::new(layer_index, vertex_index);
    }

    pub fn remove_vertex(&mut self, vertex: &VertexID) {
        // remove a vertex from the graph
        let old_index = self.layers[vertex.layer].remove_vertex(vertex);

        let (first, last) = self.layers.split_at_mut(vertex.layer);

        let swapped_vertex = &last[0].vertices[vertex.index];

        if let Some(children_indices) = &swapped_vertex.children_indices {
            let children_layer = &mut first[vertex.layer - 1];
            children_indices.iter().for_each(|child_index| {
                let child = &mut children_layer.vertices[*child_index];
                child.set_parent(vertex.index);
            });
        }

        if let Some(parent_index) = swapped_vertex.parent_index {
            let parent = &mut last[1].vertices[parent_index];
            parent.change_child(old_index, vertex.index);
        }
    }

    pub fn add_edge(&mut self, source: &VertexID, target: &VertexID) {
        // add an edge to the graph
        let source_vertex = self.get_vertex_mut(source);
        source_vertex.set_parent(target.index);
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

        if vertex.layer != self.layers.len() - 1 {
            neighbours.push(self.get_parent(vertex).unwrap());
        }
        if vertex.layer != 0 {
            neighbours.append(&mut self.get_children(vertex).unwrap());
        }

        return neighbours;
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

    pub fn is_valid_flamecast_topology(
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
}
