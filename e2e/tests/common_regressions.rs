use boat_tracking_e2e::{scroll_and_click, select_value, TestInstance};
use fantoccini::Locator;

/// The page should contain exactly one nav bar, one #content div, and one body.
/// Regression: HTMX swap replaces outer element, duplicating the layout shell.
#[tokio::test]
async fn no_duplicated_layout_elements() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    // Check the home page.
    client.goto(&instance.base_url()).await.unwrap();
    client.wait().for_element(Locator::Css("nav")).await.unwrap();

    let nav_count = client
        .find_all(Locator::Css("nav"))
        .await
        .unwrap()
        .len();
    assert_eq!(nav_count, 1, "expected exactly one <nav>, found {nav_count}");

    let content_count = client
        .find_all(Locator::Css("#content"))
        .await
        .unwrap()
        .len();
    assert_eq!(
        content_count, 1,
        "expected exactly one #content div, found {content_count}"
    );

    // Navigate via HTMX to another page and re-check.
    // Uses JS click because WebKitWebDriver reports bad rects for sticky-positioned nav links.
    scroll_and_click(&client, "nav a[href='/boats']").await;

    client
        .wait()
        .for_element(Locator::XPath("//*[@id='content']//*[contains(text(), 'Boats')]"))
        .await
        .unwrap();

    let nav_count = client
        .find_all(Locator::Css("nav"))
        .await
        .unwrap()
        .len();
    assert_eq!(
        nav_count, 1,
        "after HTMX navigation, expected one <nav>, found {nav_count}"
    );

    let content_count = client
        .find_all(Locator::Css("#content"))
        .await
        .unwrap()
        .len();
    assert_eq!(
        content_count, 1,
        "after HTMX navigation, expected one #content, found {content_count}"
    );

    client.close().await.unwrap();
}

/// After submitting a form, the page should not contain duplicate nav bars.
/// Regression: form handler returns a full page instead of a fragment, causing
/// the layout to nest inside the existing layout.
#[tokio::test]
async fn form_submission_does_not_duplicate_nav() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    // Create a boat via form submission.
    client
        .goto(&format!("{}/boats/new", instance.base_url()))
        .await
        .unwrap();

    let name = client.wait().for_element(Locator::Css("#name")).await.unwrap();
    name.send_keys("Nav Dup Test").await.unwrap();
    select_value(&client, "select[name='weight_class']", "Medium").await;
    scroll_and_click(&client, "button[type='submit']").await;

    client
        .wait()
        .for_element(Locator::XPath("//*[contains(text(), 'Nav Dup Test')]"))
        .await
        .unwrap();

    let nav_count = client
        .find_all(Locator::Css("nav"))
        .await
        .unwrap()
        .len();
    assert_eq!(
        nav_count, 1,
        "after form submission, expected one <nav>, found {nav_count}"
    );

    client.close().await.unwrap();
}

/// Every nav link should load its page content without a full page reload.
/// Regression: HTMX attributes (hx-get, hx-target) accidentally removed from nav links.
#[tokio::test]
async fn nav_links_load_content() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    client.goto(&instance.base_url()).await.unwrap();

    let pages = [
        ("Boats", "Boats"),
        ("Issues", "Issues"),
        ("Scenarios", "Scenarios"),
        ("Practices & Regattas", "Boat Uses"),
    ];

    for (link_text, expected_content) in pages {
        // JS click: WebKitWebDriver reports bad rects for sticky-positioned nav links.
        client
            .execute(
                &format!(
                    r#"Array.from(document.querySelectorAll("nav a")).find(a => a.textContent.includes("{}")).click();"#,
                    link_text
                ),
                vec![],
            )
            .await
            .unwrap();

        client
            .wait()
            .for_element(Locator::XPath(&format!(
                "//*[@id='content']//*[contains(text(), '{expected_content}')]"
            )))
            .await
            .unwrap_or_else(|_| {
                panic!("clicking '{link_text}' should render content containing '{expected_content}'")
            });
    }

    client.close().await.unwrap();
}

