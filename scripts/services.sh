#!/usr/bin/env bash
# Helios Router - Service Lifecycle Management
# Source this file or run functions directly

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOGS_DIR="${SCRIPT_DIR}/.process-compose/logs"
LOCKS_DIR="${SCRIPT_DIR}/.process-compose/locks"
NATSD_PID_FILE="${LOCKS_DIR}/nats.pid"
STREAMLIT_PID_FILE="${LOCKS_DIR}/streamlit.pid"
NATS_PORT="${HELIOS_NATS_PORT:-4222}"
NATS_HTTP_PORT="${HELIOS_NATS_HTTP_PORT:-8222}"
STREAMLIT_PORT="${HELIOS_STREAMLIT_PORT:-8501}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

# Ensure directories exist
ensure_dirs() {
    mkdir -p "$LOGS_DIR" "$LOCKS_DIR"
}

# Preflight: Check dependencies
preflight() {
    log_info "Running preflight checks..."
    
    # Check NATS
    if ! command -v nats-server &> /dev/null; then
        log_error "nats-server not found"
        log_info "Install with: brew install nats-server"
        return 1
    fi
    
    # Check Python
    if ! command -v python3 &> /dev/null; then
        log_error "python3 not found"
        return 1
    fi
    
    # Check uv or pip
    if ! command -v uv &> /dev/null && ! command -v pip &> /dev/null; then
        log_warn "Neither uv nor pip found"
    fi
    
    log_info "Preflight OK"
    return 0
}

# Start NATS server
start_nats() {
    ensure_dirs
    
    if pgrep -f "nats-server" > /dev/null 2>&1; then
        log_info "NATS already running"
        return 0
    fi
    
    log_info "Starting NATS server..."
    nats-server -js -p "$NATS_PORT" -m "$NATS_HTTP_PORT" >> "$LOGS_DIR/nats.log" 2>&1 &
    NATS_PID=$!
    echo "$NATS_PID" > "$NATSD_PID_FILE"
    
    # Wait for NATS to be ready
    local retries=10
    while [ $retries -gt 0 ]; do
        if nats-server -v &> /dev/null; then
            log_info "NATS server started (PID: $NATS_PID)"
            return 0
        fi
        sleep 0.5
        ((retries--))
    done
    
    log_error "Failed to start NATS"
    return 1
}

# Stop NATS server
stop_nats() {
    if [ -f "$NATSD_PID_FILE" ]; then
        local pid
        pid=$(cat "$NATSD_PID_FILE")
        if kill -0 "$pid" 2>/dev/null; then
            log_info "Stopping NATS (PID: $pid)..."
            kill "$pid" 2>/dev/null || true
        fi
        rm -f "$NATSD_PID_FILE"
    fi
    
    # Fallback: kill any remaining
    pgrep -f "nats-server" | xargs kill 2>/dev/null || true
    log_info "NATS stopped"
}

# Start Streamlit
start_streamlit() {
    ensure_dirs
    
    if pgrep -f "streamlit run" > /dev/null 2>&1; then
        log_info "Streamlit already running"
        return 0
    fi
    
    log_info "Starting Streamlit..."
    cd "$SCRIPT_DIR"
    
    PYTHONPATH="$SCRIPT_DIR/src:$PYTHONPATH" \
        python3 -m streamlit run app.py --server.port "$STREAMLIT_PORT" >> "$LOGS_DIR/streamlit.log" 2>&1 &
    STREAMLIT_PID=$!
    echo "$STREAMLIT_PID" > "$STREAMLIT_PID_FILE"
    
    log_info "Streamlit started on http://localhost:${STREAMLIT_PORT} (PID: $STREAMLIT_PID)"
}

# Stop Streamlit
stop_streamlit() {
    if [ -f "$STREAMLIT_PID_FILE" ]; then
        local pid
        pid=$(cat "$STREAMLIT_PID_FILE")
        if kill -0 "$pid" 2>/dev/null; then
            log_info "Stopping Streamlit (PID: $pid)..."
            kill "$pid" 2>/dev/null || true
        fi
        rm -f "$STREAMLIT_PID_FILE"
    fi
    
    pgrep -f "streamlit run" | xargs kill 2>/dev/null || true
    log_info "Streamlit stopped"
}

# Check service status
status() {
    echo "=== Helios Router Status ==="
    echo ""
    
    # NATS
    echo -n "NATS: "
    if pgrep -f "nats-server" > /dev/null 2>&1; then
        echo -e "${GREEN}running${NC}"
    else
        echo -e "${RED}stopped${NC}"
    fi
    
    # Streamlit
    echo -n "Streamlit: "
    if pgrep -f "streamlit" > /dev/null 2>&1; then
        echo -e "${GREEN}running${NC} (http://localhost:${STREAMLIT_PORT})"
    else
        echo -e "${RED}stopped${NC}"
    fi
    
    # Health checks
    echo ""
    echo "=== Health ==="
    if nc -z localhost "$NATS_PORT" 2>/dev/null; then
        echo -e "NATS port ${NATS_PORT}: ${GREEN}OK${NC}"
    else
        echo -e "NATS port ${NATS_PORT}: ${RED}DOWN${NC}"
    fi
}

# Tail logs
logs() {
    if [ $# -eq 0 ]; then
        tail -f "$LOGS_DIR"/*.log
    else
        tail -f "$LOGS_DIR/$1.log"
    fi
}

# Full start
start() {
    preflight || exit 1
    start_nats
    start_streamlit
    status
}

# Full stop
stop() {
    stop_streamlit
    stop_nats
    log_info "All services stopped"
}

# Restart
restart() {
    stop
    sleep 1
    start
}

# Help
help() {
    echo "Helios Router Service Management"
    echo ""
    echo "Usage: source scripts/services.sh && <command>"
    echo "   or: bash scripts/services.sh <command>"
    echo ""
    echo "Commands:"
    echo "  start     - Start all services"
    echo "  stop      - Stop all services"
    echo "  restart   - Restart all services"
    echo "  status    - Show service status"
    echo "  logs      - Tail logs (optional: <service>)"
    echo "  preflight - Run preflight checks"
    echo "  help      - Show this help"
}

# Main
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    case "${1:-help}" in
        start) start ;;
        stop) stop ;;
        restart) restart ;;
        status) status ;;
        logs) logs "${2:-}" ;;
        preflight) preflight ;;
        help|--help|-h) help ;;
        *) log_error "Unknown command: $1"; help; exit 1 ;;
    esac
fi
