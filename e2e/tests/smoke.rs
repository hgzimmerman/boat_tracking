use boat_tracking_e2e::TestInstance;
use fantoccini::Locator;

/// Verifies the app boots and the home page contains the expected navigation.
#[tokio::test]
async fn home_page_loads() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    client.goto(&instance.base_url()).await.unwrap();

    let source = client.source().await.unwrap();
    assert!(
        source.contains("Batches") || source.contains("Boats"),
        "expected navigation content in page body"
    );

    client.close().await.unwrap();
}

/// Verifies the boats page renders a heading.
#[tokio::test]
async fn boats_page_loads() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    client
        .goto(&format!("{}/boats", instance.base_url()))
        .await
        .unwrap();

    client
        .wait()
        .for_element(Locator::Css("h1, h2, h3"))
        .await
        .expect("expected a heading element on the boats page");

    client.close().await.unwrap();
}
