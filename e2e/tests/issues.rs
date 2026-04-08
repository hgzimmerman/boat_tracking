use boat_tracking_e2e::{scroll_and_click, select_value, TestInstance};
use fantoccini::Locator;

/// Creates an issue without a boat and verifies it appears in the issues list.
#[tokio::test]
async fn create_issue() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    // Submit issue via fetch — the handler now returns the issue list directly.
    client
        .execute(
            &format!(
                r##"await fetch("{base}/issues", {{method: "POST", headers: {{"Content-Type": "application/x-www-form-urlencoded"}}, body: "note=Cracked+rigger+on+port+side"}});"##
            ),
            vec![],
        )
        .await
        .unwrap();

    client
        .goto(&format!("{base}/issues"))
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
    let base = instance.base_url();

    // Create a boat first.
    client
        .goto(&format!("{base}/boats/new"))
        .await
        .unwrap();
    let name_field = client.wait().for_element(Locator::Css("#name")).await.unwrap();
    name_field.send_keys("Issue Test Boat").await.unwrap();
    select_value(&client, "select[name='weight_class']", "Medium").await;
    scroll_and_click(&client, "button[type='submit']").await;
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Issue Test Boat')]"))
        .await
        .unwrap();

    // Create an issue linked to that boat.
    client
        .execute(
            &format!(
                r##"await fetch("{base}/issues", {{method: "POST", headers: {{"Content-Type": "application/x-www-form-urlencoded"}}, body: "boat_id=1&note=Needs+new+oarlock"}});"##
            ),
            vec![],
        )
        .await
        .unwrap();

    client.goto(&format!("{base}/issues")).await.unwrap();
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Needs new oarlock')]"))
        .await
        .expect("issue should appear in list");

    // Resolve the issue.
    client
        .execute(
            &format!(r##"await fetch("{base}/issues/1/resolve", {{method: "POST"}});"##),
            vec![],
        )
        .await
        .unwrap();

    client.goto(&format!("{base}/issues")).await.unwrap();
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Resolved')]"))
        .await
        .expect("issue should show Resolved status");

    // Unresolve the issue.
    client
        .execute(
            &format!(r##"await fetch("{base}/issues/1/unresolve", {{method: "POST"}});"##),
            vec![],
        )
        .await
        .unwrap();

    client.goto(&format!("{base}/issues")).await.unwrap();
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//button[contains(text(), 'Resolve')]"))
        .await
        .expect("resolve button should reappear after reopening");

    client.close().await.unwrap();
}
