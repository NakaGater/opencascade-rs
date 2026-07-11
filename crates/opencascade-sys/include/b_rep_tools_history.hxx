#pragma once
#include "bindings_common.hxx"
#include "rust/cxx.h"
#include <BRepAlgoAPI_Common.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepFilletAPI_MakeChamfer.hxx>
#include <BRepFilletAPI_MakeFillet.hxx>
#include <BRepTools_History.hxx>
#include <Standard_Failure.hxx>
#include <TopTools_ListOfShape.hxx>
#include <memory>
#include <stdexcept>
#include <string>

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

// --- BRepBuilderAPI_MakeShape系(フィレット/面取り)の失敗を例外として
//     Result境界(cxx)で捕捉するためのTryラッパと、Historyへの変換 ---

// NOTE: OCCT exceptions (Standard_Failure) do NOT derive std::exception, so
// cxx's Result boundary would not catch them (std::terminate). Convert them
// to std::runtime_error here.
inline const TopoDS_Shape &BRepFilletAPI_MakeFillet_TryShape(BRepFilletAPI_MakeFillet &op) {
  try {
    op.Build();
    if (!op.IsDone()) {
      throw std::runtime_error(
          "BRepFilletAPI_MakeFillet: not done (radius may be too large for the adjacent geometry)");
    }
    return op.Shape();
  } catch (const Standard_Failure &f) {
    const char *msg = f.GetMessageString();
    throw std::runtime_error(std::string("OCCT Standard_Failure: ") + (msg ? msg : f.DynamicType()->Name()));
  }
}

inline const TopoDS_Shape &BRepFilletAPI_MakeChamfer_TryShape(BRepFilletAPI_MakeChamfer &op) {
  try {
    op.Build();
    if (!op.IsDone()) {
      throw std::runtime_error(
          "BRepFilletAPI_MakeChamfer: not done (distance may be too large for the adjacent geometry)");
    }
    return op.Shape();
  } catch (const Standard_Failure &f) {
    const char *msg = f.GetMessageString();
    throw std::runtime_error(std::string("OCCT Standard_Failure: ") + (msg ? msg : f.DynamicType()->Name()));
  }
}

inline std::unique_ptr<Handle_BRepTools_History>
BRepFilletAPI_MakeFillet_History(BRepFilletAPI_MakeFillet &op, const TopoDS_Shape &arg) {
  TopTools_ListOfShape args;
  args.Append(arg);
  return std::unique_ptr<Handle_BRepTools_History>(
      new Handle_BRepTools_History(new BRepTools_History(args, op)));
}

inline std::unique_ptr<Handle_BRepTools_History>
BRepFilletAPI_MakeChamfer_History(BRepFilletAPI_MakeChamfer &op, const TopoDS_Shape &arg) {
  TopTools_ListOfShape args;
  args.Append(arg);
  return std::unique_ptr<Handle_BRepTools_History>(
      new Handle_BRepTools_History(new BRepTools_History(args, op)));
}
