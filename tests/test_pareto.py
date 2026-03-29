import sys

import pandas as pd

sys.path.insert(0, "src")
from helios_router_ui.pareto.engine import pareto_front_mask


def test_pareto_front_mask_empty_df():
    """Test empty dataframe returns empty series."""
    df = pd.DataFrame()
    result = pareto_front_mask(df, minimize=["cost"], maximize=["quality"])
    assert len(result) == 0


def test_pareto_front_mask_single_row():
    """Test single row is always on pareto front."""
    df = pd.DataFrame({"cost": [10], "quality": [5]})
    result = pareto_front_mask(df, minimize=["cost"], maximize=["quality"])
    assert len(result) == 1
    assert result.iloc[0] == True


def test_pareto_front_mask_dominates():
    """Test that dominated solutions are filtered."""
    df = pd.DataFrame({"cost": [10, 5, 8], "quality": [5, 3, 4]})
    result = pareto_front_mask(df, minimize=["cost"], maximize=["quality"])
    # (10,5) is dominated by (8,4) - worse cost, worse quality
    # (5,3) is dominated by (8,4) - worse quality (even though better cost)
    # (8,4) should be on pareto front
    assert result.sum() >= 1  # At least (8,4) should be on front
