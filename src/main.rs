mod clustering;
mod routing;
mod vehicle;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde::{Deserialize, Serialize};
use clustering::{ClusteringAlgorithm, KMeans};
use routing::{RoutingAlgorithm, TSP};

#[derive(Deserialize, Serialize, Clone)]
struct Vehicle { lat: f64, lon: f64, label: String }

#[derive(Deserialize)]
struct SolveRequest { vehicles: Vec<Vehicle>, zones: usize }

#[derive(Serialize)]
struct Zone { color: String, stops: Vec<Vehicle>, route: Vec<[f64; 2]>, distance_km: f64 }

#[derive(Serialize)]
struct SolveResponse { zones: Vec<Zone>, total_km: f64 }

const COLORS: &[&str] = &[
    "#e74c3c","#3498db","#2ecc71","#f39c12",
    "#9b59b6","#1abc9c","#e67e22","#34495e",
];

fn compute(req: SolveRequest) -> SolveResponse {
    let n = req.vehicles.len();
    if n == 0 { return SolveResponse { zones: vec![], total_km: 0.0 }; }
    let k = req.zones.clamp(1, n);
    let data: Vec<Vec<f64>> = req.vehicles.iter().map(|v| vec![v.lat, v.lon]).collect();
    let labels = KMeans::new(k).fit(&data);
    let mut zones = Vec::new();
    let mut total = 0.0;
    for z in 0..k {
        let idx: Vec<usize> = labels.iter().enumerate()
            .filter(|(_, &l)| l == z).map(|(i, _)| i).collect();
        if idx.is_empty() { continue; }
        let pts: Vec<Vec<f64>> = idx.iter().map(|&i| data[i].clone()).collect();
        let (order, dist) = TSP::new().solve(&pts);
        total += dist;
        zones.push(Zone {
            color: COLORS[zones.len() % COLORS.len()].into(),
            route: order.iter().map(|&r| [pts[r][0], pts[r][1]]).collect(),
            stops: order.iter().map(|&r| req.vehicles[idx[r]].clone()).collect(),
            distance_km: dist,
        });
    }
    SolveResponse { zones, total_km: total }
}

fn handle(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut stream = stream;

    let mut req_line = String::new();
    if reader.read_line(&mut req_line).is_err() { return; }

    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() { break; }
        if line == "\r\n" || line.is_empty() { break; }
        let lower = line.to_lowercase();
        if lower.starts_with("content-length:") {
            content_length = lower[15..].trim().parse().unwrap_or(0);
        }
    }

    let parts: Vec<&str> = req_line.splitn(3, ' ').collect();
    let method = parts.first().copied().unwrap_or("");
    let path   = parts.get(1).copied().unwrap_or("/");

    match (method, path) {
        ("GET", "/") => {
            let body = include_str!("../static/index.html");
            let _ = write!(stream,
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                body.len(), body);
        }
        ("POST", "/api/solve") => {
            let mut buf = vec![0u8; content_length];
            let _ = reader.read_exact(&mut buf);
            let json = match serde_json::from_slice::<SolveRequest>(&buf) {
                Ok(req) => serde_json::to_string(&compute(req)).unwrap_or_default(),
                Err(e)  => format!("{{\"error\":\"{e}\"}}"),
            };
            let _ = write!(stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                json.len(), json);
        }
        ("OPTIONS", _) => {
            let _ = stream.write_all(b"HTTP/1.1 204 No Content\r\nContent-Length: 0\r\n\r\n");
        }
        _ => {
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").expect("port 3000 in use");
    println!("Mappy running -> http://0.0.0.0:3000");
    for stream in listener.incoming().flatten() {
        thread::spawn(move || handle(stream));
    }
}
