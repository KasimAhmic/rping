use neon::prelude::*;

mod ping_response;
mod rs_ping;
mod runtime;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("ping", rs_ping::ping)?;
    Ok(())
}
