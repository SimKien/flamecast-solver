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
    pub vertex_id: VertexID,
    pub parent_index: Option<usize>,
    pub children_indices: Option<Vec<usize>>,
}

impl Vertex {
    pub fn new(
        vertex_id: VertexID,
        parent_index: Option<usize>,
        children_indices: Option<Vec<usize>>,
    ) -> Self {
        Self {
            vertex_id,
            parent_index,
            children_indices,
        }
    }

    pub fn new_with_id(layer_index: usize, index: usize) -> Self {
        Self {
            vertex_id: VertexID::new(layer_index, index),
            parent_index: None,
            children_indices: None,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            vertex_id: VertexID::new(0, 0),
            parent_index: None,
            children_indices: None,
        }
    }

    pub fn set_index(&mut self, index: usize) {
        self.vertex_id.index = index;
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.vertex_id.layer = layer;
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
    pub index: usize,
    pub vertices: Vec<Vertex>,
}

impl Layer {
    pub fn new(index: usize, size: usize) -> Self {
        let mut vertices = Vec::with_capacity(size);

        (0..size).for_each(|vertex_index| {
            vertices.push(Vertex::new_with_id(index, vertex_index));
        });
        Self { index, vertices }
    }

    pub fn from(index: usize, vertices: Vec<Vertex>) -> Self {
        let mut vertices = vertices;

        let mut vertex_index = 0;
        vertices.iter_mut().for_each(|vertex| {
            vertex.set_layer(index);
            vertex.set_index(vertex_index);
            vertex_index += 1;
        });

        Self { index, vertices }
    }

    pub fn new_with_index(index: usize) -> Self {
        Self {
            index,
            vertices: Vec::new(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            index: 0,
            vertices: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        // add a vertex to the layer
        let mut vertex = vertex;
        let index = self.vertices.len();
        vertex.set_index(index);
        vertex.set_layer(self.index);
        self.vertices.push(vertex);
        return index;
    }

    pub fn remove_vertex(&mut self, vertex: &VertexID) -> usize {
        // remove a vertex from the layer and returns the the old index of the vertex which was swapped with the removed vertex
        self.vertices.swap_remove(vertex.index);

        let swapped_vertex = &mut self.vertices[vertex.index];

        let old_index = swapped_vertex.vertex_id.index;

        swapped_vertex.set_index(vertex.index);

        return old_index;
    }
}

#[derive(Debug, Clone)]
pub struct LayeredGraph {
    pub layers: Vec<Layer>,
}

impl LayeredGraph {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn new_empty() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn new_with_size(num_layers: usize) -> Self {
        Self {
            layers: (0..num_layers).map(|i| Layer::new_with_index(i)).collect(),
        }
    }

    pub fn from_sources_drains(
        sources: Vec<Vertex>,
        drains: Vec<Vertex>,
        num_layers: usize,
    ) -> Self {
        // create an almost empty graph with only sources and drains
        let mut graph = LayeredGraph::new_empty();

        graph.add_layer(Layer::from(0, sources));

        for i in 1..num_layers - 1 {
            graph.add_layer(Layer::new_with_index(i));
        }

        graph.add_layer(Layer::from(num_layers - 1, drains));

        return graph;
    }

    pub fn add_layer(&mut self, layer: Layer) {
        // add a layer to the graph
        let mut layer = layer;
        layer.index = self.layers.len();

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

    pub fn remove_edge(&mut self, source: &VertexID, target: &VertexID) {
        // remove an edge from the graph
        let source_vertex = self.get_vertex_mut(source);
        source_vertex.remove_parent();
        let target_vertex = self.get_vertex_mut(target);
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

        return self.layers.iter().fold(0, |acc, layer| {
            if layer.index == number_of_layers - 1 {
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
        let regarded_vertex = self.layers[vertex.layer]
            .vertices
            .get(vertex.index)
            .unwrap();

        return match regarded_vertex.parent_index {
            Some(parent_index) => Some(VertexID::new(vertex.layer + 1, parent_index)),
            None => None,
        };
    }

    pub fn get_children(&self, vertex: &VertexID) -> Option<Vec<VertexID>> {
        // get the children of a vertex
        let regarded_vertex = self.layers[vertex.layer]
            .vertices
            .get(vertex.index)
            .unwrap();

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
            .map(|vertex| vertex.vertex_id.clone())
            .collect()
    }

    pub fn get_drains_indexes(&self) -> Vec<VertexID> {
        // get the drains of the graph
        self.layers[self.layers.len() - 1]
            .vertices
            .iter()
            .map(|vertex| vertex.vertex_id.clone())
            .collect()
    }

    pub fn get_vertex_layers_with_indexes(&self) -> Vec<Vec<VertexID>> {
        // get the layers of the graph
        let mut layers = Vec::new();

        self.layers.iter().for_each(|layer| {
            layers.push(
                layer
                    .vertices
                    .iter()
                    .map(|vertex| vertex.vertex_id.clone())
                    .collect(),
            );
        });

        return layers;
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

        for layer in self.layers.iter().skip(1) {
            let mut current_layer_flows = Vec::new();

            if layer.index == self.layers.len() - 1 {
                break;
            }

            layer.vertices.iter().for_each(|vertex| {
                let flow = vertex
                    .children_indices
                    .as_ref()
                    .unwrap()
                    .iter()
                    .fold(0, |acc, child_index| {
                        acc + edge_flows[vertex.vertex_id.layer - 1][*child_index]
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

        for mut vertex in self.layers[0].vertices.iter() {
            visited_vertices[0][vertex.vertex_id.index] = true;
            let mut layer_index = 0;
            vertices_flows[0][vertex.vertex_id.index] = 1;
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
