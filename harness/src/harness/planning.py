"""Planning and Execution Separation - Agent planning workflow.

Provides infrastructure for separating planning from execution with
dynamic plan updates.
"""

import json
import threading
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable, Optional

from .id_utils import new_id


class PlanState(Enum):
    """State of a plan."""

    DRAFT = "draft"
    APPROVED = "approved"
    EXECUTING = "executing"
    COMPLETED = "completed"
    FAILED = "failed"
    UPDATED = "updated"


class StepState(Enum):
    """State of a plan step."""

    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    SKIPPED = "skipped"


@dataclass
class PlanStep:
    """A single step in a plan."""

    id: str
    description: str
    status: StepState = StepState.PENDING
    result: Any = None
    error: Optional[str] = None
    started_at: Optional[float] = None
    completed_at: Optional[float] = None
    dependencies: list[str] = field(default_factory=list)
    metadata: dict = field(default_factory=dict)


@dataclass
class ExecutionPlan:
    """A complete execution plan."""

    id: str
    goal: str
    steps: list[PlanStep] = field(default_factory=list)
    state: PlanState = PlanState.DRAFT
    created_at: float = field(default_factory=time.time)
    updated_at: float = field(default_factory=time.time)
    current_step: Optional[str] = None
    metadata: dict = field(default_factory=dict)


class PlanExecutor:
    """Executes plans with step-by-step execution.

    Usage:
        executor = PlanExecutor()

        # Create plan
        plan = executor.create_plan(\"Build a web app\", [
            {\"id\": \"step1\", \"description\": \"Setup project\"},
            {\"id\": \"step2\", \"description\": \"Implement backend\"},
        ])

        # Execute
        async for step in executor.execute(plan):
            print(f\"Completed: {step.description}\")
    """

    def __init__(self):
        self._plans: dict[str, ExecutionPlan] = {}
        self._step_handlers: dict[str, Callable] = {}
        self._lock = threading.RLock()

    def register_handler(self, step_type: str, handler: Callable) -> None:
        """Register a handler for a step type."""
        self._step_handlers[step_type] = handler

    def create_plan(
        self,
        goal: str,
        steps: list[dict],
        metadata: Optional[dict] = None,
    ) -> ExecutionPlan:
        """Create a new execution plan."""
        plan_id = new_id().replace("-", "")

        plan_steps = [
            PlanStep(
                id=step.get("id", f"step_{i}"),
                description=step.get("description", ""),
                dependencies=step.get("dependencies", []),
                metadata=step.get("metadata", {}),
            )
            for i, step in enumerate(steps)
        ]

        plan = ExecutionPlan(
            id=plan_id,
            goal=goal,
            steps=plan_steps,
            metadata=metadata or {},
        )

        with self._lock:
            self._plans[plan_id] = plan

        return plan

    def update_plan(self, plan_id: str, new_steps: list[dict]) -> Optional[ExecutionPlan]:
        """Update plan with new steps (dynamic re-planning)."""
        with self._lock:
            plan = self._plans.get(plan_id)
            if not plan:
                return None

            # Mark as updated
            plan.state = PlanState.UPDATED
            plan.updated_at = time.time()

            # Add new steps
            for step_data in new_steps:
                plan.steps.append(
                    PlanStep(
                        id=step_data.get("id", f"step_{len(plan.steps)}"),
                        description=step_data.get("description", ""),
                        dependencies=step_data.get("dependencies", []),
                    )
                )

            return plan

    async def execute(
        self,
        plan_id: str,
        on_step_complete: Optional[Callable] = None,
    ) -> ExecutionPlan:
        """Execute a plan step by step."""
        with self._lock:
            plan = self._plans.get(plan_id)
            if not plan:
                raise ValueError(f"Plan not found: {plan_id}")

            plan.state = PlanState.EXECUTING

        # Execute each step
        for step in plan.steps:
            if step.status == StepState.COMPLETED:
                continue

            # Check dependencies
            if not self._check_dependencies(step, plan.steps):
                step.status = StepState.SKIPPED
                continue

            # Execute step
            step.status = StepState.RUNNING
            step.started_at = time.time()
            plan.current_step = step.id

            try:
                handler = self._step_handlers.get(step.id)
                if handler:
                    step.result = await handler(step)
                else:
                    step.result = f"Executed: {step.description}"

                step.status = StepState.COMPLETED

            except Exception as e:
                step.status = StepState.FAILED
                step.error = str(e)
                plan.state = PlanState.FAILED
                break

            step.completed_at = time.time()

            if on_step_complete:
                await on_step_complete(step)

        # Mark complete
        if plan.state != PlanState.FAILED:
            plan.state = PlanState.COMPLETED

        return plan

    def _check_dependencies(self, step: PlanStep, all_steps: list[PlanStep]) -> bool:
        """Check if step dependencies are met."""
        step_map = {s.id: s for s in all_steps}

        for dep_id in step.dependencies:
            dep_step = step_map.get(dep_id)
            if not dep_step or dep_step.status != StepState.COMPLETED:
                return False

        return True

    def get_plan(self, plan_id: str) -> Optional[ExecutionPlan]:
        """Get a plan by ID."""
        return self._plans.get(plan_id)

    def get_stats(self) -> dict:
        """Get executor statistics."""
        states = {}
        for plan in self._plans.values():
            state = plan.state.value
            states[state] = states.get(state, 0) + 1

        return {
            "total_plans": len(self._plans),
            "by_state": states,
        }


