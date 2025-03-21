#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Array of example file paths
examples=(
    "01-create_new_user.ts"
    # "02-onboard_user_via_postident.ts"
    "03-create_new_wallet.ts"
    "04-migrate_wallet_from_mnemonic.ts"
    "05-migrate_wallet_from_backup.ts"
    "06-generate_new_address.ts"
    "07-get_balance.ts"
    "08-create_purchase_request.ts"
    # "09-onboard_user_via_viviswap.ts"
    "10-verify_pin.ts"
    "11-reset_pin.ts"
    "12-change_password.ts"
    "13-send_amount.ts"
    "14-get_exchange_rate.ts"
    "16-get_tx_list.ts"
    "18-delete_user.ts"
    "19-get_wallet_tx_list.ts"
    # "20-send_compliment.ts"
)

# Iterate through each example and run it
for example in "${examples[@]}"
do
    echo "------------------------------ Running example: $example -----------------------------------------------------------"
    bun "$example"
done

echo "All examples finished successfully."
