use std::ops::Div;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction, message::Message, transaction::Transaction,
};
use solana_sdk::native_token::sol_to_lamports;
use web_time::Duration;

use crate::{
    components::InvokeSignature,
    gateway::{escrow_pubkey, GatewayError, GatewayResult},
    hooks::{
        use_escrow, use_gateway, use_miner_toolbar_state,
        use_wallet_adapter::{use_wallet_adapter, InvokeSignatureStatus, WalletAdapter},
        ReadMinerToolbarState,
    },
    miner::Miner,
};

#[component]
pub fn MinerToolbarTopUp(sol_balance: Resource<GatewayResult<u64>>) -> Element {
    let toolbar_state = use_miner_toolbar_state();
    rsx! {
        // if toolbar_state.is_open() {
        //     MinerToolbarTopUpOpen {
        //         sol_balance: sol_balance.clone()
        //     }
        // } else {
            div {
                class: "flex flex-row font-semibold justify-end w-full h-full px-4 sm:px-8 pt-5 pointer-events-none",
                span {
                    class: "font-semibold",
                    "Create account →"
                }
            }
        // }
    }
}

#[component]
pub fn MinerToolbarTopUpOpen(sol_balance: Resource<GatewayResult<u64>>) -> Element {
    let wallet_adapter = use_wallet_adapter();
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let tx = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
            WalletAdapter::Connected(signer) => {
                let gateway = use_gateway();
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(50_000);
                let amount = sol_to_lamports(0.05);
                let ix_1 = solana_client_wasm::solana_sdk::system_instruction::transfer(
                    &signer,
                    &escrow_pubkey(signer),
                    amount,
                );
                let ix_2 = solana_client_wasm::solana_sdk::system_instruction::transfer(
                    &signer,
                    &ore_relayer_api::consts::COLLECTOR_ADDRESS,
                    amount.div(50), // 2% fee
                );
                let blockhash = gateway.rpc.get_latest_blockhash().await?;
                let ixs = vec![cu_limit_ix, ix_1, ix_2];
                let msg = Message::new_with_blockhash(ixs.as_slice(), Some(&signer), &blockhash);
                let tx = Transaction::new_unsigned(msg);
                Ok(tx)
            }
        }
    });

    let _ = use_resource(move || async move {
        if let InvokeSignatureStatus::Done(sig) = *invoke_signature_signal.read() {
            if let WalletAdapter::Connected(signer) = *wallet_adapter.read() {
                let gateway = use_gateway();
                async_std::task::sleep(Duration::from_millis(1000)).await;
                sol_balance.restart();
            }
        };
        ()
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full grow gap-12 sm:gap-16 justify-between px-4 sm:px-8 py-8",
            div {
                class: "flex flex-col gap-2",
                p {
                    class: "text-3xl md:text-4xl lg:text-5xl font-bold",
                    "Pay transaction fees"
                }
                p {
                    class: "text-lg",
                    "Top up your account to pay for blockchain transaction fees."
                }
                p {
                    class: "text-sm text-gray-300",
                    "This will fund your account to automate mining."
                }
            }
            div {
                class: "flex flex-col gap-4",
                if let Some(Ok(tx)) = tx.cloned() {
                    InvokeSignature { tx: tx, signal: invoke_signature_signal, start_msg: "Top up" }
                } else {
                    p {
                        class: "font-medium text-center text-sm text-gray-300 hover:underline",
                        "Loading..."
                    }
                }
                a {
                    // TODO Get referal code
                    href: "https://www.coinbase.com/price/solana",
                    target: "_blank",
                    class: "font-medium text-center text-sm text-gray-300 hover:underline",
                    "Help! I don't have any SOL."
                }
            }
        }
    }
}

#[component]
pub fn MinerToolbarCreateAccount(miner: Signal<Miner>) -> Element {
    let toolbar_state = use_miner_toolbar_state();
    rsx! {
        // if toolbar_state.is_open() {
        //     MinerToolbarCreateAccountOpen {
        //         miner
        //     }
        // } else {
            div {
                class: "flex flex-row font-semibold justify-end w-full h-full px-4 sm:px-8 pt-5 pointer-events-none",
                span {
                    class: "font-semibold",
                    "Create account →"
                }
            }
        // }
    }
}

#[component]
pub fn MinerToolbarCreateAccountOpen(miner: Signal<Miner>) -> Element {
    let wallet_adapter = use_wallet_adapter();
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let mut escrow = use_escrow();
    let tx = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
            WalletAdapter::Connected(signer) => {
                let gateway = use_gateway();
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(500_000);
                let amount = sol_to_lamports(0.05);
                let ix_1 = ore_relayer_api::instruction::open_escrow(signer, signer);
                let ix_2 = solana_client_wasm::solana_sdk::system_instruction::transfer(
                    &signer,
                    &escrow_pubkey(signer),
                    amount,
                );
                let ix_3 = solana_client_wasm::solana_sdk::system_instruction::transfer(
                    &signer,
                    &ore_relayer_api::consts::COLLECTOR_ADDRESS,
                    amount.div(50), // 2% fee
                );
                let blockhash = gateway.rpc.get_latest_blockhash().await?;
                let ixs = vec![cu_limit_ix, ix_1, ix_2, ix_3];
                let msg = Message::new_with_blockhash(ixs.as_slice(), Some(&signer), &blockhash);
                let tx = Transaction::new_unsigned(msg);
                Ok(tx)
            }
        }
    });

    let _ = use_resource(move || async move {
        if let InvokeSignatureStatus::Done(sig) = *invoke_signature_signal.read() {
            if let WalletAdapter::Connected(signer) = *wallet_adapter.read() {
                let gateway = use_gateway();
                async_std::task::sleep(Duration::from_millis(1000)).await;
                if let Ok(new_escrow) = gateway.get_escrow(signer).await {
                    escrow.set(new_escrow);
                }
            }
        };
        ()
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full grow gap-12 sm:gap-16 justify-between px-4 sm:px-8 py-8",
            div {
                class: "flex flex-col gap-2",
                p {
                    class: "text-3xl md:text-4xl lg:text-5xl font-bold",
                    "Create an account"
                }
                p {
                    class: "text-lg",
                    "Open a new account to start mining ORE."
                }
                p {
                    class: "text-sm text-gray-300",
                    "This account will secure your progress and miner rewards."
                }
            }
            div {
                class: "flex flex-col gap-4",
                if let Some(Ok(tx)) = tx.cloned() {
                    InvokeSignature { tx: tx, signal: invoke_signature_signal, start_msg: "Create account" }
                } else {
                    p {
                        class: "font-medium text-center text-sm text-gray-300 hover:underline",
                        "Loading..."
                    }
                }
                a {
                    // TODO Get referal code
                    href: "https://www.coinbase.com/price/solana",
                    target: "_blank",
                    class: "font-medium text-center text-sm text-gray-300 hover:underline",
                    "Help! I don't have any SOL."
                }
            }
        }
    }
}
