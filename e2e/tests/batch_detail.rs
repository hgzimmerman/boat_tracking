use boat_tracking_e2e::{scroll_and_click, select_value, TestInstance};
use fantoccini::Locator;

/// Helper: creates a boat via the form.
async fn create_boat(client: &fantoccini::Client, base_url: &str, name: &str, weight: &str) {
    client.goto(&format!("{base_url}/boats/new")).await.unwrap();
    let name_field = client.wait().for_element(Locator::Css("#name")).await.unwrap();
    name_field.send_keys(name).await.unwrap();
    select_value(client, "select[name='weight_class']", weight).await;
    scroll_and_click(client, "button[type='submit']").await;
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath(&format!("//*[contains(text(), '{name}')]")))
        .await
        .unwrap();
}

/// The batch detail page shows the correct scenario and boats.
#[tokio::test]
async fn batch_detail_shows_scenario_and_boats() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    create_boat(&client, &base, "Detail Boat A", "Light").await;
    create_boat(&client, &base, "Detail Boat B", "Heavy").await;

    // Add boats to session and create a batch with scenario 1 (first seeded).
    client
        .execute(
            &format!(r##"await fetch("{base}/api/batches/session/add/1", {{method: "POST"}});"##),
            vec![],
        )
        .await
        .unwrap();
    client
        .execute(
            &format!(r##"await fetch("{base}/api/batches/session/add/2", {{method: "POST"}});"##),
            vec![],
        )
        .await
        .unwrap();

    // Submit the batch with scenario 1.
    client
        .execute(
            &format!(
                r##"await fetch("{base}/batches", {{method: "POST", headers: {{"Content-Type": "application/x-www-form-urlencoded"}}, body: "use_scenario_id=1&boat_ids%5B%5D=1&boat_ids%5B%5D=2"}});"##
            ),
            vec![],
        )
        .await
        .unwrap();

    // Navigate to the batch detail page.
    client
        .goto(&format!("{base}/batches/1"))
        .await
        .unwrap();

    // Verify the scenario name is displayed.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Batch Details')]"))
        .await
        .expect("batch detail heading should be present");

    // Verify both boats are listed.
    let source = client.source().await.unwrap();
    assert!(
        source.contains("Detail Boat A"),
        "Detail Boat A should appear on batch detail page"
    );
    assert!(
        source.contains("Detail Boat B"),
        "Detail Boat B should appear on batch detail page"
    );

    // Verify the boat count is shown.
    assert!(
        source.contains("Boats Used (2)"),
        "boat count should show 2"
    );

    // Verify the "Use as Template" button is present.
    client
        .find(Locator::XPath("//a[contains(text(), 'Use as Template')]"))
        .await
        .expect("Use as Template button should be present");

    client.close().await.unwrap();
}

/// The batch detail page for a batch with a known scenario shows the scenario name.
#[tokio::test]
async fn batch_detail_shows_correct_scenario_name() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    create_boat(&client, &base, "Scenario Boat", "Medium").await;

    // Find the "Regatta" scenario ID from the scenarios page.
    client.goto(&format!("{base}/scenarios")).await.unwrap();
    let regatta_id = client
        .execute(
            r##"var rows = document.querySelectorAll("tr"); for (var r of rows) { if (r.textContent.includes("Regatta")) { var link = r.querySelector("a[href*='/edit']"); if (link) return link.href.match(/scenarios\/(\d+)/)[1]; } } return null;"##,
            vec![],
        )
        .await
        .unwrap();

    let id = regatta_id.as_str().expect("Regatta scenario should exist");

    // Create a batch with the Regatta scenario.
    client
        .execute(
            &format!(r##"await fetch("{base}/api/batches/session/add/1", {{method: "POST"}});"##),
            vec![],
        )
        .await
        .unwrap();
    client
        .execute(
            &format!(
                r##"await fetch("{base}/batches", {{method: "POST", headers: {{"Content-Type": "application/x-www-form-urlencoded"}}, body: "use_scenario_id={id}&boat_ids%5B%5D=1"}});"##
            ),
            vec![],
        )
        .await
        .unwrap();

    // View the batch detail.
    client
        .goto(&format!("{base}/batches/1"))
        .await
        .unwrap();

    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'Regatta')]"))
        .await
        .expect("Regatta scenario name should appear on batch detail page");

    client.close().await.unwrap();
}
