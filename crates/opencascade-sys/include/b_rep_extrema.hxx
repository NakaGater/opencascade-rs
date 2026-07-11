#pragma once
#include "rust/cxx.h"
#include <BRepExtrema_DistShapeShape.hxx>
#include <Standard_Failure.hxx>
#include <gp_Pnt.hxx>
#include <memory>
#include <stdexcept>
#include <string>

// NOTE: the constructor performs the computation, and OCCT exceptions
// (Standard_Failure) do not derive std::exception — convert them here so the
// cxx Result boundary can catch failures instead of aborting.
inline std::unique_ptr<BRepExtrema_DistShapeShape>
BRepExtrema_DistShapeShape_TryNew(const TopoDS_Shape &a, const TopoDS_Shape &b) {
  try {
    std::unique_ptr<BRepExtrema_DistShapeShape> op(new BRepExtrema_DistShapeShape(a, b));
    if (!op->IsDone()) {
      throw std::runtime_error("BRepExtrema_DistShapeShape: not done");
    }
    return op;
  } catch (const Standard_Failure &f) {
    const char *msg = f.GetMessageString();
    throw std::runtime_error(std::string("OCCT Standard_Failure in BRepExtrema_DistShapeShape: ") +
                             (msg && *msg ? msg : f.DynamicType()->Name()));
  }
}

inline std::unique_ptr<gp_Pnt> BRepExtrema_PointOnShape1(const BRepExtrema_DistShapeShape &op,
                                                         int n) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(op.PointOnShape1(n)));
}

inline std::unique_ptr<gp_Pnt> BRepExtrema_PointOnShape2(const BRepExtrema_DistShapeShape &op,
                                                         int n) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(op.PointOnShape2(n)));
}
