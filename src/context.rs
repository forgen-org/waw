use wasmtime::{
    component::{Linker, ResourceTable},
    Config, Engine, Result, Store,
};
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

pub struct Context {
    pub engine: Engine,
    pub linker: Linker<State>,
    pub store: Store<State>,
}

impl Context {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::<State>::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let mut builder = WasiCtxBuilder::new();

        let store = Store::new(
            &engine,
            State {
                ctx: builder.build(),
                table: ResourceTable::new(),
            },
        );

        Ok(Self {
            engine,
            linker,
            store,
        })
    }
}

pub struct State {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl IoView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for State {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
