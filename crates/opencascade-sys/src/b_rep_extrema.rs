pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_extrema.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type gp_Pnt = crate::gp::gp_Pnt;

        type BRepExtrema_DistShapeShape;

        /// 構築=計算実行。失敗はstd::runtime_errorに変換されResultで返る
        pub fn BRepExtrema_DistShapeShape_TryNew(
            a: &TopoDS_Shape,
            b: &TopoDS_Shape,
        ) -> Result<UniquePtr<BRepExtrema_DistShapeShape>>;

        pub fn Value(self: &BRepExtrema_DistShapeShape) -> f64;
        pub fn NbSolution(self: &BRepExtrema_DistShapeShape) -> i32;

        pub fn BRepExtrema_PointOnShape1(
            op: &BRepExtrema_DistShapeShape,
            n: i32,
        ) -> UniquePtr<gp_Pnt>;
        pub fn BRepExtrema_PointOnShape2(
            op: &BRepExtrema_DistShapeShape,
            n: i32,
        ) -> UniquePtr<gp_Pnt>;
    }
}
