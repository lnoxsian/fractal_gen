use crate::color::ColorScheme;
use crate::core::config::FractalConfig;
use crate::core::renderer::FractalRenderer;
use crate::discovery::{discover_surprise_me, DiscoveryOptions, FavoritesManager};
use crate::fractals::get_all_fractals;
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Method, Response, Server};

pub fn start_server(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{host}:{port}");
    let server = Server::http(&addr).map_err(|e| format!("Failed to bind to {addr}: {e}"))?;
    println!(" FractalAI Web Explorer running at http://{addr}");
    println!(" Press Ctrl+C to stop the server.");

    let renderer = Arc::new(FractalRenderer::new());
    let fav_path = FavoritesManager::default_storage_path();
    let favorites_mgr = Arc::new(Mutex::new(
        FavoritesManager::load_from_file(&fav_path).unwrap_or_default(),
    ));

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let (path, query) = match url.split_once('?') {
            Some((p, q)) => (p, q),
            None => (url.as_str(), ""),
        };

        match (request.method(), path) {
            (&Method::Get, "/") | (&Method::Get, "/index.html") => {
                let html = include_str!("index.html");
                let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
                let mut resp = Response::from_string(html);
                resp.add_header(header);
                let _ = request.respond(resp);
            }
            (&Method::Get, "/api/fractals") => {
                let list: Vec<serde_json::Value> = get_all_fractals()
                    .into_iter()
                    .map(|f| {
                        let b = f.bounds();
                        serde_json::json!({
                            "id": f.id(),
                            "name": f.name(),
                            "family": f.family(),
                            "formula": f.formula(),
                            "description": f.description(),
                            "bounds": b,
                        })
                    })
                    .collect();
                let json = serde_json::to_string(&list).unwrap_or_default();
                let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                let mut resp = Response::from_string(json);
                resp.add_header(header);
                let _ = request.respond(resp);
            }
            (&Method::Get, "/api/colors") => {
                let list: Vec<serde_json::Value> = ColorScheme::ALL
                    .iter()
                    .map(|c| {
                        serde_json::json!({
                            "index": c.index(),
                            "id": c.identifier(),
                            "description": c.description(),
                        })
                    })
                    .collect();
                let json = serde_json::to_string(&list).unwrap_or_default();
                let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                let mut resp = Response::from_string(json);
                resp.add_header(header);
                let _ = request.respond(resp);
            }
            (&Method::Get, "/api/render") => {
                let config = parse_config_from_query(query);
                let width = parse_query_param(query, "width").unwrap_or(800.0) as u32;
                let height = parse_query_param(query, "height").unwrap_or(600.0) as u32;

                let buffer = renderer.render(&config, width.min(8192), height.min(8192));
                match buffer.to_png_bytes() {
                    Ok(png_bytes) => {
                        let header = Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..]).unwrap();
                        let mut resp = Response::from_data(png_bytes);
                        resp.add_header(header);
                        if parse_query_str(query, "download").is_some() {
                            let filename = format!("{}_{}x{}.png", config.fractal_type, width, height);
                            let cd_val = format!("attachment; filename=\"{filename}\"");
                            if let Ok(cd_header) = Header::from_bytes(&b"Content-Disposition"[..], cd_val.as_bytes()) {
                                resp.add_header(cd_header);
                            }
                        }
                        let _ = request.respond(resp);
                    }
                    Err(e) => {
                        let _ = request.respond(Response::from_string(format!("Error rendering: {e}")).with_status_code(500));
                    }
                }
            }
            (&Method::Get, "/api/surprise-me") => {
                let config = parse_config_from_query(query);
                let options = DiscoveryOptions::default();
                let mgr = favorites_mgr.lock().unwrap();
                let discovered = discover_surprise_me(&config, &options, mgr.model.as_ref());

                let resp_json = match discovered {
                    Some(candidate) => serde_json::json!({
                        "status": "success",
                        "score": candidate.score,
                        "heuristicScore": candidate.heuristic_score,
                        "mlScore": candidate.ml_score,
                        "config": {
                            "fractal": candidate.config.fractal_type,
                            "zoom": candidate.config.zoom,
                            "offsetX": candidate.config.offset_x,
                            "offsetY": candidate.config.offset_y,
                            "iterations": candidate.config.iterations,
                            "color": candidate.config.color_scheme.identifier(),
                            "juliaCX": candidate.config.julia_cx,
                            "juliaCY": candidate.config.julia_cy,
                        }
                    }),
                    None => serde_json::json!({
                        "status": "not_found",
                        "message": "No interesting candidate discovered matching criteria",
                    }),
                };

                let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                let mut resp = Response::from_string(resp_json.to_string());
                resp.add_header(header);
                let _ = request.respond(resp);
            }
            (&Method::Get, "/api/favorites") => {
                let mgr = favorites_mgr.lock().unwrap();
                let json = serde_json::to_string(&mgr.favorites).unwrap_or_default();
                let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                let mut resp = Response::from_string(json);
                resp.add_header(header);
                let _ = request.respond(resp);
            }
            (&Method::Post, "/api/favorites") => {
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);
                let config: Result<FractalConfig, _> = serde_json::from_str(&body);
                match config {
                    Ok(c) => {
                        let mut mgr = favorites_mgr.lock().unwrap();
                        mgr.add_favorite(&c);
                        let _ = mgr.save_to_file(&fav_path);
                        let resp = Response::from_string("{\"status\":\"saved\"}");
                        let _ = request.respond(resp);
                    }
                    Err(e) => {
                        let _ = request.respond(Response::from_string(format!("Invalid JSON: {e}")).with_status_code(400));
                    }
                }
            }
            _ => {
                let _ = request.respond(Response::from_string("Not Found").with_status_code(404));
            }
        }
    }
    Ok(())
}

fn parse_query_param(query: &str, key: &str) -> Option<f64> {
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == key {
                return v.parse::<f64>().ok();
            }
        }
    }
    None
}

fn parse_query_str<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == key {
                return Some(v);
            }
        }
    }
    None
}

fn parse_config_from_query(query: &str) -> FractalConfig {
    let mut config = FractalConfig::default();
    if let Some(f) = parse_query_str(query, "fractal") {
        config.fractal_type = f.to_string();
    }
    if let Some(c) = parse_query_str(query, "color") {
        if let Ok(scheme) = c.parse::<ColorScheme>() {
            config.color_scheme = scheme;
        }
    }
    if let Some(z) = parse_query_param(query, "zoom") {
        config.zoom = z;
    }
    if let Some(ox) = parse_query_str(query, "offset_x") {
        config.offset_x_str = Some(ox.to_string());
        if let Ok(v) = ox.parse::<f64>() {
            config.offset_x = v;
        }
    }
    if let Some(oy) = parse_query_str(query, "offset_y") {
        config.offset_y_str = Some(oy.to_string());
        if let Ok(v) = oy.parse::<f64>() {
            config.offset_y = v;
        }
    }
    if let Some(it) = parse_query_param(query, "iterations") {
        config.iterations = it as u32;
    }
    if let Some(cx) = parse_query_param(query, "julia_cx") {
        config.julia_cx = cx;
    }
    if let Some(cy) = parse_query_param(query, "julia_cy") {
        config.julia_cy = cy;
    }
    config
}
