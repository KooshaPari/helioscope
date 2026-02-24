// Zig data structures - High-performance collections

const std = @import("std");

/// Ring buffer (circular queue) - lock-free for single producer/consumer
pub fn RingBuffer(comptime T: type, comptime capacity: usize) type {
    return struct {
        data: [capacity]T,
        read: usize = 0,
        write: usize = 0,
        count: usize = 0,

        const Self = @This();

        pub fn push(self: *Self, item: T) bool {
            if (self.count >= capacity) return false;
            self.data[self.write] = item;
            self.write = (self.write + 1) % capacity;
            self.count += 1;
            return true;
        }

        pub fn pop(self: *Self) ?T {
            if (self.count == 0) return null;
            const item = self.data[self.read];
            self.read = (self.read + 1) % capacity;
            self.count -= 1;
            return item;
        }

        pub fn len(self: *const Self) usize { return self.count; }
        pub fn isEmpty(self: *const Self) bool { return self.count == 0; }
        pub fn isFull(self: *const Self) bool { return self.count >= capacity; }
    };
}

/// Stack using array - LIFO
pub fn Stack(comptime T: type, comptime capacity: usize) type {
    return struct {
        data: [capacity]T,
        top: usize = 0,

        const Self = @This();

        pub fn push(self: *Self, item: T) bool {
            if (self.top >= capacity) return false;
            self.data[self.top] = item;
            self.top += 1;
            return true;
        }

        pub fn pop(self: *Self) ?T {
            if (self.top == 0) return null;
            self.top -= 1;
            return self.data[self.top];
        }

        pub fn peek(self: *const Self) ?T {
            if (self.top == 0) return null;
            return self.data[self.top - 1];
        }

        pub fn len(self: *const Self) usize { return self.top; }
    };
}

/// Priority queue using binary heap
pub fn PriorityQueue(comptime T: type, comptime capacity: usize) type {
    return struct {
        data: [capacity]T,
        size: usize = 0,
        lessThan: *const fn (a: T, b: T) bool,

        const Self = @This();

        pub fn init(lessThan: *const fn (a: T, b: T) bool) Self {
            return Self{ .data = undefined, .size = 0, .lessThan = lessThan };
        }

        pub fn push(self: *Self, item: T) bool {
            if (self.size >= capacity) return false;
            self.data[self.size] = item;
            self.size += 1;
            self.bubbleUp(self.size - 1);
            return true;
        }

        pub fn pop(self: *Self) ?T {
            if (self.size == 0) return null;
            const result = self.data[0];
            self.size -= 1;
            if (self.size > 0) {
                self.data[0] = self.data[self.size];
                self.bubbleDown(0);
            }
            return result;
        }

        fn bubbleUp(self: *Self, idx: usize) void {
            var current = idx;
            while (current > 0) {
                const parent = (current - 1) / 2;
                if (!self.lessThan(self.data[current], self.data[parent])) break;
                const temp = self.data[parent];
                self.data[parent] = self.data[current];
                self.data[current] = temp;
                current = parent;
            }
        }

        fn bubbleDown(self: *Self, idx: usize) void {
            var current = idx;
            while (true) {
                const left = 2 * current + 1;
                const right = 2 * current + 2;
                var smallest = current;
                if (left < self.size and self.lessThan(self.data[left], self.data[smallest])) smallest = left;
                if (right < self.size and self.lessThan(self.data[right], self.data[smallest])) smallest = right;
                if (smallest == current) break;
                const temp = self.data[smallest];
                self.data[smallest] = self.data[current];
                self.data[current] = temp;
                current = smallest;
            }
        }

        pub fn len(self: *const Self) usize { return self.size; }
    };
}

/// Open addressing hash map
pub fn HashMap(comptime K: type, comptime V: type, comptime capacity: usize) type {
    return struct {
        keys: [capacity]K,
        values: [capacity]V,
        occupied: [capacity]bool,
        size: usize = 0,

        const Self = @This();

        pub fn put(self: *Self, key: K, value: V) bool {
            if (self.size >= capacity) return false;
            var idx = self.hash(key) % capacity;
            while (self.occupied[idx]) {
                if (std.mem.eql(K, &self.keys[idx], &key)) {
                    self.values[idx] = value;
                    return true;
                }
                idx = (idx + 1) % capacity;
            }
            self.keys[idx] = key;
            self.values[idx] = value;
            self.occupied[idx] = true;
            self.size += 1;
            return true;
        }

        pub fn get(self: *const Self, key: K) ?V {
            var idx = self.hash(key) % capacity;
            var attempts: usize = 0;
            while (self.occupied[idx] and attempts < capacity) {
                if (std.mem.eql(K, &self.keys[idx], &key)) return self.values[idx];
                idx = (idx + 1) % capacity;
                attempts += 1;
            }
            return null;
        }

        fn hash(self: *const Self, key: K) u64 {
            var hasher = std.hash.Wyhash.init();
            hasher.update(std.mem.asBytes(&key));
            return hasher.final();
        }

        pub fn len(self: *const Self) usize { return self.size; }
    };
}

/// Bit vector (bitset)
pub fn BitSet(comptime size: usize) type {
    const words = (size + 63) / 64;
    return struct {
        data: [words]u64 = [_]u64{0} ** words,

        const Self = @This();

        pub fn set(self: *Self, idx: usize) void {
            if (idx >= size) return;
            self.data[idx / 64] |= @as(u64, 1) << @truncate(idx % 64);
        }

        pub fn clear(self: *Self, idx: usize) void {
            if (idx >= size) return;
            self.data[idx / 64] &= ~(@as(u64, 1) << @truncate(idx % 64));
        }

        pub fn toggle(self: *Self, idx: usize) void {
            if (idx >= size) return;
            self.data[idx / 64] ^= @as(u64, 1) << @truncate(idx % 64);
        }

        pub fn get(self: *const Self, idx: usize) bool {
            if (idx >= size) return false;
            return (self.data[idx / 64] >> @truncate(idx % 64)) & 1 == 1;
        }

        pub fn count(self: *const Self) usize {
            var total: usize = 0;
            for (self.data) |word| {
                total += @popCount(word);
            }
            return total;
        }
    };
}

pub fn main() !void {
    std.debug.print("Zig data structures ready\n", .{});
}
