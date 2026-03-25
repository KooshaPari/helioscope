"""Pareto computation module - candidate for Rust extraction"""

import numpy as np
import pandas as pd


def pareto_front_mask(df: pd.DataFrame, minimize: list[str], maximize: list[str]) -> pd.Series:
    """O(N^2) Pareto mask. True = non-dominated."""
    if df.empty:
        return pd.Series([], dtype=bool)

    mins = df[minimize].to_numpy(dtype=float) if minimize else np.zeros((len(df), 0))
    maxs = df[maximize].to_numpy(dtype=float) if maximize else np.zeros((len(df), 0))

    n = len(df)
    keep = np.ones(n, dtype=bool)

    for i in range(n):
        if not keep[i]:
            continue
        for j in range(n):
            if i == j or not keep[i]:
                continue
            no_worse_min = np.all(mins[j] <= mins[i]) if mins.shape[1] else True
            no_worse_max = np.all(maxs[j] >= maxs[i]) if maxs.shape[1] else True
            if not (no_worse_min and no_worse_max):
                continue
            strict_better = False
            if mins.shape[1]:
                strict_better = strict_better or np.any(mins[j] < mins[i])
            if maxs.shape[1]:
                strict_better = strict_better or np.any(maxs[j] > maxs[i])
            if strict_better:
                keep[i] = False
    return pd.Series(keep, index=df.index)


def compute_pareto(
    indices_df: pd.DataFrame, minimize_cost: bool = True, minimize_speed: bool = True, maximize_quality: bool = True
) -> pd.DataFrame:
    """Compute Pareto frontier for offers."""
    if indices_df.empty:
        return indices_df

    min_cols, max_cols = [], []
    if minimize_cost:
        min_cols.append("cost_usd")
    if minimize_speed:
        min_cols.append("speed_score")
    if maximize_quality:
        max_cols.append("quality")

    mask = pareto_front_mask(indices_df, minimize=min_cols, maximize=max_cols)

    result = indices_df.copy()
    result["on_pareto"] = mask
    return result


def compute_combos(df: pd.DataFrame, size: int) -> pd.DataFrame:
    """Compute combination indices (pairs/trios)."""
    if len(df) < size:
        return pd.DataFrame()

    from itertools import combinations

    rows = []
    for combo in combinations(df.to_dict("records"), size):
        combo_df = pd.DataFrame(combo)
        rows.append(
            {
                "combo": [c["offer_id"] for c in combo],
                "quality": combo_df["quality"].mean(),
                "cost_usd": combo_df["cost_usd"].sum(),
                "speed_score": combo_df["speed_score"].min(),
                "providers": list(set(combo_df["provider"].tolist())),
                "models": list(set(combo_df["model_id"].tolist())),
            }
        )

    return pd.DataFrame(rows)