class DynamicPlanner:
    """Creates and updates plans dynamically based on execution feedback.

    Usage:
        planner = DynamicPlanner()

        # Generate initial plan
        plan = await planner.create_initial_plan(\"Build a todo app\")

        # Update based on execution
        if unexpected_condition:
            plan = await planner.update_plan(plan.id, context)
    """

    def __init__(self, llm_handler: Optional[Callable] = None):
        self._llm_handler = llm_handler
        self._executor = PlanExecutor()

    async def create_initial_plan(self, goal: str) -> ExecutionPlan:
        """Create an initial plan from a goal."""
        if not self._llm_handler:
            # Return a simple default plan
            return self._executor.create_plan(
                goal,
                [
                    {"id": "analyze", "description": f"Analyze: {goal}"},
                    {"id": "implement", "description": "Implement solution"},
                    {"id": "verify", "description": "Verify results"},
                ],
            )

        # Use LLM to generate plan
        prompt = f"""Create a step-by-step plan for: {goal}

Respond with JSON:
{{
  \"steps\": [
    {{\"id\": \"step1\", \"description\": \"...\"}},
    ...
  ]
}}"""

        response = await self._llm_handler(prompt)

        try:
            data = json.loads(response)
            return self._executor.create_plan(goal, data.get("steps", []))
        except:
            return self._executor.create_plan(goal, [{"id": "default", "description": "Complete task"}])

    async def update_plan(
        self,
        plan_id: str,
        context: dict,
    ) -> Optional[ExecutionPlan]:
        """Update plan based on execution context."""
        if not self._llm_handler:
            return None

        # Get current plan
        plan = self._executor.get_plan(plan_id)
        if not plan:
            return None

        # Ask LLM to suggest updates
        prompt = f"""Current plan: {[s.description for s in plan.steps]}

Context: {json.dumps(context)}

Should the plan be updated? If yes, respond with:
{{
  \"update\": true,
  \"new_steps\": [
    {{\"id\": \"...\", \"description\": \"...\"}}
  ]
}}

If no:
{{\"update\": false}}"""

        response = await self._llm_handler(prompt)

        try:
            data = json.loads(response)
            if data.get("update"):
                return self._executor.update_plan(plan_id, data.get("new_steps", []))
        except:
            pass

        return plan
