"""Helios Router - Main Streamlit Application

Modular structure with separated concerns:
- db/schema.py: Database layer
- pareto/engine.py: Pareto computation
- ui/components.py: UI components
- nats_client.py: NATS event bus + KV cache
"""

import asyncio
from pathlib import Path

import pandas as pd
import streamlit as st

from helios_router_ui.db.schema import get_connection, init_db
from helios_router_ui.pareto.engine import compute_pareto
from helios_router_ui.ui.components import (
    render_data_editor,
    render_metrics_row,
    render_pareto_table,
    render_sidebar_controls,
    render_weight_sliders,
)

# Try to import NATS (optional)
try:
    from helios_router_ui.nats_client import TokenUsageEvent, get_nats_client

    HAS_NATS = True
except ImportError:
    HAS_NATS = False

# Constants
DB_PATH = Path(__file__).parent / "helios.db"
BENCH_GROUPS = [
    "coding_synthesis",
    "coding_debugging",
    "coding_repo_swe",
    "coding_editing_apply",
    "tool_use_agents",
    "instruction_following_format",
    "long_context_retrieval",
    "math_competition",
    "general_reasoning_knowledge",
    "multilingual",
    "safety_alignment",
    "truthfulness_hallucination",
    "multimodal_vision",
]

DEFAULT_WEIGHTS = dict.fromkeys(BENCH_GROUPS, 0.1)


def load_table(table_name: str) -> pd.DataFrame:
    """Load table from SQLite."""
    conn = get_connection()
    try:
        df = pd.read_sql_query(f"SELECT * FROM {table_name}", conn)
    except Exception:
        df = pd.DataFrame()
    conn.close()
    return df


async def get_cached_offers():
    """Get offers from cache or database"""
    if not HAS_NATS:
        return None

    try:
        nc = await get_nats_client()
        cached = await nc.kv_get("model_cache", "offers")
        if cached:
            return cached
    except Exception:
        pass
    return None


async def cache_offers(offers_df: pd.DataFrame):
    """Cache offers in NATS"""
    if not HAS_NATS:
        return

    try:
        nc = await get_nats_client()
        await nc.kv_put("model_cache", "offers", offers_df.to_dict("records"), ttl=3600)
    except Exception:
        pass


async def publish_token_usage(
    provider: str, model: str, tokens_in: int, tokens_out: int, cost: float, latency: float, role: str
):
    """Publish token usage event to NATS"""
    if not HAS_NATS:
        return

    try:
        nc = await get_nats_client()
        event = TokenUsageEvent(
            provider=provider,
            model=model,
            tokens_in=tokens_in,
            tokens_out=tokens_out,
            cost_usd=cost,
            latency_ms=latency,
            role=role,
        )
        await nc.publish_token_usage(event)
    except Exception:
        pass


def publish_pareto_update(role: str, weights: dict):
    """Stub for Pareto updates - implement NATS publish"""
    pass


