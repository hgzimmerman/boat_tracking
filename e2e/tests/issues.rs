use boat_tracking_e2e::{scroll_and_click, select_value, set_input_value, TestInstance};
use fantoccini::Locator;

/// Creates an issue without a boat and verifies it appears in the issues list.
#[tokio::test]
async fn create_issue() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    // Navigate to home first so HTMX is fully initialized before going to the form.
    client.goto(&instance.base_url()).await.unwrap();
    client.wait().for_element(Locator::Css("nav")).await.unwrap();

    client
        .goto(&format!("{}/issues/new", instance.base_url()))
        .await
        .unwrap();

    // Submit the form via fetch (scroll_and_click doesn't trigger native
    // submission for hx-post-only forms) and navigate to the issues list.
    client
        .execute(
            r##"fetch("/issues", {method: "POST", headers: {"Content-Type": "application/x-www-form-urlencoded"}, body: "note=Cracked+rigger+on+port+side"});"##,
            vec![],
        )
        .await
        .unwrap();

    // The handler responds with HX-Redirect (no body), so navigate manually.
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    client
        .goto(&format!("{}/issues", instance.base_url()))
        .await
        .unwrap();

    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath(
            "//*[contains(text(), 'Cracked rigger on port side')]",
        ))
        .await
        .expect("issue should appear in the list");

    client.close().await.unwrap();
}

/// Creates an issue linked to a boat, then resolves and unresolves it.
#[tokio::test]
async fn resolve_and_unresolve_issue() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    // Create a boat first so we can link an issue to it.
    client
        .goto(&format!("{}/boats/new", instance.base_url()))
        .await
        .unwrap();
    let name_field = client.wait().for_element(Locator::Css("#name")).await.unwrap();
    name_field.send_keys("Issue Test Boat").await.unwrap();
    select_value(&client, "select[name='weight_class']", "Medium").await;
    scroll_and_click(&client, "button[type='submit']").await;
    client
        .wait()
        .for_element(Locator::XPath("//*[contains(text(), 'Issue Test Boat')]"))
        .await
        .unwrap();

    // Create an issue linked to that boat.
    client
        .goto(&format!("{}/issues/new", instance.base_url()))
        .await
        .unwrap();

    // Select the first non-empty option (our boat) via JS.
    client
        .execute(
            r#"document.querySelector("select[name='boat_id']").selectedIndex = 1;"#,
            vec![],
        )
        .await
        .unwrap();

    set_input_value(&client, "#note", "Needs new oarlock").await;

    scroll_and_click(&client, "button[type='submit']").await;

    // Verify the issue appears with Open status.
    client
        .wait()
        .for_element(Locator::XPath("//*[contains(text(), 'Needs new oarlock')]"))
        .await
        .expect("issue should appear in list");

    // Resolve the issue via POST and reload the list.
    // The resolve/unresolve handlers return HX-Redirect headers (no body),
    // so we POST via fetch and navigate manually.
    client
        .execute(
            r##"fetch("/issues/1/resolve", {method: "POST"});"##,
            vec![],
        )
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    client
        .goto(&format!("{}/issues", instance.base_url()))
        .await
        .unwrap();

    // Verify the issue now shows as Resolved.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Resolved')]"))
        .await
        .expect("issue should show Resolved status");

    // Unresolve the issue.
    client
        .execute(
            r##"fetch("/issues/1/unresolve", {method: "POST"});"##,
            vec![],
        )
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    client
        .goto(&format!("{}/issues", instance.base_url()))
        .await
        .unwrap();

    // Verify it's back to Open (Resolve button is present again).
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//button[contains(text(), 'Resolve')]"))
        .await
        .expect("resolve button should reappear after reopening");

    client.close().await.unwrap();
}
