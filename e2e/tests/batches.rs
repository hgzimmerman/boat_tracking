use boat_tracking_e2e::{scroll_and_click, select_value, TestInstance};
use fantoccini::Locator;

/// Helper: creates a boat and returns to the boats list.
async fn create_boat(client: &fantoccini::Client, base_url: &str, name: &str, weight: &str) {
    client
        .goto(&format!("{base_url}/boats/new"))
        .await
        .unwrap();
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

/// Adding a boat from the left pane moves it to the right pane and removes it
/// from the left pane's search results.
#[tokio::test]
async fn adding_boat_removes_from_available_list() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    create_boat(&client, &base, "Alpha", "Light").await;
    create_boat(&client, &base, "Bravo", "Medium").await;

    client
        .goto(&format!("{base}/batches/new"))
        .await
        .unwrap();

    // Wait for the available boats list to load via HTMX.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath(
            "//*[@id='boat-search-results']//*[contains(text(), 'Alpha')]",
        ))
        .await
        .expect("Alpha should appear in available boats");

    // Add Alpha to the session via the API and wait for the response.
    client
        .execute(
            r##"await fetch("/api/batches/session/add/1", {method: "POST"});"##,
            vec![],
        )
        .await
        .unwrap();

    // Reload the page to see the updated state (cookie now has Alpha selected).
    client
        .goto(&format!("{base}/batches/new"))
        .await
        .unwrap();

    // Wait for the selected pane to render with Alpha.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath(
            "//*[@id='selected-boats-container']//*[contains(text(), 'Alpha')]",
        ))
        .await
        .expect("Alpha should appear in selected boats");

    // Alpha should NOT appear in the available boats (left).
    let available = client
        .find_all(Locator::XPath(
            "//*[@id='boat-search-results']//*[contains(text(), 'Alpha')]",
        ))
        .await
        .unwrap();
    assert_eq!(
        available.len(),
        0,
        "Alpha should not appear in available boats after selection"
    );

    // Bravo should still be in the available list.
    client
        .find(Locator::XPath(
            "//*[@id='boat-search-results']//*[contains(text(), 'Bravo')]",
        ))
        .await
        .expect("Bravo should still be in available boats");

    client.close().await.unwrap();
}

/// Search by name is infix (substring) and case-insensitive.
#[tokio::test]
async fn search_is_infix_case_insensitive() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    create_boat(&client, &base, "Falcon Sweep", "Heavy").await;
    create_boat(&client, &base, "River Runner", "Light").await;
    create_boat(&client, &base, "Old Falcon", "Medium").await;

    client
        .goto(&format!("{base}/batches/new"))
        .await
        .unwrap();

    // Wait for initial boat list.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath(
            "//*[@id='boat-search-results']//*[contains(text(), 'Falcon Sweep')]",
        ))
        .await
        .unwrap();

    // Search for "falcon" (infix, case-insensitive).
    client
        .execute(
            r##"fetch("/api/batches/search", {method: "POST", headers: {"Content-Type": "application/x-www-form-urlencoded"}, body: "search=falcon"}).then(r => r.text()).then(html => { document.querySelector("#boat-search-results").innerHTML = html; });"##,
            vec![],
        )
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Both "Falcon Sweep" (prefix) and "Old Falcon" (suffix) should match.
    let results = client.source().await.unwrap();
    assert!(
        results.contains("Falcon Sweep"),
        "prefix match 'Falcon Sweep' should appear"
    );
    assert!(
        results.contains("Old Falcon"),
        "suffix match 'Old Falcon' should appear"
    );
    assert!(
        !results.contains("River Runner"),
        "non-matching 'River Runner' should not appear in search results"
    );

    client.close().await.unwrap();
}

