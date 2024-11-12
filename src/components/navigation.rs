use dioxus::prelude::*;

use crate::{
    components::{GlobeIcon, OreWordmarkIcon, WalletAdapter},
    route::Route,
};

pub fn AppNavigation() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen h-full",
            AppNavBar {}
            MobileTabBar {}
            span {
                class: "py-5 sm:py-8",
                Outlet::<Route> {}
            }
        }
    }
}

pub fn LandingNavigation() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen h-screen",
            LandingNavBar {}
            Outlet::<Route> {}
        }
    }
}

fn AppNavBar() -> Element {
    let route: Route = use_route();
    let visible = match route {
        Route::Mine {} | Route::Stake {} | Route::Trade {} => "flex",
        _ => "hidden sm:flex",
    };
    rsx! {
        div {
            class: "{visible} w-screen h-20 px-5 sm:px-8",
            div {
                class: "flex flex-row justify-center sm:justify-between w-full my-auto",
                span {
                    class: "hidden sm:flex my-auto",
                    Logo {}
                }
                TabBar {}
                WalletAdapter {}
            }
        }
    }
}

fn LandingNavBar() -> Element {
    rsx! {
        div {
            class: "flex w-screen shrink-0 h-20 px-5 sm:px-8",
            div {
                class: "flex flex-row w-full my-auto",
                Logo {}
            }
        }
    }
}

fn Logo() -> Element {
    rsx! {
        Link {
            class: "p-1 my-auto rounded hover:bg-gray-800",
            to: Route::Landing {},
            OreWordmarkIcon {
                class: "h-5"
            }
        }
    }
}

fn TabBar() -> Element {
    rsx! {
        div {
            class: "hidden sm:flex flex-row h-full rounded-full elevated overflow-clip",
            Tab {
                title: "Mine",
                route: Route::Mine {}
            }
            Tab {
                title: "Stake",
                route: Route::Stake {}
            }
            Tab {
                title: "Trade",
                route: Route::Trade {}
            }
        }
    }
}

#[component]
fn Tab(title: String, route: Route) -> Element {
    let current_route = use_route();
    let selected = match route {
        Route::Mine {} => current_route == route,
        Route::Stake {} => current_route == route,
        Route::Trade {} => match current_route {
            Route::Trade {} | Route::Asset { asset: _ } => true,
            _ => false,
        },
        _ => false,
    };
    let color = if !selected { "text-gray-700" } else { "" };
    rsx! {
        Link {
            class: "flex px-8 h-10 transition hover:bg-gray-800",
            to: route,
            span {
                class: "text-sm font-medium my-auto font-semibold {color}",
                "{title}"
            }
        }
    }
}

fn MobileTabBar() -> Element {
    rsx! {
        div {
            class: "sm:hidden flex flex-row fixed bottom-0 w-full elevated",
            MobileTab {
                title: "Mine",
                route: Route::Mine {}
            }
            MobileTab {
                title: "Stake",
                route: Route::Stake {}
            }
            MobileTab {
                title: "Trade",
                route: Route::Trade {}
            }
        }
    }
}

#[component]
fn MobileTab(title: String, route: Route) -> Element {
    let current_route: Route = use_route();
    let selected = match route {
        Route::Mine {} => current_route == route,
        Route::Stake {} => current_route == route,
        Route::Trade {} => match current_route {
            Route::Trade {} | Route::Asset { asset: _ } => true,
            _ => false,
        },
        _ => false,
    };
    let color = if !selected { "text-gray-700" } else { "" };
    rsx! {
        Link {
            class: "flex h-16 w-full",
            to: route,
            span {
                class: "flex flex-col gap-1 mx-auto my-auto {color}",
                GlobeIcon {
                    class: "h-6 w-6 mx-auto"
                }
                span {
                    class: "mx-auto font-medium text-xs",
                    "{title}"
                }
            }
        }
    }
}
