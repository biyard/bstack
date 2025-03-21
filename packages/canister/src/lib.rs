use bcdk::*;
use http_gateway::*;

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
    Response::build(serde_json::json!({
        "method": "POST",
    }))
}

fn version() -> serde_json::Value {
    serde_json::json!({
        "version": "v0.1.0",
    })
}

fn get() -> serde_json::Value {
    serde_json::json!({
        "method": "GET",
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
