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

/*
/// tests of the Factorial capability
async fn factorial_0_1(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // create client and ctx
    let client = FactorialSender::via(prov);
    let ctx = Context::default();

    let resp = client.calculate(&ctx, &0).await?;
    assert_eq!(resp, 1, "0!");

    let resp = client.calculate(&ctx, &1).await?;
    assert_eq!(resp, 1, "1!");

    Ok(())
}

/// more tests of the Factorial interface
async fn factorial_more(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // create client and ctx
    let client = FactorialSender::via(prov);
    let ctx = Context::default();

    let resp = client.calculate(&ctx, &2).await?;
    assert_eq!(resp, 2, "2!");

    let resp = client.calculate(&ctx, &3).await?;
    assert_eq!(resp, 6, "3!");

    let resp = client.calculate(&ctx, &4).await?;
    assert_eq!(resp, 24, "4!");

    Ok(())
}
*/
