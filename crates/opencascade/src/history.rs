//! Query-only access to `BRepTools_History`: how sub-shapes of the inputs of a
//! modeling operation map to sub-shapes of its result (Modified / Generated /
//! IsRemoved). This enables stable, semantic references to faces/edges across
//! regeneration instead of relying on unstable sub-shape indices.

use cxx::UniquePtr;
use opencascade_sys as ffi;

use crate::primitives::{BooleanShape, Edge, Face, Shape};

/// History of a modeling operation (currently the boolean operations).
///
/// Pure query API: given a sub-shape of an *input* of the operation, returns
/// the corresponding sub-shape(s) in the *result*.
pub struct ShapeHistory {
    inner: UniquePtr<ffi::b_rep_tools_history::Handle_BRepTools_History>,
}

impl ShapeHistory {
    fn list_to_shapes(list: &ffi::top_tools::TopTools_ListOfShape) -> Vec<Shape> {
        let vec = ffi::topo_ds::shape_list_to_vector(list);
        vec.iter().map(Shape::from_shape).collect()
    }

    /// Result shapes which are modified (e.g. split or trimmed) from `shape`.
    pub fn modified(&self, shape: &Shape) -> Vec<Shape> {
        match ffi::b_rep_tools_history::BRepTools_History_Modified(&self.inner, &shape.inner) {
            Ok(list) => Self::list_to_shapes(list),
            Err(_) => vec![],
        }
    }

    /// Result shapes which are newly generated from `shape`.
    pub fn generated(&self, shape: &Shape) -> Vec<Shape> {
        match ffi::b_rep_tools_history::BRepTools_History_Generated(&self.inner, &shape.inner) {
            Ok(list) => Self::list_to_shapes(list),
            Err(_) => vec![],
        }
    }

    /// Whether `shape` was removed by the operation.
    pub fn is_removed(&self, shape: &Shape) -> bool {
        ffi::b_rep_tools_history::BRepTools_History_IsRemoved(&self.inner, &shape.inner)
            .unwrap_or(false)
    }

    /// Convenience: `modified` for a face, returning only faces.
    pub fn modified_faces(&self, face: &Face) -> Vec<Face> {
        let face_shape = ffi::topo_ds::cast_face_to_shape(&face.inner);
        let shape = Shape::from_shape(face_shape);
        self.modified(&shape)
            .iter()
            .map(|s| Face::from_face(ffi::topo_ds::TopoDS::Face(&s.inner)))
            .collect()
    }

    /// Convenience: `generated` for a face, returning only faces.
    pub fn generated_faces(&self, face: &Face) -> Vec<Face> {
        let face_shape = ffi::topo_ds::cast_face_to_shape(&face.inner);
        let shape = Shape::from_shape(face_shape);
        self.generated(&shape)
            .iter()
            .map(|s| Face::from_face(ffi::topo_ds::TopoDS::Face(&s.inner)))
            .collect()
    }

    /// Convenience: `is_removed` for a face.
    pub fn is_removed_face(&self, face: &Face) -> bool {
        let face_shape = ffi::topo_ds::cast_face_to_shape(&face.inner);
        let shape = Shape::from_shape(face_shape);
        self.is_removed(&shape)
    }

    /// Convenience: `modified` for an edge, returning only edges.
    pub fn modified_edges(&self, edge: &Edge) -> Vec<Edge> {
        let edge_shape = ffi::topo_ds::cast_edge_to_shape(&edge.inner);
        let shape = Shape::from_shape(edge_shape);
        self.modified(&shape)
            .iter()
            .map(|s| Edge::from_edge(ffi::topo_ds::TopoDS::Edge(&s.inner)))
            .collect()
    }

    /// Convenience: `is_removed` for an edge.
    pub fn is_removed_edge(&self, edge: &Edge) -> bool {
        let edge_shape = ffi::topo_ds::cast_edge_to_shape(&edge.inner);
        let shape = Shape::from_shape(edge_shape);
        self.is_removed(&shape)
    }
}

impl Shape {
    /// Like [`Shape::subtract`], but also returns the operation history.
    pub fn subtract_with_history(&self, other: &Shape) -> (BooleanShape, ShapeHistory) {
        let mut op = ffi::b_rep_algo_api::BRepAlgoAPI_Cut_new(&self.inner, &other.inner);
        let new_edges = new_edges_from(op.pin_mut().SectionEdges());
        let shape = Shape::from_shape(op.pin_mut().Shape());
        let history = ShapeHistory {
            inner: ffi::b_rep_tools_history::BRepAlgoAPI_Cut_History(op.pin_mut()),
        };
        (BooleanShape { shape, new_edges }, history)
    }

    /// Like [`Shape::union`], but also returns the operation history.
    pub fn union_with_history(&self, other: &Shape) -> (BooleanShape, ShapeHistory) {
        let mut op = ffi::b_rep_algo_api::BRepAlgoAPI_Fuse_new(&self.inner, &other.inner);
        let new_edges = new_edges_from(op.pin_mut().SectionEdges());
        let shape = Shape::from_shape(op.pin_mut().Shape());
        let history = ShapeHistory {
            inner: ffi::b_rep_tools_history::BRepAlgoAPI_Fuse_History(op.pin_mut()),
        };
        (BooleanShape { shape, new_edges }, history)
    }

    /// Like [`Shape::intersect`], but also returns the operation history.
    pub fn intersect_with_history(&self, other: &Shape) -> (BooleanShape, ShapeHistory) {
        let mut op = ffi::b_rep_algo_api::BRepAlgoAPI_Common_new(&self.inner, &other.inner);
        let new_edges = new_edges_from(op.pin_mut().SectionEdges());
        let shape = Shape::from_shape(op.pin_mut().Shape());
        let history = ShapeHistory {
            inner: ffi::b_rep_tools_history::BRepAlgoAPI_Common_History(op.pin_mut()),
        };
        (BooleanShape { shape, new_edges }, history)
    }
}

fn new_edges_from(edge_list: &ffi::top_tools::TopTools_ListOfShape) -> Vec<crate::primitives::Edge> {
    let vec = ffi::topo_ds::shape_list_to_vector(edge_list);
    let mut new_edges = vec![];
    for shape in vec.iter() {
        let edge = ffi::topo_ds::TopoDS::Edge(shape);
        new_edges.push(crate::primitives::Edge::from_edge(edge));
    }
    new_edges
}
