#[cfg(test)]
mod tests {
    use wasmtime::component::{Component, Linker};
    use wasmtime::{Engine, Store};

    wasmtime::component::bindgen!({
        path: "../../plugins/random.comp/wit",
        world: "random-provider",
    });

    #[test]
    fn seeded_component_replays_bytes() -> wasmtime::Result<()> {
        let engine = Engine::default();
        let component = Component::from_file(
            &engine,
            std::env::var_os("RANDOM_COMPONENT_PATH").expect("RANDOM_COMPONENT_PATH selects component"),
        )?;
        let linker = Linker::<()>::new(&engine);
        let mut store = Store::new(&engine, ());
        let instance = RandomProvider::instantiate(&mut store, &component, &linker)?;
        let seeded = instance.gams_random_seeded_random();
        let random = instance.wasi_random_random();
        seeded.call_set_seed(&mut store, 123)?;
        let first = random.call_get_random_bytes(&mut store, 16)?;
        seeded.call_set_seed(&mut store, 123)?;
        let second = random.call_get_random_bytes(&mut store, 16)?;
        assert_eq!(first, second);
        assert_eq!(first.len(), 16);
        assert_eq!(seeded.call_get_seed(&mut store)?, 123);
        Ok(())
    }
}
