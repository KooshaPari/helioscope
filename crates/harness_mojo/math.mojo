# Mojo numerical compute module for heliosHarness
# High-performance vector/matrix operations

from python import Python
from math import exp, log

# Vectorized sigmoid for AI/ML workloads
fn sigmoid[x: Float64](v: Float64) -> Float64:
    return 1.0 / (1.0 + exp(-v))

fn sigmoid_vector[x: Float64](values: List[Float64]) -> List[Float64]:
    result = List[Float64]()
    for i in range(len(values)):
        result.append(sigmoid(values[i]))
    return result

# Weighted scoring for provider selection
fn calculate_score(latency: Float64, cost: Float64, reliability: Float64, weights: List[Float64]) -> Float64:
    # weights: [reliability_weight, latency_weight, cost_weight]
    let rel_score = reliability * weights[0]
    let lat_score = (1.0 - latency) * weights[1]
    let cost_score = (1.0 - cost) * weights[2]
    return rel_score + lat_score + cost_score

# Exponential moving average
fn ema[current: Float64](values: List[Float64], period: Int) -> Float64:
    if len(values) == 0:
        return current
    let alpha = 2.0 / (Float64(period) + 1.0)
    var ema_val = values[0]
    for i in range(1, len(values)):
        ema_val = alpha * values[i] + (1.0 - alpha) * ema_val
    return ema_val

# Resource prediction using linear regression
fn predict_resource[alpha: Float64](history: List[Float64]) -> Float64:
    if len(history) < 2:
        if len(history) == 1:
            return history[0]
        return 0.0
    
    # Simple exponential smoothing prediction
    var predicted = history[0]
    for i in range(1, len(history)):
        predicted = alpha * history[i] + (1.0 - alpha) * predicted
    return predicted

fn main():
    print("Mojo numerical compute ready")
