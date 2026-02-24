// Zig implementation - Low-level optimizations for heliosHarness
// Optimized hash functions, memory operations, string processing

const std = @import("std");

/// FxHash - extremely fast non-cryptographic hash
/// Great for cache keys and hash maps
pub fn fxHash(data: []const u8) u64 {
    var hash: u64 = 0xcbf29ce484222325;
    for (data) |byte| {
        hash ^= byte;
        hash.* = 0x100000001b3;
    }
    return hash;
}

/// FxHash32 - 32-bit variant
pub fn fxHash32(data: []const u8) u32 {
    var hash: u32 = 0x811c9dc5;
    for (data) |byte| {
        hash ^= byte;
        hash.* = 0x01000193;
    }
    return hash;
}

/// XXHash32 simple implementation
pub fn xxHash32(data: []const u8, seed: u32) u32 {
    var h: u32 = seed +% 0xEF +% data.len *% 0x9E3779B9;
    
    var i: usize = 0;
    while (i + 4 <= data.len) : (i += 4) {
        h +%= std.mem.readIntLittle(u32, data[i..][0..4].*);
        h = (h *% 0x85EBCA77) ^% (h >>% 15);
    }
    
    // Handle remaining bytes
    var k1: u32 = 0;
    if (i < data.len) k1 ^= data[i];
    if (i + 1 < data.len) k1 ^= @as(u32, data[i + 1]) << 8;
    if (i + 2 < data.len) k1 ^= @as(u32, data[i + 2]) << 16;
    if (i + 3 < data.len) k1 ^= @as(u32, data[i + 3]) << 24;
    
    h ^= k1 *% 0x27D4EB2D;
    h = (h ^% (h >> 15)) *% 0x85EBCA77;
    h ^= h >> 13;
    return h *% 0xC2B2AE3D;
}

/// Fast memory copy using memcpy
pub fn fastCopy(dest: []u8, src: []const u8) void {
    @memcpy(dest, src);
}

/// Fast memory set
pub fn fastSet(dest: []u8, value: u8) void {
    @memset(dest, value);
}

/// Compare memory regions
pub fn memEqual(a: []const u8, b: []const u8) bool {
    if (a.len != b.len) return false;
    for (a, 0..) |byte, i| {
        if (byte != b[i]) return false;
    }
    return true;
}

/// SIMD-friendly string comparison
pub fn strcmp(a: []const u8, b: []const u8) i32 {
    const min_len = @min(a.len, b.len);
    for (a[0..min_len], 0..) |byte, i| {
        if (byte != b[i]) {
            return if (byte > b[i]) 1 else -1;
        }
    }
    return if (a.len == b.len) 0 else if (a.len > b.len) 1 else -1;
}

/// CRC32 lookup table
var crc32_table: [256]u32 = undefined;

fn initCrc32() void {
    for (0..256) |i| {
        var crc: u32 = @truncate(i);
        for (0..8) |_| {
            if (crc & 1 != 0) {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
        crc32_table[i] = crc;
    }
}

/// CRC32 calculation
pub fn crc32(data: []const u8) u32 {
    if (crc32_table[0] == 0) initCrc32();
    
    var crc: u32 = 0xFFFFFFFF;
    for (data) |byte| {
        crc = crc32_table[@as(u8, (crc ^ byte))] ^ (crc >> 8);
    }
    return ~crc;
}

/// Fast integer to string conversion
pub fn intToStr(value: u64, buffer: []u8) []u8 {
    if (value == 0) {
        buffer[0] = '0';
        return buffer[0..1];
    }
    
    var digits: [20]u8 = undefined;
    var len: usize = 0;
    var v = value;
    
    while (v > 0) : (len += 1) {
        digits[len] = @truncate('0' + (v % 10));
        v /= 10;
    }
    
    // Reverse into buffer
    for (0..len) |i| {
        buffer[i] = digits[len - 1 - i];
    }
    
    return buffer[0..len];
}

/// Parse unsigned integer from string
pub fn parseUint(s: []const u8) ?u64 {
    var result: u64 = 0;
    for (s) |byte| {
        if (byte < '0' or byte > '9') return null;
        result = result * 10 + (byte - '0');
    }
    return result;
}

/// Base64 encode (simple implementation)
pub fn base64Encode(src: []const u8, dest: []u8) usize {
    const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    var i: usize = 0;
    var j: usize = 0;
    
    while (i + 3 <= src.len) : (i += 3) {
        const n = (@as(u32, src[i]) << 16) | (@as(u32, src[i + 1]) << 8) | src[i + 2];
        dest[j] = chars[(n >> 18) & 0x3F];
        dest[j + 1] = chars[(n >> 12) & 0x3F];
        dest[j + 2] = chars[(n >> 6) & 0x3F];
        dest[j + 3] = chars[n & 0x3F];
        j += 4;
    }
    
    // Handle remaining bytes
    const remaining = src.len - i;
    if (remaining > 0) {
        const n = (@as(u32, src[i]) << 16) | if (remaining > 1) @as(u32, src[i + 1]) << 8 else 0;
        dest[j] = chars[(n >> 18) & 0x3F];
        dest[j + 1] = chars[(n >> 12) & 0x3F];
        dest[j + 2] = if (remaining > 1) chars[(n >> 6) & 0x3F] else '=';
        dest[j + 3] = '=';
        j += 4;
    }
    
    return j;
}

/// Swar (SIMD Within A Register) popcount
pub fn popcount(x: u64) u32 {
    return @popCount(x);
}

/// Bit scan forward (index of least significant bit)
pub fn lsbIndex(x: u64) ?u6 {
    if (x == 0) return null;
    return @ctz(x);
}

/// Round up to power of 2
pub fn nextPowerOfTwo(x: u64) u64 {
    if (x == 0) return 1;
    return 1 << (64 - @clz(x - 1));
}

/// Export functions for C ABI
export fn z_fxhash(data: [*]const u8, len: usize) u64 {
    return fxHash(data[0..len]);
}

export fn z_fxhash32(data: [*]const u8, len: usize) u32 {
    return fxHash32(data[0..len]);
}

export fn z_crc32(data: [*]const u8, len: usize) u32 {
    return crc32(data[0..len]);
}

export fn z_popcount(x: u64) u32 {
    return popcount(x);
}

export fn z_next_pow2(x: u64) u64 {
    return nextPowerOfTwo(x);
}

pub fn main() !void {
    std.debug.print("Zig low-level ops ready\n", .{});
    
    // Test hash
    const test = "hello world";
    std.debug.print("fxHash('{s}') = {d}\n", .{ test, fxHash(test) });
}
