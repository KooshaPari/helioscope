package harness

import (
	"context"
	"sync"
	"time"
)

// Cache is a simple in-memory cache with TTL
type Cache struct {
	mu    sync.RWMutex
	items map[string]cacheItem
}

type cacheItem struct {
	value      interface{}
	expiresAt time.Time
}

// NewCache creates a new cache
func NewCache() *Cache {
	return &Cache{
		items: make(map[string]cacheItem),
	}
}

// Set stores a value with TTL
func (c *Cache) Set(key string, value interface{}, ttl time.Duration) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.items[key] = cacheItem{
		value:      value,
		expiresAt: time.Now().Add(ttl),
	}
}

// Get retrieves a value
func (c *Cache) Get(key string) (interface{}, bool) {
	c.mu.RLock()
	defer c.mu.RUnlock()
	item, ok := c.items[key]
	if !ok || time.Now().After(item.expiresAt) {
		return nil, false
	}
	return item.value, true
}

// Delete removes a key
func (c *Cache) Delete(key string) {
	c.mu.Lock()
	defer c.mu.Unlock()
	delete(c.items, key)
}

// Clear removes all items
func (c *Cache) Clear() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.items = make(map[string]cacheItem)
}

// Len returns the number of items
func (c *Cache) Len() int {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return len(c.items)
}

// LRUCache is a simple LRU cache
type LRUCache struct {
	mu       sync.Mutex
	capacity int
	list     *list
	items    map[string]*listNode
}

type listNode struct {
	key   string
	value interface{}
	prev  *listNode
	next  *listNode
}

type list struct {
	head *listNode
	tail *listNode
	len  int
}

func newList() *list { return &list{} }

func (l *list) pushFront(node *listNode) {
	node.next = l.head
	node.prev = nil
	if l.head != nil {
		l.head.prev = node
	}
	l.head = node
	if l.tail == nil {
		l.tail = node
	}
	l.len++
}

func (l *list) moveToFront(node *listNode) {
	if node.prev != nil {
		node.prev.next = node.next
		if node.next != nil {
			node.next.prev = node.prev
		} else {
			l.tail = node.prev
		}
		l.len--
		l.pushFront(node)
	}
}

func (l *list) popBack() *listNode {
	if l.tail == nil {
		return nil
	}
	node := l.tail
	l.tail = node.prev
	if l.tail != nil {
		l.tail.next = nil
	}
	l.len--
	return node
}

// NewLRUCache creates a new LRU cache
func NewLRUCache(capacity int) *LRUCache {
	return &LRUCache{
		capacity: capacity,
		list:     newList(),
		items:    make(map[string]*listNode),
	}
}

// Get retrieves a value
func (c *LRUCache) Get(key string) (interface{}, bool) {
	c.mu.Lock()
	defer c.mu.Unlock()
	node, ok := c.items[key]
	if !ok {
		return nil, false
	}
	c.list.moveToFront(node)
	return node.value, true
}

// Set stores a value
func (c *LRUCache) Set(key string, value interface{}) {
	c.mu.Lock()
	defer c.mu.Unlock()
	if node, ok := c.items[key]; ok {
		node.value = value
		c.list.moveToFront(node)
		return
	}
	node := &listNode{key: key, value: value}
	c.items[key] = node
	c.list.pushFront(node)
	if c.list.len > c.capacity {
		if old := c.list.popBack(); old != nil {
			delete(c.items, old.key)
		}
	}
}

// ContextWithTimeout creates a context with timeout
func ContextWithTimeout(timeout time.Duration) (context.Context, context.CancelFunc) {
	return context.WithTimeout(context.Background(), timeout)
}
