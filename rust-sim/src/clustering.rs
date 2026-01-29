#[allow(dead_code)]

use crate::node::Node;
use glam::Vec2;
use rand::seq::index::sample;

const MAX_ITER: usize = 100;
const EPS: f32 = 1e-4;

pub(crate) struct KMeans{
    n_clusters: usize,
    centroids: Vec<Vec2>,
    clusters: Vec<usize>,
} 

impl KMeans{
    pub fn new(n_clusters: usize) -> Self{
        Self{
            n_clusters,
            centroids: Vec::new(),
            clusters: Vec::new()
        }
    }

    pub fn centroids(&self) -> &Vec<Vec2>{
        &self.centroids
    }

    pub fn clusters(&self) -> &Vec<usize>{
        &self.clusters
    }


    fn update_centroids(&mut self, wsn: &[Node]) -> Vec<Vec2>{
        let mut accum = vec![(Vec2::new(0.0, 0.0), 0usize); self.n_clusters];
        let previous_centroids = self.centroids.clone();

        // accumulate sums
        for (i, &c_id) in self.clusters.iter().enumerate() {
            accum[c_id].0 += wsn[i].position;
            accum[c_id].1 += 1;
        }

        // update centroids
        for (i, (sum, count)) in accum.into_iter().enumerate() {
            if count > 0 {
                self.centroids[i] = sum / count as f32;
            }
        }
        previous_centroids
    }

    pub fn fit(&mut self, wsn: &Vec<Node>){
        let mut rng = rand::rng();
        self.centroids = sample(&mut rng, wsn.len(), self.n_clusters)
            .into_iter()
            .map(|x| wsn[x].position)
            .collect();

        self.clusters = vec![0;wsn.len()];
        for _ in 0..MAX_ITER {
            for i in 0..wsn.len(){
                let mut min_dist = f32::INFINITY;
                for (index,&centroid) in self.centroids.iter().enumerate(){

                    let dist = (wsn[i].position - centroid).length();

                    if dist < min_dist{
                        min_dist = dist;
                        self.clusters[i] = index;
                    }
                }
            }

            let prev_centroids = self.update_centroids(&wsn);

            let mut max_shift: f32 = 0.0;
            for (a, b) in prev_centroids.iter().zip(self.centroids.iter()) {
                max_shift = max_shift.max((*a - *b).length());
            }

            if max_shift < EPS {
                break;
            }
        }
    }
}
