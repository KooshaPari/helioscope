"""
Helios Router Data Management - SQLite Database Schema
"""

import sqlite3
from datetime import datetime
from pathlib import Path

DB_PATH = Path(__file__).parent / "helios.db"


def get_connection():
    return sqlite3.connect(DB_PATH)


def init_db():
    """Initialize the database with all required tables."""
    conn = get_connection()
    cursor = conn.cursor()

    # Meta table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS meta (
            key TEXT PRIMARY KEY,
            value TEXT,
            updated_at TEXT
        )
    """)

    # Providers table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS providers (
            provider_id TEXT PRIMARY KEY,
            display_name TEXT,
            provider_type TEXT,
            api_style TEXT,
            auth_methods TEXT,
            regions TEXT,
            volatility_score REAL DEFAULT 0.2,
            notes TEXT,
            created_at TEXT,
            updated_at TEXT
        )
    """)

    # Plans table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS plans (
            plan_id TEXT PRIMARY KEY,
            provider_id TEXT,
            plan_type TEXT DEFAULT 'payg_token',
            plan_display_name TEXT,
            monthly_fee_usd REAL DEFAULT 0,
            included_tokens_per_month INTEGER,
            tokens_per_day_cap INTEGER,
            included_units_per_month INTEGER,
            unit_overage_usd REAL DEFAULT 0,
            tokens_used_today INTEGER DEFAULT 0,
            units_used_mtd INTEGER DEFAULT 0,
            tokens_remaining_today INTEGER,
            units_remaining INTEGER,
            plan_shadow REAL DEFAULT 1.0,
            created_at TEXT,
            updated_at TEXT
        )
    """)

    # Offers table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS offers (
            offer_id TEXT PRIMARY KEY,
            provider_id TEXT,
            model_id TEXT,
            model_family TEXT,
            model_variant TEXT,
            plan_id TEXT,
            region TEXT,
            endpoint_url TEXT,
            lifecycle_state TEXT DEFAULT 'active',
            allowed_roles TEXT,
            max_context_tokens INTEGER,
            max_output_tokens INTEGER,
            price_in_usd_per_mtok REAL,
            price_out_usd_per_mtok REAL,
            cache_read_usd_per_mtok REAL,
            cache_write_usd_per_mtok REAL,
            rpm_limit INTEGER,
            tpm_limit INTEGER,
            unit_multiplier REAL DEFAULT 1.0,
            is_included_0x INTEGER DEFAULT 0,
            volatility_score REAL DEFAULT 0.2,
            prompt_contract_type TEXT,
            supports_tools INTEGER DEFAULT 0,
            supports_json_mode INTEGER DEFAULT 0,
            supports_streaming INTEGER DEFAULT 1,
            notes TEXT,
            created_at TEXT,
            updated_at TEXT,
            FOREIGN KEY (provider_id) REFERENCES providers(provider_id),
            FOREIGN KEY (plan_id) REFERENCES plans(plan_id)
        )
    """)

    # Roles table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS roles (
            role_id TEXT PRIMARY KEY,
            description TEXT,
            role_class TEXT,
            speed_mode TEXT DEFAULT 'bulk',
            requires_tools INTEGER DEFAULT 0,
            requires_json_mode INTEGER DEFAULT 0,
            min_quality_default REAL DEFAULT 0.0,
            max_cost_usd_default REAL DEFAULT 1.0,
            lexi_order TEXT DEFAULT 'quality_desc,cost_asc',
            w_coding_synthesis REAL DEFAULT 0.20,
            w_coding_debugging REAL DEFAULT 0.20,
            w_coding_repo_swe REAL DEFAULT 0.20,
            w_coding_editing_apply REAL DEFAULT 0.15,
            w_tool_use_agents REAL DEFAULT 0.15,
            w_instruction_following_format REAL DEFAULT 0.10,
            w_long_context_retrieval REAL DEFAULT 0.0,
            w_math_competition REAL DEFAULT 0.0,
            w_general_reasoning_knowledge REAL DEFAULT 0.0,
            w_multilingual REAL DEFAULT 0.0,
            w_safety_alignment REAL DEFAULT 0.0,
            w_truthfulness_hallucination REAL DEFAULT 0.0,
            w_multimodal_vision REAL DEFAULT 0.0,
            online_weight_ramp_k INTEGER DEFAULT 200,
            created_at TEXT,
            updated_at TEXT
        )
    """)

    # Benchmarks table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS benchmarks (
            benchmark_id TEXT PRIMARY KEY,
            benchmark_name TEXT,
            benchmark_version TEXT,
            publisher TEXT,
            benchmark_type_group TEXT,
            primary_domain TEXT,
            task_format TEXT,
            scoring_direction TEXT DEFAULT 'higher_better',
            score_unit TEXT,
            difficulty_band TEXT,
            skill_tags TEXT,
            normalization_method TEXT,
            notes TEXT,
            created_at TEXT,
            updated_at TEXT
        )
    """)

    # Benchmark Results table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS bench_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subject_type TEXT,
            subject_id TEXT,
            benchmark_id TEXT,
            score_value REAL,
            score_unit TEXT,
            evaluation_date TEXT,
            source_type TEXT,
            source_ref TEXT,
            confidence REAL DEFAULT 1.0,
            prompting_style TEXT,
            temperature REAL,
            notes TEXT,
            created_at TEXT,
            FOREIGN KEY (benchmark_id) REFERENCES benchmarks(benchmark_id)
        )
    """)

    # Telemetry table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS telemetry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            offer_id TEXT,
            role_id TEXT,
            window TEXT DEFAULT '1h',
            sample_n INTEGER DEFAULT 0,
            queue_p50_ms REAL,
            queue_p95_ms REAL,
            ttft_p50_ms REAL,
            ttft_p95_ms REAL,
            itl_p50_ms REAL,
            itl_p95_ms REAL,
            tps_stream_p50 REAL,
            tps_stream_p95 REAL,
            speed_score_p50 REAL,
            speed_score_p95 REAL,
            success_rate REAL,
            error_rate_total REAL,
            error_rate_429 REAL,
            error_rate_5xx REAL,
            schema_adherence REAL,
            tokens_in_mean REAL,
            tokens_out_mean REAL,
            tokens_out_p95 REAL,
            escalation_rate REAL,
            apply_p95_ms_per_file REAL,
            as_of TEXT,
            created_at TEXT,
            FOREIGN KEY (offer_id) REFERENCES offers(offer_id),
            FOREIGN KEY (role_id) REFERENCES roles(role_id)
        )
    """)

    # Indices cache table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS indices_cache (
            offer_id TEXT,
            role_id TEXT,
            quality REAL,
            quality_source TEXT,
            offline_quality REAL,
            coverage REAL,
            online_quality REAL,
            online_n REAL,
            cost_usd REAL,
            speed_score REAL,
            computed_at TEXT,
            PRIMARY KEY (offer_id, role_id)
        )
    """)

    # Pareto cache table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS pareto_cache (
            role_id TEXT,
            combo_size INTEGER,
            offer_ids TEXT,
            providers TEXT,
            models TEXT,
            quality REAL,
            cost_usd REAL,
            speed_score REAL,
            on_pareto INTEGER,
            computed_at TEXT,
            PRIMARY KEY (role_id, combo_size, offer_ids)
        )
    """)

    # Audit log table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT,
            user TEXT,
            table_name TEXT,
            action TEXT,
            key TEXT,
            before_json TEXT,
            after_json TEXT
        )
    """)

    # Insert default meta values
    now = datetime.now().isoformat()
    cursor.execute(
        """
        INSERT OR IGNORE INTO meta (key, value, updated_at) VALUES
        ('schema_version', '1.0', ?),
        ('default_budget', '600', ?),
        ('days_in_month', '30', ?),
        ('budget_shadow_eps', '0.12', ?)
    """,
        (now, now, now, now),
    )

    # Insert default roles if not exists
    default_roles = [
        ("code_reasoner", "Code reasoning/analysis", "reasoning", "bulk", 0.0, 0.5),
        ("code_patch_generator", "Code patch generation", "generation", "bulk", 0.0, 0.25),
        ("code_apply_patch", "Code apply/patch execution", "apply", "apply", 0.0, 0.1),
        ("code_scaffold_fast", "Fast code scaffolding", "generation", "interactive", 0.0, 0.15),
        ("code_small_transform", "Small code transformations", "editing", "bulk", 0.0, 0.2),
    ]

    for role_id, desc, role_class, speed_mode, min_q, max_c in default_roles:
        cursor.execute(
            """
            INSERT OR IGNORE INTO roles (role_id, description, role_class, speed_mode,
                min_quality_default, max_cost_usd_default, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (role_id, desc, role_class, speed_mode, min_q, max_c, now, now),
        )

    conn.commit()
    conn.close()


def log_audit(table_name: str, action: str, key: str, before: dict = None, after: dict = None):
    """Log an audit entry."""
    import json

    conn = get_connection()
    cursor = conn.cursor()
    now = datetime.now().isoformat()
    cursor.execute(
        """
        INSERT INTO audit_log (timestamp, user, table_name, action, key, before_json, after_json)
        VALUES (?, ?, ?, ?, ?, ?, ?)
    """,
        (
            now,
            "local_user",
            table_name,
            action,
            key,
            json.dumps(before) if before else None,
            json.dumps(after) if after else None,
        ),
    )
    conn.commit()
    conn.close()


if __name__ == "__main__":
    init_db()
