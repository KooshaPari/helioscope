"""UI components for Helios Router Dashboard"""

from typing import Any

import pandas as pd
import streamlit as st


def render_sidebar_controls() -> dict[str, Any]:
    """Render sidebar controls and return values."""
    st.sidebar.header("Budget Controls")
    budget_monthly = st.sidebar.number_input("Monthly Budget ($)", value=600.0, step=10.0)
    spend_to_date = st.sidebar.number_input("Spend to Date ($)", value=0.0, step=10.0)
    day_of_month = st.sidebar.number_input("Day of Month", 1, 31, 15)
    days_in_month = st.sidebar.number_input("Days in Month", 28, 31, 30)

    st.sidebar.header("Request Sizing")
    tok_in = st.sidebar.number_input("Est. Input Tokens", value=12000, step=500)
    tok_out = st.sidebar.number_input("Est. Output Tokens", value=2500, step=250)

    st.sidebar.header("Pareto Settings")
    minimize_cost = st.sidebar.checkbox("Minimize Cost", value=True)
    minimize_speed = st.sidebar.checkbox("Minimize Speed", value=True)
    maximize_quality = st.sidebar.checkbox("Maximize Quality", value=True)
    max_combo_n = st.sidebar.number_input("Max offers for pairs/trios", 5, 80, 25)
    allow_pairs = st.sidebar.checkbox("Compute Pairs", value=True)
    allow_trios = st.sidebar.checkbox("Compute Trios", value=False)

    return {
        "budget_monthly": budget_monthly,
        "spend_to_date": spend_to_date,
        "day_of_month": day_of_month,
        "days_in_month": days_in_month,
        "tok_in": tok_in,
        "tok_out": tok_out,
        "minimize_cost": minimize_cost,
        "minimize_speed": minimize_speed,
        "maximize_quality": maximize_quality,
        "max_combo_n": max_combo_n,
        "allow_pairs": allow_pairs,
        "allow_trios": allow_trios,
    }


def render_data_editor(df: pd.DataFrame, key: str, column_config: dict[str, Any] = None) -> pd.DataFrame:
    """Render editable dataframe with config."""
    return st.data_editor(df, num_rows="dynamic", use_container_width=True, key=key, column_config=column_config or {})


def render_weight_sliders(weights: dict[str, float], key_prefix: str = "") -> dict[str, float]:
    """Render visual weight sliders."""
    new_weights = {}
    for name, value in weights.items():
        label = name.replace("_", " ").title()
        new_weights[name] = st.slider(label, 0.0, 1.0, float(value), 0.01, key=f"{key_prefix}_{name}")
    return new_weights


def render_pareto_table(df: pd.DataFrame) -> None:
    """Render Pareto results table with metrics."""
    col1, col2 = st.columns([3, 1])
    with col1:
        st.dataframe(
            df.sort_values(["on_pareto", "quality"], ascending=[False, False]), use_container_width=True, height=300
        )
    with col2:
        count = df["on_pareto"].sum() if "on_pareto" in df.columns else 0
        st.metric("Pareto Offers", count)


def render_metrics_row(data: dict[str, Any]) -> None:
    """Render metrics row."""
    cols = st.columns(len(data))
    for i, (label, value) in enumerate(data.items()):
        with cols[i]:
            st.metric(label, value)
