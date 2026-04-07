use boat_tracking_e2e::TestInstance;
use fantoccini::Locator;

/// Creates a new scenario and verifies it appears in the scenario list.
#[tokio::test]
async fn create_scenario() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    // Submit via fetch since the form only has hx-post (no native action).
    client
        .execute(
            &format!(
                r##"await fetch("{base}/scenarios", {{method: "POST", headers: {{"Content-Type": "application/x-www-form-urlencoded"}}, body: "name=Sunday+Fun+Row&default_time=09%3A00"}});"##
            ),
            vec![],
        )
        .await
        .unwrap();

    client
        .goto(&format!("{base}/scenarios"))
        .await
        .unwrap();

    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Sunday Fun Row')]"))
        .await
        .expect("new scenario should appear in list");

    // Verify the default time is displayed.
    let source = client.source().await.unwrap();
    assert!(
        source.contains("9:00 AM"),
        "default time 9:00 AM should be displayed in the scenario list"
    );

    client.close().await.unwrap();
}

/// Creates a scenario without a default time and verifies it shows a dash.
#[tokio::test]
async fn create_scenario_without_default_time() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    client
        .execute(
            &format!(
                r##"await fetch("{base}/scenarios", {{method: "POST", headers: {{"Content-Type": "application/x-www-form-urlencoded"}}, body: "name=Open+Paddle"}});"##
            ),
            vec![],
        )
        .await
        .unwrap();

    client
        .goto(&format!("{base}/scenarios"))
        .await
        .unwrap();

    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Open Paddle')]"))
        .await
        .expect("new scenario should appear in list");

    client.close().await.unwrap();
}

/// Edits an existing scenario's name and verifies the update.
#[tokio::test]
async fn edit_scenario() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    // Find a seeded scenario ID. "Learn To Row" is one of the seeds.
    client
        .goto(&format!("{base}/scenarios"))
        .await
        .unwrap();

    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Learn To Row')]"))
        .await
        .expect("Learn To Row should be in the list");

    // Get the scenario's ID from the edit link href.
    let edit_href = client
        .execute(
            r##"var link = Array.from(document.querySelectorAll("a")).find(a => a.previousElementSibling?.previousElementSibling?.textContent?.includes("Learn To Row") || a.closest("tr")?.textContent?.includes("Learn To Row")); return link ? link.getAttribute("href") : null;"##,
            vec![],
        )
        .await
        .unwrap();

    let href = edit_href.as_str().expect("should find edit link for Learn To Row");
    // href is like "/scenarios/6/edit", extract the ID
    let id: &str = href.split('/').nth(2).expect("should parse scenario ID from href");

    // Update via fetch.
    client
        .execute(
            &format!(
                r##"await fetch("{base}/scenarios/{id}", {{method: "POST", headers: {{"Content-Type": "application/x-www-form-urlencoded"}}, body: "name=Learn+To+Scull&default_time=10%3A00"}});"##
            ),
            vec![],
        )
        .await
        .unwrap();

    client
        .goto(&format!("{base}/scenarios"))
        .await
        .unwrap();

    // Old name should be gone, new name should appear.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Learn To Scull')]"))
        .await
        .expect("renamed scenario should appear in list");

    let source = client.source().await.unwrap();
    assert!(
        !source.contains("Learn To Row"),
        "old scenario name should no longer appear"
    );
    assert!(
        source.contains("10:00 AM"),
        "new default time should be displayed"
    );

    client.close().await.unwrap();
}
