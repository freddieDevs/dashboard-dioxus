use dioxus::prelude::*;
use dioxus_logger::tracing;
use crate::ui::form::{use_form_context, FormProvider};
use crate::ui::form_field::FormField;
use crate::ui::button::{ButtonComponent, ButtonVariant, ButtonSize};

#[component]
pub fn SigninPage() -> Element {

  let form = use_form_context();
  
  rsx! {
    div {
      class: "fixed inset-0 z-50 bg-background/80 backdrop-blur-sm",
      div {
        class: "flex items-center justify-center h-screen",
        div {
          class: "fixed z-50 grid w-full max-w-lg mx-auto border rounded-lg p-4 shadow-lg bg-amber-100 text-cyan-900",
          h1 {
            class: "text-center text-2xl font-bold",
            "Sign In"
          }
          div {
            div {
              class: "space-y-4 py-2 pb-4",
              FormProvider {
                div {
                  FormField {
                    field_name: "username",
                    placeholder: "Username",
                    input_type: "text",
                  }
                  FormField {
                    field_name: "password",
                    placeholder: "Password",
                    input_type: "password",
                  }
                
                }
                div {
                  class: "pt-6 space-x-2 flex items-center justify-end w-full",
                  ButtonComponent{
                    onclick: move |_| {
                      if let Ok(handle_submit) = form.handle_submit.try_borrow() {
                        handle_submit();
                      } else {
                        tracing::info!("failed to access the handle_submit closure");
                      }
                    },
                    variant: ButtonVariant::Default,
                    size: ButtonSize::Default
                  }
                }
              }
            }
          }
          
        }
      }
    }
  }
}

#[component]
fn UsernameField () -> Element {
  let form = use_form_context();
  
  rsx! {
    input {
      r#type: "text",
      placeholder: "Enter username",
      value: "{form.values.read().username}",
      oninput: move |ev| {
        if let Ok(mut set_value) = form.set_value.try_borrow_mut() {
          set_value("username", ev.value().clone());
        }
      }
    }
  }
}

#[component]
fn PasswordField () -> Element {
  let form = use_form_context();
  
  rsx! {
    input {
      r#type: "password",
      placeholder: "Enter password",
      value: "{form.values.read().password}",
      oninput: move |ev| {
        if let Ok(mut set_value) = form.set_value.try_borrow_mut() {
          set_value("password", ev.value().clone());
        }
      }
    }
  }
}

#[component]
fn SubmitButton() -> Element {
  let form = use_form_context();
  
  rsx! {
    button {
      
      onclick: move |_| {
        if let Ok(handle_submit) = form.handle_submit.try_borrow() {
          handle_submit();
        } else {
          tracing::info!("failed to access the handle_submit closure");
        }
      },
      class: "bg-cyan-700 text-accent hover:bg-cyan-800",
      "sign in"
    }
  }
}