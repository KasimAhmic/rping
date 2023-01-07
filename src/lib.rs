use neon::prelude::*;

mod ping_response;
mod rping;
mod runtime;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("ping", rping::ping)?;
    Ok(())
}
