#!/bin/bash

PORTS=(6000 6001 6002)
CONFIG_FILE=("config1.json" "config2.json" "config3.json")
PIDS=()

cleanup() {
    echo "Завершение процессов..."
    for pid in "${PIDS[@]}"; do
        kill "$pid" 2>/dev/null && echo "Процесс $pid завершен"
    done
    exit 0
}

trap cleanup SIGINT SIGTERM

for port in "${PORTS[@]}"; do
    cargo run --package smart_house --bin fake_power_socket --profile release -- "$port" &
    PIDS+=($!)
done

for config in "${CONFIG_FILE[@]}"; do
    cargo run --package smart_house --bin fake_thermometer --profile release -- "src/fake_thermometer/$config" &
    PIDS+=($!)
done

sleep 2
echo -e "\nНажмите Ctrl+C для завершения."
wait