# Mojo statistics module - Statistical functions and distributions

from math import exp, log, sqrt, pi, erf

# Normal (Gaussian) distribution
fn normal_pdf(x: Float64, mu: Float64, sigma: Float64) -> Float64:
    let coeff = 1.0 / (sigma * sqrt(2.0 * pi))
    let exponent = -0.5 * ((x - mu) / sigma) ** 2
    return coeff * exp(exponent)

fn normal_cdf(x: Float64, mu: Float64, sigma: Float64) -> Float64:
    return 0.5 * (1.0 + erf((x - mu) / (sigma * sqrt(2.0))))

# Exponential distribution
fn exponential_pdf(x: Float64, lambda: Float64) -> Float64:
    if x < 0.0: return 0.0
    return lambda * exp(-lambda * x)

fn exponential_cdf(x: Float64, lambda: Float64) -> Float64:
    if x < 0.0: return 0.0
    return 1.0 - exp(-lambda * x)

# Poisson distribution
fn poisson_pmf(k: Int, lambda: Float64) -> Float64:
    if k < 0: return 0.0
    # P(X=k) = (lambda^k * e^(-lambda)) / k!
    var result = exp(-lambda)
    for i in range(1, k + 1):
        result = result * lambda / Float64(i)
    return result

# Binomial distribution
fn binomial_pmf(k: Int, n: Int, p: Float64) -> Float64:
    if k < 0 or k > n: return 0.0
    # Using approximation for large n
    let mu = Float64(n) * p
    let sigma = sqrt(Float64(n) * p * (1.0 - p))
    return normal_pdf(Float64(k), mu, sigma)

# Statistical functions
fn mean(values: List[Float64]) -> Float64:
    if len(values) == 0: return 0.0
    var sum_val: Float64 = 0.0
    for v in values:
        sum_val += v
    return sum_val / Float64(len(values))

fn variance(values: List[Float64]) -> Float64:
    if len(values) < 2: return 0.0
    let m = mean(values)
    var sum_sq: Float64 = 0.0
    for v in values:
        sum_sq += (v - m) ** 2
    return sum_sq / Float64(len(values) - 1)

fn stddev(values: List[Float64]) -> Float64:
    return sqrt(variance(values))

fn covariance(x: List[Float64], y: List[Float64]) -> Float64:
    if len(x) != len(y) or len(x) < 2: return 0.0
    let mx = mean(x)
    let my = mean(y)
    var sum_prod: Float64 = 0.0
    for i in range(len(x)):
        sum_prod += (x[i] - mx) * (y[i] - my)
    return sum_prod / Float64(len(x) - 1)

fn correlation(x: List[Float64], y: List[Float64]) -> Float64:
    let cov = covariance(x, y)
    let sx = stddev(x)
    let sy = stddev(y)
    if sx == 0.0 or sy == 0.0: return 0.0
    return cov / (sx * sy)

# Moving average
fn moving_average(values: List[Float64], window: Int) -> List[Float64]:
    if window <= 0 or len(values) < window:
        return values
    
    result = List[Float64]()
    for i in range(window - 1, len(values)):
        var sum_val: Float64 = 0.0
        for j in range(window):
            sum_val += values[i - j]
        result.append(sum_val / Float64(window))
    return result

# Z-score normalization
fn zscore(values: List[Float64]) -> List[Float64]:
    let m = mean(values)
    let s = stddev(values)
    if s == 0.0: return values
    
    result = List[Float64]()
    for v in values:
        result.append((v - m) / s)
    return result

# Min-max normalization
fn minmax_normalize(values: List[Float64]) -> List[Float64]:
    if len(values) == 0: return values
    
    var min_val = values[0]
    var max_val = values[0]
    for v in values:
        if v < min_val: min_val = v
        if v > max_val: max_val = v
    
    let range_val = max_val - min_val
    if range_val == 0.0: return values
    
    result = List[Float64]()
    for v in values:
        result.append((v - min_val) / range_val)
    return result

# Percentile
fn percentile(sorted_values: List[Float64], p: Float64) -> Float64:
    if len(sorted_values) == 0: return 0.0
    if p <= 0.0: return sorted_values[0]
    if p >= 100.0: return sorted_values[-1]
    
    let idx = (p / 100.0) * Float64(len(sorted_values) - 1)
    let lower = Int(idx)
    let upper = lower + 1
    let frac = idx - Float64(lower)
    
    if upper >= len(sorted_values):
        return sorted_values[lower]
    
    return sorted_values[lower] * (1.0 - frac) + sorted_values[upper] * frac

fn main():
    print("Mojo statistics ready")