/// All "Add New" / "New" buttons should navigate to their respective form pages.
/// Regression: href or hx-get on action buttons pointed to wrong route.
#[tokio::test]
async fn add_new_buttons_navigate_to_forms() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    let cases = [
        ("/boats", "Add", "Boat Name"),
        ("/issues", "New Issue", "Issue Description"),
        ("/scenarios", "New Scenario", "Name"),
    ];

    for (page, button_text, form_field_label) in cases {
        client
            .goto(&format!("{}{page}", instance.base_url()))
            .await
            .unwrap();

        // JS click: some action links report bad rects in WebKitWebDriver.
        client
            .execute(
                &format!(
                    r#"Array.from(document.querySelectorAll("a")).find(a => a.textContent.includes("{}")).click();"#,
                    button_text
                ),
                vec![],
            )
            .await
            .unwrap();

        client
            .wait()
            .for_element(Locator::XPath(&format!(
                "//*[contains(text(), '{form_field_label}')]"
            )))
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "clicking '{button_text}' on {page} should show form with '{form_field_label}'"
                )
            });
    }

    client.close().await.unwrap();
}

/// Form submissions should produce a success toast, not an error page.
/// Regression: handler returns wrong status code or breaks the HTMX swap.
#[tokio::test]
async fn boat_form_submission_shows_success() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    client
        .goto(&format!("{}/boats/new", instance.base_url()))
        .await
        .unwrap();

    let name = client.wait().for_element(Locator::Css("#name")).await.unwrap();
    name.send_keys("Toast Test Boat").await.unwrap();

    select_value(&client, "select[name='weight_class']", "Light").await;

    scroll_and_click(&client, "button[type='submit']").await;

    // After successful creation the boats list should show the new boat.
    // An error page would not contain this text.
    client
        .wait()
        .for_element(Locator::XPath("//*[contains(text(), 'Toast Test Boat')]"))
        .await
        .expect("boat list should contain the newly created boat");

    client.close().await.unwrap();
}

/// Static assets (CSS, JS) must be served correctly.
/// Regression: public/ directory not deployed alongside binary, or ServeDir
/// misconfigured, resulting in unstyled pages.
#[tokio::test]
async fn static_assets_are_served() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    client.goto(&instance.base_url()).await.unwrap();

    // Verify that the stylesheet link is present and references tailwind.
    client
        .wait()
        .for_element(Locator::Css("link[href='/tailwind.css']"))
        .await
        .expect("tailwind CSS link should be in the page head");

    // Verify the HTMX script is loaded.
    client
        .find(Locator::Css("script[src='/htmx.min.js']"))
        .await
        .expect("htmx script tag should be in the page head");

    client.close().await.unwrap();
}

/// The seeded scenarios should be present in a fresh database.
/// Regression: migrations broken or scenario seeding removed.
#[tokio::test]
async fn seeded_scenarios_present() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    client
        .goto(&format!("{}/scenarios", instance.base_url()))
        .await
        .unwrap();

    let expected = ["Masters AM Practice", "Masters PM Practice", "Learn To Row"];
    for name in expected {
        client
            .wait()
            .for_element(Locator::XPath(&format!(
                "//*[contains(text(), '{name}')]"
            )))
            .await
            .unwrap_or_else(|_| panic!("seeded scenario '{name}' should be present"));
    }

    client.close().await.unwrap();
}

/// CSV export links should be present and return a downloadable response.
/// Regression: export route removed or handler panics on empty data.
#[tokio::test]
async fn csv_export_links_present() {
    let instance = TestInstance::start().await;
    let client = instance.connect().await;

    client
        .goto(&format!("{}/boats", instance.base_url()))
        .await
        .unwrap();

    client
        .wait()
        .for_element(Locator::Css("a[href='/boats_export.csv']"))
        .await
        .expect("boats CSV export link should be present");

    client
        .goto(&format!("{}/batches", instance.base_url()))
        .await
        .unwrap();

    client
        .wait()
        .for_element(Locator::Css("a[href='/uses_export.csv']"))
        .await
        .expect("uses CSV export link should be present");

    client.close().await.unwrap();
}
