use dioxus::prelude::*;

use crate::components::{AppNavigation, LandingNavigation};
use crate::pages::*;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[layout(LandingNavigation)]
        #[route("/")]
        Landing {},
    #[end_layout]

    #[layout(AppNavigation)]
        #[route("/mine")]
        Mine {},
        #[route("/mine/:pool")]
        Pool { pool: String },
        #[route("/stake")]
        Stake {},
        #[route("/stake/:pair")]
        Pair { pair: String },
        #[route("/trade")]
        Trade {},
        #[route("/trade/:market")]
        Market { market: String },
        #[route("/deposit")]
        Deposit {},
        #[route("/swap")]
        Swap {},
        #[route("/pay")]
        Pay {},
    #[end_layout]


    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}
