use playground::example_hub;
use serde_json;
use signalrs_core::protocol;
use tokio;

#[tokio::test]
async fn example_hub_simple_invocation_succesfull() {
    let hub = example_hub::HubInvoker::new();

    let invocation =
        protocol::Invocation::new(Some("123".to_string()), "add".to_string(), Some((1, 2)));
    let request = serde_json::to_string(&invocation).unwrap();

    let response = hub.invoke_text(&request).await;

    dbg!(response.clone());

    let response = response.unwrap_single();
    let response: protocol::Completion<i32> = serde_json::from_str(&response).unwrap();

    let expected_response = protocol::Completion::new("123".to_string(), Some(3), None);

    assert_eq!(expected_response, response);
}

#[tokio::test]
async fn example_hub_simple_invocation_failed() {
    let hub = example_hub::HubInvoker::new();

    let invocation = protocol::Invocation::new(
        Some("123".to_string()),
        "single_result_failure".to_string(),
        Some((1, 2)),
    );
    let request = serde_json::to_string(&invocation).unwrap();

    let response = hub.invoke_text(&request).await;

    dbg!(response.clone());

    let response = response.unwrap_single();
    let response: protocol::Completion<i32> = serde_json::from_str(&response).unwrap();

    let expected_response =
        protocol::Completion::new("123".to_string(), None, Some("An error!".to_string()));

    assert_eq!(expected_response, response);
}

#[tokio::test]
async fn example_hub_batched_invocation() {
    let hub = example_hub::HubInvoker::new();

    let invocation =
        protocol::Invocation::new(Some("123".to_string()), "batched".to_string(), Some((5usize, ())));
    let request = serde_json::to_string(&invocation).unwrap();

    let response = hub.invoke_text(&request).await;

    dbg!(response.clone());

    let response = response.unwrap_single();
    let response: protocol::Completion<Vec<usize>> = serde_json::from_str(&response).unwrap();

    let expected_response =
        protocol::Completion::new("123".to_string(), Some(vec![0, 1, 2, 3, 4]), None);

    assert_eq!(expected_response, response);
}
