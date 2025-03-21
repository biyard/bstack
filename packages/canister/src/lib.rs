use std::cell::RefCell;

use bcdk::*;
use http_gateway::*;

thread_local! {
    static STATE: RefCell<u64> = RefCell::default();
}

#[query]
async fn http_request(req: Request) -> Response {
    if &req.method == "POST" {
        return Response::upgrade();
    }

    if req.url == "/version" {
        Response::build(version())
    } else {
        Response::build(get())
    }
}

#[update]
async fn http_request_update(_req: Request) -> Response {
    let v = STATE.with_borrow_mut(|v| {
        *v += 1;
        *v
    });

    Response::build(serde_json::json!({
        "method": "POST",
        "value": v
    }))
}

fn version() -> serde_json::Value {
    serde_json::json!({
        "version": "v0.1.0",
    })
}

fn get() -> serde_json::Value {
    STATE.with(|v| {
        serde_json::json!({
            "method": "GET",
            "value": v
        })
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|s| {
        storage::stable_save((s.clone(),)).expect("Failed to save stable memory");
    });
}

// Restore state after upgrade
#[post_upgrade]
fn post_upgrade() {
    let (saved_state,): (u64,) = storage::stable_restore().unwrap_or_default();
    STATE.with_borrow_mut(|s| {
        *s = saved_state;
    });
}

export_candid!();
