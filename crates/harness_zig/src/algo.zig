// Zig algorithms - Sorting, searching, optimization

const std = @import("std");

/// Quick sort implementation
pub fn quickSort(comptime T: type, slice: []T, lessThan: *const fn (a: T, b: T) bool) void {
    if (slice.len < 2) return;
    
    const pivot = slice[slice.len / 2];
    var left: usize = 0;
    var right: usize = slice.len;
    
    while (left < right) {
        if (lessThan(slice[left], pivot)) {
            left += 1;
        } else {
            right -= 1;
            const temp = slice[left];
            slice[left] = slice[right];
            slice[right] = temp;
        }
    }
    
    quickSort(T, slice[0..left], lessThan);
    quickSort(T, slice[right..], lessThan);
}

/// Binary search - returns index or null
pub fn binarySearch(comptime T: type, slice: []const T, target: T, compare: *const fn (a: T, b: T) i32) ?usize {
    var left: usize = 0;
    var right: usize = slice.len;
    
    while (left < right) {
        const mid = left + (right - left) / 2;
        const cmp = compare(slice[mid], target);
        
        if (cmp == 0) return mid;
        if (cmp < 0) left = mid + 1;
        else right = mid;
    }
    
    return null;
}

/// Merge sort implementation
pub fn mergeSort(comptime T: type, slice: []T, lessThan: *const fn (a: T, b: T) bool) void {
    if (slice.len < 2) return;
    
    const mid = slice.len / 2;
    mergeSort(T, slice[0..mid], lessThan);
    mergeSort(T, slice[mid..], lessThan);
    merge(T, slice[0..mid], slice[mid..], slice, lessThan);
}

fn merge(comptime T: type, left: []const T, right: []const T, result: []T, lessThan: *const fn (a: T, b: T) bool) void {
    var i: usize = 0;
    var j: usize = 0;
    var k: usize = 0;
    
    while (i < left.len and j < right.len) {
        if (lessThan(left[i], right[j])) {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    
    while (i < left.len) {
        result[k] = left[i];
        i += 1;
        k += 1;
    }
    
    while (j < right.len) {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}

/// Heap sort
pub fn heapSort(comptime T: type, slice: []T, lessThan: *const fn (a: T, b: T) bool) void {
    if (slice.len < 2) return;
    
    // Build max heap
    var i: usize = slice.len / 2;
    while (i > 0) {
        i -= 1;
        siftDown(T, slice, i, slice.len, lessThan);
    }
    
    // Extract elements
    i = slice.len;
    while (i > 1) {
        i -= 1;
        const temp = slice[0];
        slice[0] = slice[i];
        slice[i] = temp;
        siftDown(T, slice, 0, i, lessThan);
    }
}

fn siftDown(comptime T: type, slice: []T, root: usize, heap_len: usize, lessThan: *const fn (a: T, b: T) bool) void {
    while (root * 2 + 1 < heap_len) {
        var child = root * 2 + 1;
        
        if (child + 1 < heap_len and lessThan(slice[child], slice[child + 1])) {
            child += 1;
        }
        
        if (!lessThan(slice[root], slice[child])) break;
        
        const temp = slice[root];
        slice[root] = slice[child];
        slice[child] = temp;
        
        root = child;
    }
}

/// Find minimum in slice
pub fn findMin(comptime T: type, slice: []const T, lessThan: *const fn (a: T, b: T) bool) ?T {
    if (slice.len == 0) return null;
    var min_val = slice[0];
    for (slice[1..]) |val| {
        if (lessThan(val, min_val)) min_val = val;
    }
    return min_val;
}

/// Find maximum in slice
pub fn findMax(comptime T: type, slice: []const T, lessThan: *const fn (a: T, b: T) bool) ?T {
    if (slice.len == 0) return null;
    var max_val = slice[0];
    for (slice[1..]) |val| {
        if (lessThan(max_val, val)) max_val = val;
    }
    return max_val;
}

/// Calculate mean
pub fn mean(slice: []const f64) f64 {
    if (slice.len == 0) return 0.0;
    var sum: f64 = 0.0;
    for (slice) |val| sum += val;
    return sum / @as(f64, @floatFromInt(slice.len));
}

/// Calculate variance
pub fn variance(slice: []const f64) f64 {
    if (slice.len < 2) return 0.0;
    const m = mean(slice);
    var sum_sq: f64 = 0.0;
    for (slice) |val| {
        const diff = val - m;
        sum_sq += diff * diff;
    }
    return sum_sq / @as(f64, @floatFromInt(slice.len - 1));
}

/// Calculate standard deviation
pub fn stddev(slice: []const f64) f64 {
    return @sqrt(variance(slice));
}

/// Linear interpolation
pub fn lerp(a: f64, b: f64, t: f64) f64 {
    return a + (b - a) * t;
}

/// Clamp value between min and max
pub fn clamp(val: f64, min_val: f64, max_val: f64) f64 {
    if (val < min_val) return min_val;
    if (val > max_val) return max_val;
    return val;
}

/// Map value from one range to another
pub fn mapRange(value: f64, inMin: f64, inMax: f64, outMin: f64, outMax: f64) f64 {
    return outMin + (value - inMin) * (outMax - outMin) / (inMax - inMin);
}

pub fn main() !void {
    std.debug.print("Zig algorithms ready\n", .{});
}
