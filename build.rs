fn main() {
    println!("cargo:rustc-link-search=native=/opt/gurobi811/linux64/lib");
    println!("cargo:rustc-link-lib=gurobi81");
}
