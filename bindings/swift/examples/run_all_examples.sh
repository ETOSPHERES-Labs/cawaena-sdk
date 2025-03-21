#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Array of all example names to run
examples=(
    "01_create_new_user"
    # "02_onboard_user_postident"
    "03_create_new_wallet"
    "04_migrate_wallet_from_mnemonic"
    "05_migrate_wallet_from_backup"
    "06_generate_new_address"
    "07_get_balance"
    "08_create_purchase_request"
    # "09_onboard_user_viviswap"
    "10_verify_pin"
    "11_reset_pin"
    "12_change_password"
    "13_send_amount"
    "14_get_exchange_rate"
    "16_get_tx_list"
    "18_delete_user"
    "19_get_wallet_tx_list"
    # "20_send_compliment"
)

# Loop through and run each example
for example in "${examples[@]}"
do
    echo "----------------------------------- Running $example ------------------------------------------------"
    swift run $example
done

echo "All examples finished successfully."
