// Résout le problème du facteur: ordre optimal de visite

// Distance haversine (km) - réaliste pour GPS
pub fn haversine(a: &[f64], b: &[f64]) -> f64 {
    let r = 6371.0;
    let (lat1, lon1) = (a[0].to_radians(), a[1].to_radians());
    let (lat2, lon2) = (b[0].to_radians(), b[1].to_radians());
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;
    let h = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin()
}

// Trait pour injection de dépendances
pub trait RoutingAlgorithm {
    fn solve(&self, points: &[Vec<f64>]) -> (Vec<usize>, f64);
    fn name(&self) -> &str;
}

// Nearest Neighbor + 2-opt
pub struct TSP;

impl TSP {
    pub fn new() -> Self { Self }

    fn nearest_neighbor(&self, pts: &[Vec<f64>]) -> Vec<usize> {
        let n = pts.len();
        if n == 0 { return vec![]; }
        let mut visited = vec![false; n];
        let mut route = vec![0];
        visited[0] = true;

        for _ in 1..n {
            let last = *route.last().unwrap();
            let next = (0..n)
                .filter(|&j| !visited[j])
                .min_by(|&a, &b| {
                    haversine(&pts[last], &pts[a])
                        .partial_cmp(&haversine(&pts[last], &pts[b]))
                        .unwrap()
                });
            if let Some(j) = next {
                route.push(j);
                visited[j] = true;
            }
        }
        route
    }

    fn route_length(&self, pts: &[Vec<f64>], route: &[usize]) -> f64 {
        route.windows(2)
            .map(|w| haversine(&pts[w[0]], &pts[w[1]]))
            .sum()
    }

    // 2-opt: améliore la route en inversant des segments
    fn two_opt(&self, pts: &[Vec<f64>], mut route: Vec<usize>) -> Vec<usize> {
        let n = route.len();
        if n < 4 { return route; }
        let mut improved = true;

        while improved {
            improved = false;
            for i in 1..n - 1 {
                for j in i + 1..n {
                    let mut new_route = route.clone();
                    new_route[i..=j].reverse();
                    if self.route_length(pts, &new_route)
                        < self.route_length(pts, &route)
                    {
                        route = new_route;
                        improved = true;
                    }
                }
            }
        }
        route
    }
}

impl RoutingAlgorithm for TSP {
    fn solve(&self, points: &[Vec<f64>]) -> (Vec<usize>, f64) {
        let route = self.nearest_neighbor(points);
        let route = self.two_opt(points, route);
        let dist = self.route_length(points, &route);
        (route, dist)
    }

    fn name(&self) -> &str { "TSP (NN + 2-opt)" }
}
