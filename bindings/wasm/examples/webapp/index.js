
// Step 1: import the main method from the example file
import { main as create_new_user } from './examples/01-create_new_user';
import { main as onboard_user_via_postident } from './examples/02-onboard_user_via_postident';
import { main as create_new_wallet } from './examples/03-create_new_wallet';
import { main as migrate_wallet_from_mnemonic } from './examples/04-migrate_wallet_from_mnemonic';
import { main as migrate_wallet_from_backup } from './examples/05-migrate_wallet_from_backup';
import { main as generate_new_iota_address } from './examples/06-generate_new_address';
import { main as get_balance } from './examples/07-get_balance';
import { main as create_purchase_request } from './examples/08-create_purchase_request';
import { main as onboard_user_via_viviswap } from './examples/09-onboard_user_via_viviswap';
import { main as verify_pin } from './examples/10-verify_pin';
import { main as reset_pin } from './examples/11-reset_pin';
import { main as change_password } from './examples/12-change_password';
import { main as send_amount } from './examples/13-send_amount';
import { main as get_exchange_rate } from './examples/14-get_exchange_rate';
import { main as get_tx_list } from './examples/16-get_tx_list'
import { main as delete_user } from './examples/18-delete_user';
import { main as get_wallet_tx_list } from './examples/19-get_wallet_tx_list'
import { main as send_compliment } from './examples/20-send_compliment';

// Step 2: populate this array with the name of the file and the main entry function
const examples = [
    ['01-create_new_user.js', create_new_user],
    ['02-onboard_user_via_postident.js', onboard_user_via_postident],
    ['03-create_new_wallet.js', create_new_wallet],
    ['04-migrate_wallet_from_mnemonic.js', migrate_wallet_from_mnemonic],
    ['05-migrate_wallet_from_backup.js', migrate_wallet_from_backup],
    ['06-generate_new_address.js', generate_new_iota_address],
    ['07-get_balance.js', get_balance],
    ['08-create_purchase_request.js', create_purchase_request],
    ['09-onboard_user_via_viviswap.js', onboard_user_via_viviswap],
    ['10-verify_pin.js', verify_pin],
    ['11-reset_pin.js', reset_pin],
    ['12-change_password.js', change_password],
    ['13-send_amount.js', send_amount],
    ['14-get_exchange_rate.js', get_exchange_rate],
    ['16-get_tx_list.js', get_tx_list],
    ['18-delete_user.js', delete_user],
    ['19-wallet_tx_list.js', get_wallet_tx_list],
    ['20-send_compliment.js', send_compliment],
];


let div = document.getElementById("buttons");

for (let e of examples) {
    let btn = document.createElement("button");
    btn.innerText = e[0]; // button text
    btn.onclick = function () {
        // clear the local storage first
        try {
            window.localStorage.clear();
        } catch (e) {
            console.log("Could not clear local storage: ", e);
        }
        handleError(e[1]); // call the main function on click
    }
    div.appendChild(btn);
    div.appendChild(document.createElement('br'));
}


async function handleError(fn) {
    try {
        await fn();
    } catch (error) {
        console.error(error);
    }
}
