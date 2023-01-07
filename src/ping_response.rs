use neon::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PingResponse {
    pub source: String,
    pub size: String,
    pub sequence: f64,
    pub ttl: f64,
    pub duration: f64,
}

impl PingResponse {
    pub fn to_object<'a>(&self, ctx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = ctx.empty_object();

        let source = ctx.string(&self.source);
        let size = ctx.string(&self.size);
        let sequence = ctx.number(self.sequence);
        let ttl = ctx.number(self.ttl);
        let duration = ctx.number(self.duration);

        obj.set(ctx, "source", source)?;
        obj.set(ctx, "size", size)?;
        obj.set(ctx, "sequence", sequence)?;
        obj.set(ctx, "ttl", ttl)?;
        obj.set(ctx, "duration", duration)?;

        Ok(obj)
    }
}
