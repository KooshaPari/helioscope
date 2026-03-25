"""
Sample data generator for Helios Router MVP
"""

import sys

sys.path.insert(0, "./src")
from datetime import datetime

from helios_router_ui.db.schema import get_connection, init_db


def generate_sample_data():
    """Generate sample data for MVP testing."""
    conn = get_connection()
    cursor = conn.cursor()
    now = datetime.now().isoformat()

    # Sample Providers
    providers = [
        ("openai", "OpenAI", "aggregator", "openai_compat", 0.1, "US East"),
        ("anthropic", "Anthropic", "direct", "anthropic", 0.15, "US East"),
        ("google", "Google AI", "direct", "google", 0.1, "US West"),
        ("cerebras", "Cerebras", "direct", "openai_compat", 0.05, "US East"),
        ("deepseek", "DeepSeek", "aggregator", "openai_compat", 0.1, "Global"),
        ("meta", "Meta AI", "direct", "openai_compat", 0.1, "US East"),
    ]

    for pid, name, ptype, style, vol, region in providers:
        cursor.execute(
            """
            INSERT OR IGNORE INTO providers (provider_id, display_name, provider_type, api_style,
                regions, volatility_score, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (pid, name, ptype, style, region, vol, now, now),
        )

    # Sample Plans
    plans = [
        ("payg_openai", "openai", "payg_token", "OpenAI Pay-per-use", 0, 0, 0, 0, 0),
        ("payg_anthropic", "anthropic", "payg_token", "Anthropic Pay-per-use", 0, 0, 0, 0, 0),
        ("subscription_gpt4", "openai", "fixed_bucket", "GPT-4 Subscription", 20, 50000000, 0, 0, 0),
        ("daily_cerebras", "cerebras", "daily_bucket", "Cerebras Daily", 0, 0, 50000000, 0, 0),
        ("copilot_units", "openai", "weighted_units", "Copilot Units", 10, 0, 0, 5000, 0.04),
        ("free_deepseek", "deepseek", "volatile_free", "DeepSeek Free", 0, 0, 0, 0, 0),
    ]

    for plan_id, prov, ptype, name, fee, tokens_m, tokens_d, _units, overage in plans:
        cursor.execute(
            """
            INSERT OR IGNORE INTO plans (plan_id, provider_id, plan_type, plan_display_name,
                monthly_fee_usd, included_tokens_per_month, tokens_per_day_cap,
                unit_overage_usd, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (plan_id, prov, ptype, name, fee, tokens_m, tokens_d, overage, now, now),
        )

    # Sample Offers
    offers = [
        ("gpt-4o", "openai", "gpt-4o", "payg_openai", "us-east", 2.5, 10.0, 500, 150000, 1.0, 0),
        ("gpt-4o-mini", "openai", "gpt-4o-mini", "payg_openai", "us-east", 0.15, 0.6, 500, 150000, 1.0, 0),
        (
            "claude-3-5-sonnet",
            "anthropic",
            "claude-3-5-sonnet",
            "payg_anthropic",
            "us-east",
            3.0,
            15.0,
            500,
            200000,
            1.0,
            0,
        ),
        ("claude-3-haiku", "anthropic", "claude-3-haiku", "payg_anthropic", "us-east", 0.25, 1.25, 500, 200000, 1.0, 0),
        ("gemini-1.5-pro", "google", "gemini-1.5-pro", "payg_openai", "us-west", 1.25, 5.0, 500, 200000, 1.0, 0),
        ("gemini-1.5-flash", "google", "gemini-1.5-flash", "payg_openai", "us-west", 0.075, 0.3, 500, 200000, 1.0, 0),
        ("cerebras-code", "cerebras", "cerebras-code-9b", "daily_cerebras", "us-east", 0.0, 0.0, 900, 1000000, 0.0, 0),
        ("deepseek-chat", "deepseek", "deepseek-chat", "free_deepseek", "global", 0.0, 0.0, 500, 64000, 0.0, 1),
        ("llama-3-70b", "meta", "llama-3-70b-instruct", "payg_openai", "us-east", 0.9, 0.9, 500, 128000, 1.0, 0),
    ]

    for offer_id, prov, model, plan, region, price_in, price_out, rpm, tpm, unit_mult, is_0x in offers:
        cursor.execute(
            """
            INSERT OR IGNORE INTO offers (offer_id, provider_id, model_id, plan_id, region,
                price_in_usd_per_mtok, price_out_usd_per_mtok, rpm_limit, tpm_limit,
                unit_multiplier, is_included_0x, lifecycle_state, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (offer_id, prov, model, plan, region, price_in, price_out, rpm, tpm, unit_mult, is_0x, "active", now, now),
        )

    # Sample Telemetry
    cursor.execute("SELECT offer_id FROM offers")
    offer_ids = [r[0] for r in cursor.fetchall()]
    roles = ["code_reasoner", "code_patch_generator", "code_apply_patch", "code_scaffold_fast"]

    import random

    random.seed(42)

    for offer_id in offer_ids:
        for role in roles:
            ttft = random.uniform(500, 3000)
            itl = random.uniform(20, 100)
            tps = random.uniform(30, 80)
            err_rate = random.uniform(0, 0.05)

            cursor.execute(
                """
                INSERT OR IGNORE INTO telemetry (offer_id, role_id, window, sample_n,
                    ttft_p95_ms, itl_p95_ms, tps_stream_p50, error_rate_total,
                    schema_adherence, speed_score_p95, success_rate, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    offer_id,
                    role,
                    "1h",
                    random.randint(100, 1000),
                    ttft,
                    itl,
                    tps,
                    err_rate,
                    1 - err_rate,
                    ttft + 1000,
                    1 - err_rate,
                    now,
                ),
            )

    conn.commit()
    conn.close()


if __name__ == "__main__":
    init_db()
    generate_sample_data()
