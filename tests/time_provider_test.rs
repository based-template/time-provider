use time_interface::*;
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_test_util::{
    check,
    cli::print_test_results,
    provider_test::test_provider,
    testing::{TestOptions, TestResult},
};
#[allow(unused_imports)]
use wasmcloud_test_util::{run_selected, run_selected_spawn};

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, health_check, timestamp_ne_0, format_timestamp);
    //let res = run_selected_spawn!(&opts, health_check, factorial_0_1, factorial_more);
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.pass).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut dowwn gracefully
    let provider = test_provider().await;
    let _ = provider.shutdown().await;
}

/// test that health check returns healthy
async fn health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // health check
    let hc = prov.health_check().await;
    check!(hc.is_ok())?;
    Ok(())
}

/// test basic functionality of timestamp()
async fn timestamp_ne_0(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    let client = TimeSender::via(prov);
    let ctx = Context::default();

    let resp = client.get_timestamp(&ctx).await?;
    assert!(resp > 0, "timestamp is positive");

    Ok(())
}

/// test basic functionality for format_timestamp()
async fn format_timestamp(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    let client = TimeSender::via(prov);
    let ctx = Context::default();

    let timestamp = client.get_timestamp(&ctx).await?;
    let format_str_req = FormatTimeRequest {
        rfc: String::from("RFC2822"),
        timestamp: timestamp,
    };
    let rfc2822_time_str = client.format_timestamp(&ctx, &format_str_req).await;
    assert!(rfc2822_time_str.is_ok(), "format_timestamp() succeeded");

    Ok(())
}
