use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, HtmlElement, HtmlInputElement, Window};

/// Get the window object
fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

/// Get the document object
fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

/// Log a message to the browser console
#[wasm_bindgen]
pub fn log(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

/// Create a new HTML element and append it to the body
#[wasm_bindgen]
pub fn create_element(tag: &str, text: &str) -> Result<(), JsValue> {
    let document = document();
    let body = document.body().expect("document should have a body");

    let element = document.create_element(tag)?;
    element.set_text_content(Some(text));
    body.append_child(&element)?;

    Ok(())
}

/// Create a button with a click handler
#[wasm_bindgen]
pub fn create_button(text: &str, id: &str) -> Result<(), JsValue> {
    let document = document();
    let body = document.body().expect("document should have a body");

    let button = document.create_element("button")?;
    button.set_text_content(Some(text));
    button.set_id(id);

    // Add some styling
    let html_element: HtmlElement = button.dyn_into()?;
    let style = html_element.style();
    style.set_property("margin", "10px")?;
    style.set_property("padding", "10px 20px")?;
    style.set_property("font-size", "16px")?;
    style.set_property("cursor", "pointer")?;

    body.append_child(&html_element)?;

    Ok(())
}

/// Get element by ID and update its text content
#[wasm_bindgen]
pub fn update_element(id: &str, new_text: &str) -> Result<(), JsValue> {
    let document = document();
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element with id '{}' not found", id)))?;

    element.set_text_content(Some(new_text));
    Ok(())
}

/// Remove element by ID
#[wasm_bindgen]
pub fn remove_element(id: &str) -> Result<(), JsValue> {
    let document = document();
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element with id '{}' not found", id)))?;

    element.remove();
    Ok(())
}

/// Create an input field
#[wasm_bindgen]
pub fn create_input(id: &str, placeholder: &str) -> Result<(), JsValue> {
    let document = document();
    let body = document.body().expect("document should have a body");

    let input = document.create_element("input")?;
    input.set_id(id);

    let html_input: HtmlInputElement = input.dyn_into()?;
    html_input.set_placeholder(placeholder);

    // Add styling
    let style = html_input.style();
    style.set_property("margin", "10px")?;
    style.set_property("padding", "8px")?;
    style.set_property("font-size", "14px")?;

    body.append_child(&html_input)?;

    Ok(())
}

/// Get value from input field
#[wasm_bindgen]
pub fn get_input_value(id: &str) -> Result<String, JsValue> {
    let document = document();
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element with id '{}' not found", id)))?;

    let input: HtmlInputElement = element.dyn_into()?;
    Ok(input.value())
}

/// Create a list with items
#[wasm_bindgen]
pub fn create_list(items: Vec<JsValue>) -> Result<(), JsValue> {
    let document = document();
    let body = document.body().expect("document should have a body");

    let ul = document.create_element("ul")?;

    for item in items {
        let li = document.create_element("li")?;
        if let Some(text) = item.as_string() {
            li.set_text_content(Some(&text));
        }
        ul.append_child(&li)?;
    }

    body.append_child(&ul)?;
    Ok(())
}

/// Add CSS class to element
#[wasm_bindgen]
pub fn add_class(id: &str, class_name: &str) -> Result<(), JsValue> {
    let document = document();
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element with id '{}' not found", id)))?;

    element.class_list().add_1(class_name)?;
    Ok(())
}

/// Remove CSS class from element
#[wasm_bindgen]
pub fn remove_class(id: &str, class_name: &str) -> Result<(), JsValue> {
    let document = document();
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element with id '{}' not found", id)))?;

    element.class_list().remove_1(class_name)?;
    Ok(())
}

/// Toggle CSS class on element
#[wasm_bindgen]
pub fn toggle_class(id: &str, class_name: &str) -> Result<(), JsValue> {
    let document = document();
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element with id '{}' not found", id)))?;

    element.class_list().toggle(class_name)?;
    Ok(())
}

/// Set innerHTML (use with caution - can be XSS vulnerable)
#[wasm_bindgen]
pub fn set_inner_html(id: &str, html: &str) -> Result<(), JsValue> {
    let document = document();
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element with id '{}' not found", id)))?;

    element.set_inner_html(html);
    Ok(())
}
