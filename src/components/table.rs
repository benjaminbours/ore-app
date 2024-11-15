use dioxus::prelude::*;

use crate::{
    components::{Col, Row},
    route::Route,
};

#[component]
pub fn Table(header: Element, rows: Element) -> Element {
    rsx! {
        Col {
            class: "sm:mx-5 overflow-x-scroll",
            {header}
            {rows}
        }
    }
}

#[component]
pub fn TableHeader(
    left: String,
    right_1: String,
    right_2: Option<String>,
    right_3: Option<String>,
) -> Element {
    rsx! {
        Row {
            class: "h-8 sm:h-10 w-full min-w-max px-5 sm:px-3 justify-between font-medium text-xs sm:text-sm text-gray-700",
            span {
                class: "flex w-screen sm:w-full sm:min-w-96 -ml-5 sm:ml-0 px-5 sm:px-0",
                Row {
                    class: "my-auto w-full sm:min-w-96 grow-0 shrink-0 sm:grow justify-between",
                    span {
                        class: "w-min sm:w-64",
                        {left}
                    }
                    span {
                        class: "flex text-right w-40 my-auto justify-end",
                        {right_1}
                    }
                }
            }
            Row {
                class: "text-right",
                if let Some(right_2) = right_2 {
                    span {
                        class: "w-40",
                        {right_2}
                    }
                }
                if let Some(right_3) = right_3 {
                    span {
                        class: "w-40",
                        {right_3}
                    }
                }
            }
        }
    }
}

#[component]
pub fn TableRowLink(
    to: Route,
    left: Element,
    right_1: Element,
    right_2: Option<Element>,
    right_3: Option<Element>,
) -> Element {
    rsx! {
        Link {
            to: to,
            class: "flex flex-row w-full min-w-max px-5 sm:px-3 py-4 sm:rounded-md transition hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
            span {
                class: "w-screen sm:w-full sm:min-w-96 -ml-5 sm:ml-0 px-5 sm:px-0",
                Row {
                    class: "my-auto w-full sm:min-w-96 grow-0 shrink-0 sm:grow justify-between",
                    span {
                        class: "w-min sm:w-64",
                        {left}
                    }
                    span {
                        class: "flex text-right w-40 my-auto justify-end",
                        {right_1}
                    }
                }
            }
            Row {
                class: "text-right",
                if let Some(right_2) = right_2 {
                    span {
                        class: "flex w-40 justify-end",
                        {right_2}
                    }
                }
                if let Some(right_3) = right_3 {
                    span {
                        class: "flex w-40 justify-end",
                        {right_3}
                    }
                }
            }
        }
    }
}
