use serde::{Deserialize, Serialize};

use crate::VertexID;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Neighbor {
    Recable(VertexID, VertexID),
    Swap(VertexID, VertexID),
    Merge(VertexID, VertexID),
    Split(Vec<VertexID>),
}

impl Neighbor {
    pub fn to_string(&self) -> String {
        match self {
            Neighbor::Recable(v1, v2) => {
                return format!("Recable: {} -> {}", v1.to_string(), v2.to_string());
            }
            Neighbor::Swap(v1, v2) => {
                return format!("Swap: {} <-> {}", v1.to_string(), v2.to_string());
            }
            Neighbor::Merge(v1, v2) => {
                return format!("Merge: {} {}", v1.to_string(), v2.to_string());
            }
            Neighbor::Split(v) => {
                let mut s = String::from("Split: ");
                for v_id in v {
                    s.push_str(&v_id.to_string());
                    s.push_str(" ");
                }
                return s;
            }
        }
    }

    pub fn is_split(&self) -> bool {
        match self {
            Neighbor::Split(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NeighborCost {
    pub neighbor: Neighbor,
    pub cost: f64,
}

impl NeighborCost {
    pub fn new(neighbor: Neighbor, cost: f64) -> Self {
        Self { neighbor, cost }
    }

    pub fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.cost < other.cost {
            self
        } else {
            other
        }
    }
}
