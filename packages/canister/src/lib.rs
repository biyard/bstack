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

// fn matcher(url: String, route: String) -> (bool, HashMap<String, String>) {
//     let parts: Vec<_> = url.split("?").collect();
//     let url = parts[0].split("/");
//     let route = route.split("/");
//     let mut m = HashMap::new();

//     for (u, r) in url.zip(route) {
//         if r.starts_with(":") {
//             m.insert(r.replace(":", ""), u.to_string());
//         } else if u != r {
//             return (false, m);
//         }
//     }

//     (true, m)
// }

export_candid!();
