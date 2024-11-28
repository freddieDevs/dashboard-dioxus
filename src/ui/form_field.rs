use dioxus::prelude::*;
use crate::ui::form::use_form_context;

#[derive(Props, Clone, PartialEq)]
pub struct FormFieldProps {
  pub field_name: &'static str,
  pub placeholder: &'static str,
  pub input_type: &'static str, 
}

#[component]
pub fn FormField(props: FormFieldProps) -> Element {
  let form = use_form_context();

  rsx! {
    input {
      class: "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 mb-4 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
      r#type: props.input_type,
      placeholder: props.placeholder,
      value: "{form.values.read().get(props.field_name)}",
      oninput: move |ev| {
        if let Ok(mut set_value) = form.set_value.try_borrow_mut() {
          set_value(props.field_name, ev.value().clone());
        }
      }
    }
  }
}