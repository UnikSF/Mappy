pub trait ClusteringAlgorithm {
    fn fit(&self, data: &[Vec<f64>]) -> Vec<usize>;
    fn name(&self) -> &str;
}

pub struct KMeans {
    pub k: usize,
}

impl KMeans {
    pub fn new(k: usize) -> Self { Self { k } }
}

impl ClusteringAlgorithm for KMeans {
    fn fit(&self, data: &[Vec<f64>]) -> Vec<usize> {
        let n = data.len();
        if n == 0 { return vec![]; }
        let k = self.k.min(n);
        let mut centroids: Vec<Vec<f64>> = data[..k].to_vec();
        let mut labels = vec![0usize; n];

        for _ in 0..100 {
            let mut changed = false;
            for (i, point) in data.iter().enumerate() {
                let nearest = (0..k)
                    .min_by(|&a, &b| {
                        dist(point, &centroids[a])
                            .partial_cmp(&dist(point, &centroids[b]))
                            .unwrap()
                    })
                    .unwrap();
                if labels[i] != nearest { labels[i] = nearest; changed = true; }
            }
            if !changed { break; }

            let dims = data[0].len();
            for c in 0..k {
                let members: Vec<&Vec<f64>> = data.iter().enumerate()
                    .filter(|(i, _)| labels[*i] == c)
                    .map(|(_, v)| v)
                    .collect();
                if members.is_empty() { continue; }
                centroids[c] = (0..dims)
                    .map(|d| members.iter().map(|v| v[d]).sum::<f64>() / members.len() as f64)
                    .collect();
            }
        }
        labels
    }

    fn name(&self) -> &str { "K-Means" }
}

fn dist(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}