def main():
    if not DB_PATH.exists():
        init_db()

    st.set_page_config(page_title="Helios Router", layout="wide", page_icon="🔄")

    # NATS status
    nats_status = "🔴 Disconnected"
    if HAS_NATS:
        nats_status = "🟢 NATS Connected"
    st.sidebar.markdown(f"**Event Bus:** {nats_status}")

    # Sidebar controls
    ctrl = render_sidebar_controls()

    # Load data
    offers_df = load_table("offers")
    roles_df = load_table("roles")
    # telemetry_df = load_table("telemetry")

    # Cache offers in background
    if HAS_NATS and not offers_df.empty:
        try:
            asyncio.create_task(cache_offers(offers_df))
        except RuntimeError:
            pass  # Not in async context

    # Main tabs
    tabs = st.tabs(["📊 Dashboard", "💎 Offers", "⚖️ Roles", "🎯 Pareto", "📡 Events"])

    with tabs[0]:  # Dashboard
        st.title("📊 Helios Router Dashboard")

        # Budget metrics
        remaining = ctrl["budget_monthly"] - ctrl["spend_to_date"]
        render_metrics_row(
            {
                "Offers": len(offers_df),
                "Roles": len(roles_df),
                "Budget": f"${ctrl['budget_monthly']:.0f}",
                "Remaining": f"${remaining:.0f}",
            }
        )

        # Cost chart
        if not offers_df.empty and "price_out_usd_per_mtok" in offers_df.columns:
            st.subheader("💰 Cost Overview")
            cost_data = offers_df[["offer_id", "price_out_usd_per_mtok"]].dropna()
            if not cost_data.empty:
                cost_data = cost_data.sort_values("price_out_usd_per_mtok").head(15)
                st.bar_chart(cost_data.set_index("offer_id"))

    with tabs[1]:  # Offers
        st.header("💎 Offers")

        # Filters
        with st.expander("🔍 Filters", expanded=True):
            col1, col2 = st.columns(2)
            with col1:
                search = st.text_input("Search", placeholder="Search offers...")
            with col2:
                providers = ["All"] + (
                    offers_df["provider_id"].unique().tolist() if "provider_id" in offers_df.columns else []
                )
                provider_filter = st.selectbox("Provider", providers)

        # Apply filters
        filtered = offers_df
        if search:
            filtered = filtered[filtered.apply(lambda r: search.lower() in str(r.values).lower(), axis=1)]
        if provider_filter != "All" and "provider_id" in filtered.columns:
            filtered = filtered[filtered["provider_id"] == provider_filter]

        st.caption(f"Showing {len(filtered)} of {len(offers_df)} offers")

        if not filtered.empty:
            render_data_editor(
                filtered,
                "offers_editor",
                {
                    "lifecycle_state": st.column_config.SelectboxColumn(
                        "Lifecycle", options=["active", "canary", "degraded", "suspended", "disabled"]
                    )
                },
            )

    with tabs[2]:  # Roles
        st.header("⚖️ Roles & Weights")

        if not roles_df.empty:
            selected_role = st.selectbox("Select Role", roles_df["role_id"].tolist())
            role_row = (
                roles_df[roles_df["role_id"] == selected_role].iloc[0]
                if len(roles_df[roles_df["role_id"] == selected_role]) > 0
                else None
            )

            # Get weights from role
            role_weights = {}
            for g in BENCH_GROUPS:
                col = f"w_{g}"
                role_weights[g] = (
                    DEFAULT_WEIGHTS.get(g, 0.0)
                    if role_row is None
                    else role_row.to_dict().get(col, DEFAULT_WEIGHTS.get(g, 0.0))
                )

            col1, col2 = st.columns(2)
            with col1:
                st.subheader("Adjust Weights")
                weights = render_weight_sliders(role_weights, key_prefix=f"role_{selected_role}")

                # Normalize
                total = sum(weights.values())
                if total > 0:
                    norm = {k: v / total for k, v in weights.items()}
                    st.caption("Normalized:")
                    st.json(norm)

                if st.button("💾 Save Weights"):
                    conn = get_connection()
                    cursor = conn.cursor()
                    for g in BENCH_GROUPS:
                        cursor.execute(
                            f"UPDATE roles SET w_{g} = ?, updated_at = ? WHERE role_id = ?",
                            (weights[g], pd.Timestamp.now().isoformat(), selected_role),
                        )
                    conn.commit()
                    conn.close()
                    st.success("Weights saved!")

                    # Publish Pareto update
                    if HAS_NATS:
                        try:
                            asyncio.create_task(publish_pareto_update(selected_role, weights))
                        except RuntimeError:
                            pass

            with col2:
                st.subheader("Live Pareto Preview")

                # Build indices (simplified)
                if not offers_df.empty:
                    # Quick quality/cost calc
                    indices = offers_df[["offer_id", "provider_id", "model_id"]].copy()
                    indices["quality"] = 0.5  # Default
                    indices["cost_usd"] = offers_df.get("price_out_usd_per_mtok", 0).fillna(0.1)
                    indices["speed_score"] = 50  # Default

                    # Compute Pareto
                    pareto = compute_pareto(indices)
                    render_pareto_table(pareto)

    with tabs[3]:  # Pareto
        st.header("🎯 Pareto Analysis")

        # Full Pareto computation
        if not offers_df.empty:
            role_id = st.selectbox("Role", roles_df["role_id"].tolist() if not roles_df.empty else ["default"])

            # Get weights
            role_row = (
                roles_df[roles_df["role_id"] == role_id].iloc[0]
                if len(roles_df[roles_df["role_id"] == role_id]) > 0
                else None
            )
            weights = {
                g: role_row.to_dict().get(f"w_{g}", DEFAULT_WEIGHTS.get(g, 0.1))
                if role_row is not None
                else DEFAULT_WEIGHTS.get(g, 0.1)
                for g in BENCH_GROUPS
            }

            # Compute indices
            indices = offers_df[["offer_id", "provider_id", "model_id"]].copy()
            indices["quality"] = 0.5
            indices["cost_usd"] = offers_df.get("price_out_usd_per_mtok", 0).fillna(0.1)
            indices["speed_score"] = 50

            # Pareto
            min_cost = st.checkbox("Minimize Cost", value=True)
            min_speed = st.checkbox("Minimize Speed", value=True)
            max_quality = st.checkbox("Maximize Quality", value=True)

            pareto = compute_pareto(indices, min_cost, min_speed, max_quality)

            render_pareto_table(pareto)

            # CSV download
            st.download_button("📥 Download CSV", pareto.to_csv(index=False), "pareto.csv")

    with tabs[4]:  # Events
        st.header("📡 Event Bus")

        st.info("NATS JetStream + KV Store for caching and events")

        col1, col2 = st.columns(2)
        with col1:
            st.subheader("KV Stores")
            st.write("- provider_cache (5min TTL)")
            st.write("- model_cache (1hr TTL)")
            st.write("- pareto_cache (1min TTL)")
            st.write("- price_cache (1min TTL)")

        with col2:
            st.subheader("Streams")
            st.write("- token_usage (7 day retention)")
            st.write("- pareto_updates (1hr retention)")
            st.write("- model_events (24hr retention)")

        # Simulate token usage
        with st.expander("📤 Send Test Event"):
            provider = st.selectbox("Provider", ["openai", "anthropic", "google"])
            model = st.text_input("Model", "gpt-4o")
            tokens_in = st.number_input("Tokens In", 1000)
            tokens_out = st.number_input("Tokens Out", 500)
            cost = st.number_input("Cost ($)", 0.1)

            if st.button("Publish Token Usage"):
                if HAS_NATS:
                    try:
                        asyncio.get_event_loop().run_until_complete(
                            publish_token_usage(provider, model, tokens_in, tokens_out, cost, 1200, "default")
                        )
                        st.success("Event published!")
                    except Exception as e:
                        st.error(f"Error: {e}")
                else:
                    st.warning("NATS not available")


if __name__ == "__main__":
    main()
