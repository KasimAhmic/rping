use std::{net::Ipv4Addr, time::Duration};

use neon::prelude::*;
use surge_ping::PingSequence;

pub struct PingResponse {
    pub source: Ipv4Addr,
    pub size: usize,
    pub sequence: PingSequence,
    pub ttl: u8,
    pub duration: Duration,
}

impl PingResponse {
    pub fn to_object<'a>(&self, ctx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = ctx.empty_object();

        let source = ctx.string(&self.source.to_string());
        let size = ctx.number(self.size as f64);
        let sequence = ctx.number(self.sequence.into_u16());
        let ttl = ctx.number(self.ttl);
        let duration = ctx.number(self.duration.as_secs_f64());

        obj.set(ctx, "source", source)?;
        obj.set(ctx, "size", size)?;
        obj.set(ctx, "sequence", sequence)?;
        obj.set(ctx, "ttl", ttl)?;
        obj.set(ctx, "duration", duration)?;

        Ok(obj)
    }
}
