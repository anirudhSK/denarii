pub use std::os::raw::{c_int, c_double, c_char, c_void};
pub type c_str = *const c_char;

use std::ptr;
use std::ffi::CString;
use std::convert::TryInto;

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

struct GurobiOptimizer {
  env   : *mut GRBenv,
  model : *mut GRBmodel
}

impl GurobiOptimizer {
  pub fn new(name : &str) -> GurobiOptimizer {
    let mut optimizer = GurobiOptimizer{ env : ptr::null_mut(),  model : ptr::null_mut() };
    let log_file_c_str   = CString::new(name.to_owned() + ".log").expect("CString::new failed");
    let log_file_c_ptr   = log_file_c_str.as_ptr();
    let model_name_c_str = CString::new(name).expect("CString::new failed");
    let model_name_c_ptr = model_name_c_str.as_ptr();
    unsafe {
      GRBloadenv(&mut optimizer.env, log_file_c_ptr);
      GRBnewmodel(optimizer.env, &mut optimizer.model, model_name_c_ptr, 0, ptr::null_mut(),
                  ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    }
    return optimizer;
  }
  pub fn add_var(&mut self, var_name : &str, var_type : char, is_objective : bool) {
    assert!(['C', 'B', 'I'].contains(&var_type), "var_type must be C (real), B (binary), or I (integer)");
    let var_name_c_str = CString::new(var_name).expect("CString::new failed");
    let var_name_c_ptr = var_name_c_str.as_ptr();
    unsafe {
      let coeff = is_objective as i8 as f64;
      GRBaddvar(self.model, 0, ptr::null_mut(), ptr::null_mut(), coeff, 0.0, 1e100, var_type as i8, var_name_c_ptr);
    }
  }
  pub fn add_constraint(&mut self,
                        lhs_vars : &Vec<(i32)>,
                        lhs_coeffs : &Vec<f64>,
                        sense : c_char,
                        rhs : f64,
                        constraint_name : &str) {
    assert!(['<' as c_char, '>' as c_char, '=' as c_char].contains(&sense));
    let constraint_name_c_str = CString::new(constraint_name).expect("CString::new failed");
    let constraint_name_c_ptr = constraint_name_c_str.as_ptr();
    assert!(lhs_vars.len() == lhs_coeffs.len());
    unsafe {
      GRBaddconstr(self.model, lhs_vars.len().try_into().unwrap(), lhs_vars.as_ptr(),
                   lhs_coeffs.as_ptr(), sense, rhs, constraint_name_c_ptr);
    }
  }
  pub fn optimize(&mut self, sense : &str) {
    assert!(["max", "min"].contains(&sense));
    let sense_int = if sense == "min" {1} else {-1};
    let model_sense_c_str = CString::new("ModelSense").expect("Cstring::new failed");
    let model_sense_c_ptr  = model_sense_c_str.as_ptr();
    unsafe {
      GRBsetintattr(self.model, model_sense_c_ptr, sense_int);
      GRBoptimize(self.model);
    }
  }
}

fn main() {
  let mut optimizer = GurobiOptimizer::new("mip1");
  optimizer.add_var("x", 'B', false);
  optimizer.add_var("y", 'B', false);
  optimizer.add_var("z", 'B', false);
  optimizer.add_var("obj", 'I', true);
  optimizer.add_constraint(&vec![0, 1, 2, 3], &vec![1.0, 1.0, 2.0, -1.0], '=' as c_char, 0.0, "cequal");
  optimizer.add_constraint(&vec![0, 1, 2], &vec![1.0, 2.0, 3.0], '<' as c_char, 4.0, "c0");
  optimizer.add_constraint(&vec![0, 1], &vec![1.0, 1.0], '>' as c_char, 1.0, "c1");
  optimizer.optimize("max");
}
