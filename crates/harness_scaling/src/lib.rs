//! Scaling module - Optimized resource management
//! Features: Predictive scaling, circuit breaker, hysteresis

use std::collections::VecDeque;

/// Configuration for auto-scaling
#[derive(Debug, Clone)]
pub struct ScalingConfig {
    pub min_instances: u32,
    pub max_instances: u32,
    pub target_cpu_percent: f64,
    pub target_memory_percent: f64,
    pub scale_up_threshold: f64,
    pub scale_down_threshold: f64,
    pub cooldown_secs: u64,
}

impl Default for ScalingConfig {
    fn default() -> Self {
        Self {
            min_instances: 1,
            max_instances: 10,
            target_cpu_percent: 50.0,
            target_memory_percent: 70.0,
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
            cooldown_secs: 60,
        }
    }
}

/// Resource snapshot
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub instances: u32,
    pub timestamp: std::time::Instant,
}

/// Resource sampler with window
#[allow(dead_code)]
pub struct ResourceSampler {
    samples: VecDeque<f64>,
    window_seconds: u64,
    max_samples: usize,
}

impl ResourceSampler {
    pub fn new(window_seconds: u64) -> Self {
        // Assume 1 sample per second for window
        let max_samples = window_seconds as usize;
        Self {
            samples: VecDeque::with_capacity(max_samples),
            window_seconds,
            max_samples,
        }
    }

    #[inline]
    pub fn add(&mut self, value: f64) {
        if self.samples.len() >= self.max_samples {
            self.samples.pop_front();
        }
        self.samples.push_back(value);
    }

    #[inline]
    pub fn average(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.samples.iter().sum::<f64>() / self.samples.len() as f64
    }

    #[inline]
    pub fn max(&self) -> f64 {
        self.samples.iter().cloned().fold(0.0_f64, f64::max)
    }

    #[inline]
    pub fn min(&self) -> f64 {
        self.samples.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

/// Predictive scaling using linear regression
pub struct PredictiveScaler {
    history: VecDeque<(f64, f64)>, // (time, value)
    prediction_horizon: usize,
}

impl PredictiveScaler {
    pub fn new(horizon: usize) -> Self {
        Self {
            history: VecDeque::new(),
            prediction_horizon: horizon,
        }
    }

    pub fn add_sample(&mut self, value: f64) {
        let time = self.history.len() as f64;
        self.history.push_back((time, value));
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    /// Predict value at future time step
    pub fn predict(&self) -> Option<f64> {
        if self.history.len() < 2 {
            return None;
        }

        let n = self.history.len() as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;

        for (x, y) in &self.history {
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
        }

        let denominator = n * sum_xx - sum_x * sum_x;
        if denominator == 0.0 {
            return None;
        }

        let slope = (n * sum_xy - sum_x * sum_y) / denominator;
        let intercept = (sum_y - slope * sum_x) / n;

        let future_x = (self.history.len() + self.prediction_horizon) as f64;
        Some(slope * future_x + intercept)
    }
}

/// Calculate replicas with hysteresis
pub fn calculate_replicas(
    config: &ScalingConfig,
    cpu_percent: f64,
    memory_percent: f64,
    current: u32,
) -> u32 {
    let avg_load = (cpu_percent + memory_percent) / 2.0;

    let target = if avg_load > config.target_cpu_percent * config.scale_up_threshold {
        // Scale up
        (current as f64 * 1.5).ceil() as u32
    } else if avg_load < config.target_cpu_percent * config.scale_down_threshold {
        // Scale down
        (current as f64 * 0.7).floor() as u32
    } else {
        current
    };

    target.clamp(config.min_instances, config.max_instances)
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker for fault tolerance
pub struct CircuitBreaker {
    failure_count: u32,
    success_count: u32,
    failure_threshold: u32,
    success_threshold: u32,
    state: CircuitState,
    last_failure: Option<std::time::Instant>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32) -> Self {
        Self {
            failure_count: 0,
            success_count: 0,
            failure_threshold,
            success_threshold: 3,
            state: CircuitState::Closed,
            last_failure: None,
        }
    }

    #[inline]
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.success_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    #[inline]
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(std::time::Instant::now());

        match self.state {
            CircuitState::Closed => {
                if self.failure_count >= self.failure_threshold {
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
            }
            CircuitState::Open => {}
        }
    }

    #[inline]
    pub fn is_available(&self) -> bool {
        self.state != CircuitState::Open
    }

    pub fn state(&self) -> &CircuitState {
        &self.state
    }

    pub fn can_attempt(&self, retry_timeout_secs: u64) -> bool {
        if self.state != CircuitState::Open {
            return true;
        }

        if let Some(last) = self.last_failure {
            return last.elapsed().as_secs() >= retry_timeout_secs;
        }
        false
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(5)
    }
}

/// Token bucket rate limiter
pub struct TokenBucket {
    tokens: f64,
    capacity: f64,
    refill_rate: f64, // tokens per second
    last_refill: std::time::Instant,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            tokens: capacity,
            capacity,
            refill_rate,
            last_refill: std::time::Instant::now(),
        }
    }

    #[inline]
    pub fn try_acquire(&mut self, tokens: f64) -> bool {
        self.refill();
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    #[inline]
    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);
        self.last_refill = std::time::Instant::now();
    }
}
