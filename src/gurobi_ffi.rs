pub use std::os::raw::{c_int, c_double, c_char, c_void};
pub type c_str = *const c_char;

use std::ptr;
use std::ffi::CString;

pub enum GRBenv {}

pub enum GRBmodel {}

#[repr(C)]
pub struct GRBsvec {
  /// sparse vector length
  pub len: c_int,
  /// indices array of the sparse vector
  pub ind: *mut c_int,
  /// value array of the sparse vector
  pub val: *mut c_double
}

extern "C" {
  // Constructors
  pub fn GRBloadenv(envP: *mut *mut GRBenv, logfilename: c_str) -> c_int;

  pub fn GRBnewmodel(env: *mut GRBenv, modelP: *mut *mut GRBmodel, Pname: c_str, numvars: c_int,
                     obj: *const c_double, lb: *const c_double, ub: *const c_double, vtype: *const c_char,
                     varnames: *const c_str)
                     -> c_int;

  // Destructors
  pub fn GRBfreeenv(env: *mut GRBenv);

  pub fn GRBfreemodel(model: *mut GRBmodel) -> c_int;


  // Variables
  pub fn GRBaddvar(model: *mut GRBmodel, numnz: c_int, vind: *const c_int, vval: *const c_double, obj: f64, lb: f64,
                   ub: f64, vtype: c_char, name: c_str)
                   -> c_int;

  // Constraints
  pub fn GRBaddconstr(model: *mut GRBmodel, numnz: c_int, cind: *const c_int, cval: *const c_double, sense: c_char,
                      rhs: c_double, constrname: c_str)
                      -> c_int;

  // Optimize
  pub fn GRBoptimize(model: *mut GRBmodel) -> c_int;

  // Diagnostics
  pub fn GRBgeterrormsg(env: *mut GRBenv) -> c_str;

  // Control solver operation (https://www.gurobi.com/documentation/8.1/refman/parameters.html)
  pub fn GRBsetparam(env : *mut GRBenv, paramname : c_str, value : c_str);

  // Retrieve solutions (https://www.gurobi.com/documentation/8.1/refman/attributes.html)
  pub fn GRBgetdblattr(model: *mut GRBmodel, attrname: c_str, valueP: *mut c_double) -> c_int;

  pub fn GRBgetintattr(model: *mut GRBmodel, attrname: c_str, valueP: *mut c_int) -> c_int;

  pub fn GRBgetdblattrarray(model: *mut GRBmodel, attrname: c_str, first: c_int, len: c_int, values: *mut c_double)
                            -> c_int;

  pub fn GRBsetintattr(model: *mut GRBmodel, attrname: c_str, value: c_int) -> c_int;

  // Write to file
  pub fn GRBwrite(model: *mut GRBmodel, filename: c_str) -> c_int;
}

fn main() {
  let mut env : *mut GRBenv = ptr::null_mut();
  let c_ptr: *const c_char = CString::new("mip1.log").unwrap().as_ptr();
  let x = unsafe { GRBloadenv(&mut env, c_ptr)};
}
