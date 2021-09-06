//! time-provider capability provider
//!
//!
use chrono::{DateTime, NaiveDateTime, Utc};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use time_interface::{FormatTimeRequest, Time, TimeReceiver};
use wasmbus_rpc::provider::prelude::*;
//use wasmcloud_interface_factorial::{Factorial, FactorialReceiver};

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(TimeProviderProvider::default())?;

    eprintln!("time-provider provider exiting");
    Ok(())
}

/// time-provider capability provider implementation
#[derive(Default, Clone)]
struct TimeProviderProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for TimeProviderProvider {}
impl TimeReceiver for TimeProviderProvider {}
impl ProviderHandler for TimeProviderProvider {}

/// Handle Factorial methods
#[async_trait]
impl Time for TimeProviderProvider {
    /// Provides time according to Unix epoch format
    async fn get_timestamp(&self, _ctx: &Context) -> RpcResult<u64> {
        Ok(timestamp())
    }

    async fn format_timestamp(&self, _ctx: &Context, arg: &FormatTimeRequest) -> RpcResult<String> {
        Ok(timestamp_format_string(
            arg.timestamp,
            arg.rfc.to_uppercase(),
        ))
    }
}

/// Get UTC timestamp w/ millisecond precision
fn timestamp() -> u64 {
    let utc_time: DateTime<Utc> = Utc::now();
    return utc_time.timestamp_millis() as u64;
}

/// Convert timestamp to DateTime, then format as a string according to RFC specified
fn timestamp_format_string(timestamp: u64, rfc_format: String) -> String {
    let u64_secs = timestamp / 1000;
    let u64_msecs = timestamp - u64_secs;
    let u32_nsecs = (u64_msecs * 1000000) as u32;
    let i64_timestamp = u64_secs as i64;
    let dt =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(i64_timestamp, u32_nsecs), Utc);
    if rfc_format.eq("RFC2822") {
        return dt.to_rfc2822();
    } else {
        return dt.to_rfc3339();
    }
}

/// Handle incoming rpc messages and dispatch to applicable trait handler.
#[async_trait]
impl MessageDispatch for TimeProviderProvider {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> RpcResult<Message<'_>> {
        let op = match message.method.split_once('.') {
            Some((cls, op)) if cls == "Time" => op,
            //Some((cls, op)) if cls == "GetTimestamp" => op,
            //Some((cls, op)) if cls == "FormatTimestamp" => op,
            None => message.method,
            _ => {
                return Err(RpcError::MethodNotHandled(message.method.to_string()));
            }
        };
        TimeReceiver::dispatch(
            self,
            ctx,
            &Message {
                method: op,
                arg: message.arg,
            },
        )
        .await
    }
}
