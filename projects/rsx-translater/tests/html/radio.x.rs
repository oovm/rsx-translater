pub fn Component(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "p-6 card bordered",
            div {
                class: "form-control",
                label {
                    class: "cursor-pointer label",
                    span {
                        class: "label-text",
                        "Neutral"
                    }
                    input {
                        class: "radio",
                        checked: "checked",
                        name: "opt",
                        r#type: "radio",
                        value: "",
                    }
                }
            }
            div {
                class: "form-control",
                label {
                    class: "cursor-pointer label",
                    span {
                        class: "label-text",
                        "Primary"
                    }
                    input {
                        class: "radio radio-primary",
                        checked: "checked",
                        name: "opt",
                        r#type: "radio",
                        value: "",
                    }
                }
            }
            div {
                class: "form-control",
                label {
                    class: "cursor-pointer label",
                    span {
                        class: "label-text",
                        "Secondary"
                    }
                    input {
                        class: "radio radio-secondary",
                        checked: "checked",
                        name: "opt",
                        r#type: "radio",
                        value: "",
                    }
                }
            }
            div {
                class: "form-control",
                label {
                    class: "cursor-pointer label",
                    span {
                        class: "label-text",
                        "Accent"
                    }
                    input {
                        class: "radio radio-accent",
                        checked: "checked",
                        name: "opt",
                        r#type: "radio",
                        value: "",
                    }
                }
            }
            div {
                class: "form-control",
                label {
                    class: "label",
                    span {
                        class: "label-text",
                        "Disabled"
                    }
                    input {
                        class: "radio",
                        checked: "checked",
                        disabled: "disabled",
                        name: "opt",
                        r#type: "radio",
                        value: "",
                    }
                }
            }
        }
        
    })
}