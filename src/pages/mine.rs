use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::*,
    route::Route,
};

pub fn Mine() -> Element {
    rsx! {
        Col {
            class: "pb-20 sm:pb-16 gap-8",
            gap: 8,
            Header {}
            Col {
                class: "md:flex-row md:gap-0 px-5 sm:px-8",
                gap: 8,
                Balance {}
                Yield{}
            }
            PoolTable {}
        }
    }
}

fn Header() -> Element {
    rsx! {
        Row {
            class: "justify-between h-10 px-5 sm:px-8",
            gap: 4,
            span {
                class: "font-wide text-2xl sm:text-3xl font-semibold align-text-bottom my-auto",
                "Mine"
            }
            StartButton {}
        }
    }
}

fn StartButton() -> Element {
    rsx! {
        Link {
            to: Route::Swap {},
            class: "h-10 controls-primary rounded-full px-4 gap-2 -mr-2",
            PlayIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
            span {
                class: "my-auto",
                "Start"
            }
        }
    }
}

fn DownloadButton() -> Element {
    rsx! {
        Link {
            to: Route::Download {},
            class: "flex rounded bg-controls-secondary p-2 sm:p-4",
            Row {
                gap: 4,
                DownloadIcon {
                    class: "w-8 h-8 my-auto text-elements-midEmphasis",
                }
                Col {
                    class: "my-auto w-full",
                    span {
                        class: "text-elements-highEmphasis font-semibold",
                        "Download"
                    }
                    span {
                        class: "text-elements-midEmphasis text-sm",
                        "Available for Mac, Windows, and Linux."
                    }
                }
            }
        }
    }
}

fn PoolTable() -> Element {
    // TODO Read from config file
    let listed_pools = vec![
        Pool {
            address: Pubkey::new_unique(),
            name: "Ec1ipse".to_owned(),
            description: "".to_owned(),
            image: "https://pbs.twimg.com/profile_images/1736827532312211456/V0bvyS_2_400x400.jpg"
                .to_owned(),
        },
        Pool {
            address: Pubkey::new_unique(),
            name: "Rush".to_owned(),
            description: "".to_owned(),
            image: "https://pbs.twimg.com/profile_images/1825694276929368064/GJcGr3rR_400x400.jpg"
                .to_owned(),
        },
    ];

    rsx! {
        Col {
            gap: 2, 
            Table {
                header: rsx! {
                    TableHeader {
                        left: "Pool",
                        right_1: "Hashpower",
                        right_2: "Multiplier",
                        right_3: "Yield",
                    }
                },
                rows: rsx! {
                    for pool in listed_pools {
                        PoolRow { pool: pool }
                    }
                }
            }
        }
    }
}

#[component]
fn PoolRow(pool: Pool) -> Element {
    rsx! {
        TableRowLink {
            to: Route::Pool { pool: pool.name.clone() },
            left: rsx! {
                Row {
                    gap: 4,
                    img {
                        class: "w-10 h-10 my-auto bg-gray-900 rounded border border-gray-800",
                        src: "{pool.image}"
                    }
                    Col {
                        class: "my-auto",
                        span {
                            class: "font-medium",
                            "{pool.name}"
                        }
                    }
                }
            },
            right_1: rsx! {
                span {
                    "64480"
                }
            },
            right_2: rsx! {
                span {
                    "2.4x",
                }
            },
            right_3: rsx! {
                span {
                    class: "text-elements-gold",
                    OreValueSmall {
                        ui_amount_string: "2.054"
                    }
                }
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Pool {
    address: Pubkey,
    name: String,
    description: String,
    image: String,
}
