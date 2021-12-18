wit_bindgen_rust::import!("provider.wit");
wit_bindgen_rust::export!("runner.wit");

struct Exports;

impl runner::Runner for Exports {
    fn doit(it: provider::Thing) {
        it.frob()
    }
}
