from harness.interfaces import CanonicalCommand, EvidenceBucket
from harness.runner import Runner, RunnerConfig


def test_runner_writes_logs(tmp_path):
    command = CanonicalCommand(command="echo hi", bucket=EvidenceBucket.RUNTIME, cwd=".")
    r = Runner(RunnerConfig(timeout_seconds=2, continue_on_fail=True)).run_profile([command], str(tmp_path))
    assert len(r) == 1
    assert r[0].stdout_file


def test_runner_retries_and_timeout(tmp_path):
    command = CanonicalCommand(command="sleep 2", bucket=EvidenceBucket.RUNTIME, cwd=".")
    runner = Runner(RunnerConfig(timeout_seconds=1, retries=1, retry_delay_seconds=0))
    run = runner.run_profile([command], str(tmp_path))[0]
    assert run.returncode == 124
    assert run.attempts == 2
    assert run.timed_out is True


def test_runner_budget_termination(tmp_path):
    command = CanonicalCommand(command="echo first", bucket=EvidenceBucket.RUNTIME, cwd=".")
    command2 = CanonicalCommand(command="echo second", bucket=EvidenceBucket.RUNTIME, cwd=".")
    runner = Runner(RunnerConfig(timeout_seconds=2, budget_seconds=0))
    run = runner.run_profile([command, command2], str(tmp_path))
    assert len(run) == 1
    assert run[0].returncode == 0
