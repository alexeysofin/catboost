fn main() {
    println!(
        "cargo:rustc-link-search={}",
        "catboost_build/catboost/libs/model_interface/"
    );
}
