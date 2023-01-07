use crate::ping_response::PingResponse;
use crate::runtime::async_runtime;
use neon::prelude::*;
use surge_ping::IcmpPacket;

async fn execute_ping(
    ip_address: String,
    num_pings: u32,
) -> Result<Vec<PingResponse>, Box<dyn std::error::Error + Send + Sync>> {
    let payload = [0; 24];
    let mut results: Vec<PingResponse> = Vec::new();

    for _ in 0..num_pings {
        let (packet, duration) =
            match surge_ping::ping(std::net::IpAddr::V4(ip_address.parse()?), &payload).await {
                Ok((IcmpPacket::V4(packet), duration)) => (packet, duration),
                Ok(_) => unimplemented!(),
                Err(e) => panic!("{}", e),
            };

        let result = PingResponse {
            source: packet.get_source(),
            size: packet.get_size(),
            sequence: packet.get_sequence(),
            ttl: packet.get_ttl(),
            duration,
        };

        results.push(result);
    }

    Ok(results)
}

pub fn ping(mut fc: FunctionContext) -> JsResult<JsPromise> {
    let rt = async_runtime(&mut fc)?;
    let channel = fc.channel();

    let ip_address = fc.argument::<JsString>(0)?.value(&mut fc);
    let num_pings = fc.argument::<JsNumber>(1)?.value(&mut fc) as u32;

    let (deferred, promise) = fc.promise();

    rt.spawn(async move {
        let result = execute_ping(ip_address, num_pings).await;

        deferred.settle_with(&channel, move |mut tc| {
            let results = result.unwrap();

            let arr = JsArray::new(&mut tc, results.len() as u32);

            for (i, ping_result) in results.iter().enumerate() {
                let value = ping_result.to_object(&mut tc)?;

                arr.set(&mut tc, i as u32, value)?;
            }

            Ok(arr)
        });
    });

    Ok(promise)
}
