wit_bindgen_rust::import!("provider.wit");
wit_bindgen_rust::export!("runner.wit");

struct Runner;

impl runner::Runner for Runner {
    fn doit(it: wit_bindgen_rust::Handle<provider::Thing>) {
        it.frob()
    }
}
