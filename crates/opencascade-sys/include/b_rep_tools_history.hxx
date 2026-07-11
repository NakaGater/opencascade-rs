#pragma once
#include "bindings_common.hxx"
#include "rust/cxx.h"
#include <BRepAlgoAPI_Common.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepTools_History.hxx>
#include <TopTools_ListOfShape.hxx>
#include <memory>

typedef opencascade::handle<BRepTools_History> Handle_BRepTools_History;

inline std::unique_ptr<Handle_BRepTools_History> BRepAlgoAPI_Cut_History(BRepAlgoAPI_Cut &op) {
  return std::unique_ptr<Handle_BRepTools_History>(new Handle_BRepTools_History(op.History()));
}

inline std::unique_ptr<Handle_BRepTools_History> BRepAlgoAPI_Fuse_History(BRepAlgoAPI_Fuse &op) {
  return std::unique_ptr<Handle_BRepTools_History>(new Handle_BRepTools_History(op.History()));
}

inline std::unique_ptr<Handle_BRepTools_History> BRepAlgoAPI_Common_History(BRepAlgoAPI_Common &op) {
  return std::unique_ptr<Handle_BRepTools_History>(new Handle_BRepTools_History(op.History()));
}

inline const TopTools_ListOfShape &BRepTools_History_Modified(const Handle_BRepTools_History &history,
                                                              const TopoDS_Shape &shape) {
  return handle_try_deref(history).Modified(shape);
}

inline const TopTools_ListOfShape &BRepTools_History_Generated(const Handle_BRepTools_History &history,
                                                               const TopoDS_Shape &shape) {
  return handle_try_deref(history).Generated(shape);
}

inline bool BRepTools_History_IsRemoved(const Handle_BRepTools_History &history, const TopoDS_Shape &shape) {
  return handle_try_deref(history).IsRemoved(shape);
}
