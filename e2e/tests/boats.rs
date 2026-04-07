use boat_tracking_e2e::{scroll_and_click, select_value, set_input_value, TestInstance};
use fantoccini::Locator;

/// Creates a boat via the form and verifies it appears in the boat list.
#[tokio::test]
async fn create_boat() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    client
        .goto(&format!("{}/boats/new", instance.base_url()))
        .await
        .unwrap();

    let name_field = client.wait().for_element(Locator::Css("#name")).await.unwrap();
    name_field.send_keys("Test Sculler").await.unwrap();

    select_value(&client, "select[name='weight_class']", "Light").await;
    select_value(&client, "select[name='boat_type']", "Single").await;

    scroll_and_click(&client, "button[type='submit']").await;

    client
        .wait()
        .for_element(Locator::XPath("//*[contains(text(), 'Test Sculler')]"))
        .await
        .expect("new boat should appear in the list");

    client.close().await.unwrap();
}

/// Creates a boat, navigates to its edit page, renames it, and verifies the update.
#[tokio::test]
async fn edit_boat() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    // Create a boat.
    client
        .goto(&format!("{}/boats/new", instance.base_url()))
        .await
        .unwrap();

    let name_field = client.wait().for_element(Locator::Css("#name")).await.unwrap();
    name_field.send_keys("Original Name").await.unwrap();
    select_value(&client, "select[name='weight_class']", "Heavy").await;
    scroll_and_click(&client, "button[type='submit']").await;

    // Wait for list to confirm creation.
    client
        .wait()
        .for_element(Locator::XPath("//*[contains(text(), 'Original Name')]"))
        .await
        .expect("boat should appear in list");

    // Navigate to the boat's edit page directly (boat ID 1 for first created boat).
    client
        .goto(&format!("{}/boats/1/edit", instance.base_url()))
        .await
        .unwrap();

    // Clear the name field and enter a new name.
    client.wait().for_element(Locator::Css("#name")).await.unwrap();
    set_input_value(&client, "#name", "Renamed Boat").await;

    scroll_and_click(&client, "button[type='submit']").await;

    // Verify the renamed boat appears in the list.
    client
        .wait()
        .for_element(Locator::XPath("//*[contains(text(), 'Renamed Boat')]"))
        .await
        .expect("renamed boat should appear in the list");

    client.close().await.unwrap();
}
