pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_tools_history.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopTools_ListOfShape = crate::top_tools::TopTools_ListOfShape;
        type BRepAlgoAPI_Cut = crate::b_rep_algo_api::BRepAlgoAPI_Cut;
        type BRepAlgoAPI_Fuse = crate::b_rep_algo_api::BRepAlgoAPI_Fuse;
        type BRepAlgoAPI_Common = crate::b_rep_algo_api::BRepAlgoAPI_Common;

        // Handles
        type Handle_BRepTools_History;
        pub fn IsNull(self: &Handle_BRepTools_History) -> bool;
        // End Handles

        pub fn BRepAlgoAPI_Cut_History(
            op: Pin<&mut BRepAlgoAPI_Cut>,
        ) -> UniquePtr<Handle_BRepTools_History>;
        pub fn BRepAlgoAPI_Fuse_History(
            op: Pin<&mut BRepAlgoAPI_Fuse>,
        ) -> UniquePtr<Handle_BRepTools_History>;
        pub fn BRepAlgoAPI_Common_History(
            op: Pin<&mut BRepAlgoAPI_Common>,
        ) -> UniquePtr<Handle_BRepTools_History>;

        pub fn BRepTools_History_Modified<'a>(
            history: &'a Handle_BRepTools_History,
            shape: &TopoDS_Shape,
        ) -> Result<&'a TopTools_ListOfShape>;
        pub fn BRepTools_History_Generated<'a>(
            history: &'a Handle_BRepTools_History,
            shape: &TopoDS_Shape,
        ) -> Result<&'a TopTools_ListOfShape>;
        pub fn BRepTools_History_IsRemoved(
            history: &Handle_BRepTools_History,
            shape: &TopoDS_Shape,
        ) -> Result<bool>;
    }

    impl UniquePtr<Handle_BRepTools_History> {}
}
