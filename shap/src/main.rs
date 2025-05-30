use catboost::Model;

fn main() {
    let model = Model::load("model.cbm").expect("failed to load model");

    let floats: Vec<f32> = vec![1.1, 2.2];
    let strings: Vec<&str> = vec!["a", "b"];

    let shap_values = model.calc_shap_values_single(&floats, &strings);

    println!("shap_values: {:?}", shap_values)
}