/// Creating a batch via the "Use as Template" button pre-selects the boats
/// from the original batch.
#[tokio::test]
async fn batch_template_preselects_boats() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    create_boat(&client, &base, "Template Boat A", "Light").await;
    create_boat(&client, &base, "Template Boat B", "Heavy").await;

    // Create a batch with both boats via API.
    client
        .execute(
            r##"fetch("/api/batches/session/add/1", {method: "POST"});"##,
            vec![],
        )
        .await
        .unwrap();
    client
        .execute(
            r##"fetch("/api/batches/session/add/2", {method: "POST"});"##,
            vec![],
        )
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Submit the batch via fetch (hx-post form, handler returns HX-Redirect).
    client
        .execute(
            r##"fetch("/batches", {method: "POST", headers: {"Content-Type": "application/x-www-form-urlencoded"}, body: "use_scenario_id=1&boat_ids%5B%5D=1&boat_ids%5B%5D=2"});"##,
            vec![],
        )
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Navigate to the batch detail page.
    client
        .goto(&format!("{base}/batches/1"))
        .await
        .unwrap();

    // Click "Use as Template".
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//a[contains(text(), 'Use as Template')]"))
        .await
        .unwrap();

    client
        .goto(&format!("{base}/batches/new?template=1"))
        .await
        .unwrap();

    // Both boats should be pre-selected in the right pane.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath(
            "//*[@id='selected-boats-container']//*[contains(text(), 'Template Boat A')]",
        ))
        .await
        .expect("Template Boat A should be pre-selected");

    client
        .find(Locator::XPath(
            "//*[@id='selected-boats-container']//*[contains(text(), 'Template Boat B')]",
        ))
        .await
        .expect("Template Boat B should be pre-selected");

    client.close().await.unwrap();
}

/// The submit button is disabled when no boats are selected.
#[tokio::test]
async fn cannot_submit_empty_batch() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    // Need at least one boat in the DB so the page loads normally.
    create_boat(&client, &base, "Lonely Boat", "Light").await;

    client
        .goto(&format!("{base}/batches/new"))
        .await
        .unwrap();

    // Wait for the page to load.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::XPath("//*[contains(text(), 'No boats selected yet')]"))
        .await
        .expect("should show empty selection message");

    // The submit button should be disabled.
    let disabled = client
        .execute(
            r##"return document.querySelector("button[type='submit']").disabled;"##,
            vec![],
        )
        .await
        .unwrap();
    assert_eq!(
        disabled,
        serde_json::Value::Bool(true),
        "submit button should be disabled with no boats selected"
    );

    client.close().await.unwrap();
}

/// Selecting a scenario with a known default time populates the datetime field.
#[tokio::test]
async fn scenario_default_populates_time() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;
    let base = instance.base_url();

    // Need a boat so the batch creation page loads.
    create_boat(&client, &base, "Time Boat", "Medium").await;

    client
        .goto(&format!("{base}/batches/new"))
        .await
        .unwrap();

    // Wait for page to load.
    client
        .wait()
        .at_most(std::time::Duration::from_secs(5))
        .for_element(Locator::Css("select[name='use_scenario_id']"))
        .await
        .unwrap();

    // Find a scenario with a default time. "Masters AM Practice" defaults to 05:30.
    // Get its ID from the dropdown options.
    let scenario_id = client
        .execute(
            r##"var opts = document.querySelectorAll("select[name='use_scenario_id'] option"); for (var o of opts) { if (o.textContent.includes("Masters AM")) return o.value; } return null;"##,
            vec![],
        )
        .await
        .unwrap();

    assert_ne!(
        scenario_id,
        serde_json::Value::Null,
        "Masters AM Practice scenario should exist"
    );

    // Select that scenario via Alpine.js x-model.
    let id_str = scenario_id.as_str().unwrap();
    client
        .execute(
            &format!(
                r##"var sel = document.querySelector("select[name='use_scenario_id']"); sel.value = "{}"; sel.dispatchEvent(new Event("change", {{bubbles: true}}));"##,
                id_str
            ),
            vec![],
        )
        .await
        .unwrap();

    // Give Alpine.js a moment to react.
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // The recorded_at field should now contain today's date + 05:30.
    let recorded_at = client
        .execute(
            r##"return document.querySelector("input[name='recorded_at']").value;"##,
            vec![],
        )
        .await
        .unwrap();

    let val = recorded_at.as_str().unwrap();
    assert!(
        val.contains("05:30"),
        "recorded_at should contain 05:30 from Masters AM default, got: {val}"
    );

    client.close().await.unwrap();
}
