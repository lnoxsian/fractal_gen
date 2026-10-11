use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[inline(always)]
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x.clamp(-30.0, 30.0)).exp())
}

#[inline(always)]
fn d_sigmoid(y: f64) -> f64 {
    y * (1.0 - y)
}

/// 11 inputs -> 32 hidden -> 16 hidden -> 1 output MLP neural net
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryNeuralNet {
    // Layer 1: 11 -> 32
    pub w1: Vec<Vec<f64>>,
    pub b1: Vec<f64>,
    // Layer 2: 32 -> 16
    pub w2: Vec<Vec<f64>>,
    pub b2: Vec<f64>,
    // Layer 3: 16 -> 1
    pub w3: Vec<f64>,
    pub b3: f64,
}

impl DiscoveryNeuralNet {
    pub fn new() -> Self {
        let mut rng = SmallRng::seed_from_u64(42);
        let mut rand_weight = |fan_in: usize| {
            let scale = (2.0 / fan_in as f64).sqrt();
            rng.gen_range(-1.0..1.0) * scale
        };

        let mut w1 = Vec::with_capacity(32);
        for _ in 0..32 {
            let mut row = Vec::with_capacity(11);
            for _ in 0..11 {
                row.push(rand_weight(11));
            }
            w1.push(row);
        }
        let b1 = vec![0.0; 32];

        let mut w2 = Vec::with_capacity(16);
        for _ in 0..16 {
            let mut row = Vec::with_capacity(32);
            for _ in 0..32 {
                row.push(rand_weight(32));
            }
            w2.push(row);
        }
        let b2 = vec![0.0; 16];

        let mut w3 = Vec::with_capacity(16);
        for _ in 0..16 {
            w3.push(rand_weight(16));
        }
        let b3 = 0.0;

        Self {
            w1,
            b1,
            w2,
            b2,
            w3,
            b3,
        }
    }

    /// Forward inference returning a score in [0.0, 1.0]
    pub fn predict(&self, input: &[f64; 11]) -> f64 {
        // Hidden layer 1
        let mut h1 = [0.0; 32];
        for i in 0..32 {
            let mut sum = self.b1[i];
            for j in 0..11 {
                sum += self.w1[i][j] * input[j];
            }
            h1[i] = sigmoid(sum);
        }

        // Hidden layer 2
        let mut h2 = [0.0; 16];
        for i in 0..16 {
            let mut sum = self.b2[i];
            for j in 0..32 {
                sum += self.w2[i][j] * h1[j];
            }
            h2[i] = sigmoid(sum);
        }

        // Output layer
        let mut out_sum = self.b3;
        for i in 0..16 {
            out_sum += self.w3[i] * h2[i];
        }
        sigmoid(out_sum)
    }

    /// Train one iteration on an input vector and target [0.0 or 1.0]
    pub fn train_step(&mut self, input: &[f64; 11], target: f64, lr: f64) -> f64 {
        // Forward pass
        let mut z1 = [0.0; 32];
        let mut h1 = [0.0; 32];
        for i in 0..32 {
            let mut sum = self.b1[i];
            for j in 0..11 {
                sum += self.w1[i][j] * input[j];
            }
            z1[i] = sum;
            h1[i] = sigmoid(sum);
        }

        let mut z2 = [0.0; 16];
        let mut h2 = [0.0; 16];
        for i in 0..16 {
            let mut sum = self.b2[i];
            for j in 0..32 {
                sum += self.w2[i][j] * h1[j];
            }
            z2[i] = sum;
            h2[i] = sigmoid(sum);
        }

        let mut out_sum = self.b3;
        for i in 0..16 {
            out_sum += self.w3[i] * h2[i];
        }
        let output = sigmoid(out_sum);
        let error = target - output;

        // Backward pass
        let d_out = error * d_sigmoid(output);

        let mut d_h2 = [0.0; 16];
        for i in 0..16 {
            d_h2[i] = d_out * self.w3[i] * d_sigmoid(h2[i]);
            self.w3[i] += lr * d_out * h2[i];
        }
        self.b3 += lr * d_out;

        let mut d_h1 = [0.0; 32];
        for j in 0..32 {
            let mut sum = 0.0;
            for i in 0..16 {
                sum += d_h2[i] * self.w2[i][j];
            }
            d_h1[j] = sum * d_sigmoid(h1[j]);
        }

        for i in 0..16 {
            for j in 0..32 {
                self.w2[i][j] += lr * d_h2[i] * h1[j];
            }
            self.b2[i] += lr * d_h2[i];
        }

        for i in 0..32 {
            for j in 0..11 {
                self.w1[i][j] += lr * d_h1[i] * input[j];
            }
            self.b1[i] += lr * d_h1[i];
        }

        error.abs()
    }
}

impl Default for DiscoveryNeuralNet {
    fn default() -> Self {
        Self::new()
    }
}
