"""
NATS Client for Helios Router
Provides event bus and KV cache functionality via JetStream
"""

import json
import logging
from dataclasses import asdict, dataclass
from datetime import datetime
from typing import Any

import nats
from typing import Optional
from nats.errors import NotFoundError
from nats.js.api import StorageType, StreamConfig

logger = logging.getLogger(__name__)


@dataclass
class TokenUsageEvent:
    """Token usage event schema"""

    provider: str
    model: str
    tokens_in: int
    tokens_out: int
    cost_usd: float
    latency_ms: float
    role: str
    timestamp: str = None

    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = datetime.utcnow().isoformat() + "Z"

    def to_json(self) -> str:
        return json.dumps(asdict(self))


@dataclass
class ParetoUpdateEvent:
    """Pareto frontier update event"""

    role_id: str
    weights: dict[str, float]
    pareto_offers: list[str]
    timestamp: str = None

    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = datetime.utcnow().isoformat() + "Z"


class NATSClient:
    """NATS client with JetStream and KV support"""

    def __init__(self, servers: list[str] = None):
        self.servers = servers or ["nats://localhost:4222"]
        self.nc: Optional[nats.NATS] = None
        self.js: Optional[Any] = None

    async def connect(self) -> "NATSClient":
        """Connect to NATS"""
        self.nc = await nats.connect(self.servers)
        self.js = self.nc.jetstream()
        logger.info(f"Connected to NATS: {self.servers}")
        return self

    async def close(self):
        """Close connection"""
        if self.nc:
            await self.nc.close()

    async def ensure_streams(self):
        """Create streams if they don't exist"""
        # Token Usage Stream
        try:
            await self.js.add_stream(
                name="token_usage",
                subjects=["token.usage.*"],
                config=StreamConfig(
                    storage=StorageType.File,
                    subjects=["token.usage.*"],
                    retention=7 * 24 * 60 * 60,
                ),
            )
        except Exception as e:
            logger.debug(f"Stream may exist: {e}")

        # Pareto Updates Stream
        try:
            await self.js.add_stream(
                name="pareto_updates",
                subjects=["pareto.updated"],
                config=StreamConfig(
                    storage=StorageType.File,
                    retention=3600,  # 1 hour
                ),
            )
        except Exception as e:
            logger.debug(f"Stream may exist: {e}")

        # Model Events Stream
        try:
            await self.js.add_stream(
                name="model_events",
                subjects=["model.*"],
                config=StreamConfig(
                    storage=StorageType.File,
                    retention=24 * 3600,  # 24 hours
                ),
            )
        except Exception as e:
            logger.debug(f"Stream may exist: {e}")

    # ============== Key-Value Stores ==============

    async def create_kv_store(self, name: str, ttl: int = 300):
        """Create KV store with TTL"""
        try:
            await self.js.create_key_value(
                name=name,
                max_value_size=1024 * 100,  # 100KB max
                max_bytes=None,
                history=3,  # Keep last 3 values
                ttl=ttl * 1_000_000_000,  # Convert to nanoseconds
            )
            logger.info(f"Created KV store: {name} (TTL: {ttl}s)")
        except Exception as e:
            logger.debug(f"KV store may exist: {e}")

    async def get_kv(self, store_name: str):
        """Get KV store handle"""
        return await self.js.key_value(store_name)

    async def kv_put(self, store: str, key: str, value: Any, ttl: int = None):
        """Put value in KV store"""
        kv = await self.get_kv(store)
        data = json.dumps(value) if not isinstance(value, str) else value
        await kv.put(key, data.encode())

    async def kv_get(self, store: str, key: str) -> Any | None:
        """Get value from KV store"""
        try:
            kv = await self.get_kv(store)
            entry = await kv.get(key)
            if entry:
                return json.loads(entry.value().decode())
        except NotFoundError:
            pass
        except Exception as e:
            logger.warning(f"KV get error: {e}")
        return None

    async def kv_delete(self, store: str, key: str):
        """Delete key from KV store"""
        try:
            kv = await self.get_kv(store)
            await kv.delete(key)
        except Exception as e:
            logger.warning(f"KV delete error: {e}")

    async def kv_watch(self, store: str, prefix: str = ""):
        """Watch KV store for changes"""
        kv = await self.get_kv(store)
        watcher = await kv.watch(prefix)
        return watcher

    # ============== Event Publishing ==============

    async def publish_token_usage(self, event: TokenUsageEvent):
        """Publish token usage event"""
        subject = f"token.usage.{event.provider}.{event.model.replace('/', '_')}"
        await self.js.publish(subject, event.to_json().encode())
        logger.debug(f"Published token usage: {subject}")

    async def publish_pareto_update(self, event: ParetoUpdateEvent):
        """Publish Pareto update event"""
        await self.js.publish("pareto.updated", json.dumps(asdict(event)).encode())
        logger.debug("Published pareto update")

    async def publish_model_status(self, provider: str, model: str, status: str):
        """Publish model status change"""
        event = {
            "provider": provider,
            "model": model,
            "status": status,
            "timestamp": datetime.utcnow().isoformat() + "Z",
        }
        await self.js.publish(f"model.{provider}.{model.replace('/', '_')}", json.dumps(event).encode())

    # ============== Event Subscribing ==============

    async def subscribe_token_usage(self, handler, subject: str = "token.usage.>"):
        """Subscribe to token usage events"""
        sub = await self.js.subscribe(subject)
        async for msg in sub.messages:
            try:
                data = json.loads(msg.data.decode())
                await handler(data)
                await msg.ack()
            except Exception as e:
                logger.error(f"Error processing token usage: {e}")

    async def subscribe_pareto_updates(self, handler):
        """Subscribe to Pareto updates"""
        sub = await self.js.subscribe("pareto.updated")
        async for msg in sub.messages:
            try:
                data = json.loads(msg.data.decode())
                await handler(data)
                await msg.ack()
            except Exception as e:
                logger.error(f"Error processing pareto update: {e}")

    # ============== Convenience Methods ==============

    async def cache_provider_status(self, provider: str, status: dict):
        """Cache provider status"""
        await self.kv_put("provider_cache", provider, status, ttl=300)

    async def get_provider_status(self, provider: str) -> dict | None:
        """Get cached provider status"""
        return await self.kv_get("provider_cache", provider)

    async def cache_model_price(self, provider: str, model: str, price: dict):
        """Cache model price"""
        key = f"{provider}/{model}"
        await self.kv_put("price_cache", key, price, ttl=60)

    async def get_model_price(self, provider: str, model: str) -> dict | None:
        """Get cached model price"""
        key = f"{provider}/{model}"
        return await self.kv_get("price_cache", key)

    async def cache_pareto_frontier(self, role_id: str, frontier: list[dict]):
        """Cache computed Pareto frontier"""
        await self.kv_put("pareto_cache", role_id, frontier, ttl=60)

    async def get_pareto_frontier(self, role_id: str) -> list[dict] | None:
        """Get cached Pareto frontier"""
        return await self.kv_get("pareto_cache", role_id)


# Global client instance
_client: NATSClient | None = None


async def get_nats_client() -> NATSClient:
    """Get global NATS client"""
    global _client
    if _client is None:
        _client = NATSClient()
        await _client.connect()
        await _client.ensure_streams()

        # Create KV stores
        await _client.create_kv_store("provider_cache", ttl=300)  # 5 min
        await _client.create_kv_store("model_cache", ttl=3600)  # 1 hour
        await _client.create_kv_store("pareto_cache", ttl=60)  # 1 min
        await _client.create_kv_store("price_cache", ttl=60)  # 1 min

    return _client


async def close_nats_client():
    """Close global NATS client"""
    global _client
    if _client:
        await _client.close()
        _client = None
