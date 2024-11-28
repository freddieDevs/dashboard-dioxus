use dioxus::prelude::*;
use dioxus_logger::tracing;
use std::rc::Rc;
use std::cell::RefCell;

//struct to represent form state and methods
#[derive(Clone)]
pub struct FormContext {
    pub values: Signal<FormValues>,
    pub set_value: Rc<RefCell<dyn FnMut(&str, String)>>,
    pub handle_submit: Rc<RefCell<dyn Fn()>>,
}

//struct to hold form values
#[derive(Clone, Default, PartialEq, Debug)] 
pub struct FormValues {
    pub username: String,
    pub password: String,
}

impl FormValues {
    pub fn get(&self, field_name: &str) -> String {
        match field_name {
            "username" => self.username.clone(),
            "password" => self.password.clone(),
            _ => String::new(),
        }
    }
}

//form provider props
#[derive(Props, Clone, PartialEq)]
pub struct FormProviderProps {
    children: Element,
}

#[component]
pub fn FormProvider (props: FormProviderProps) -> Element {
    // signal to manage form values
    let mut values = use_signal(|| FormValues::default());

    //fn to update a specific form_field 
    let set_value = 
    Rc::new(RefCell::new(move |field_name: &str, value: String| {
        values.with_mut(|state| match field_name {
            "username" => *&mut state.username = value,
            "password" => *&mut state.password = value,
            _ => {}
        });
    }));

    //fn to handle form submission
    let handle_submit = Rc::new(RefCell::new(move || {
        tracing::info!("Form submited with values: {:?}", *values.read());
    }));

    //create & provide the FormContext
    let form_context = FormContext {
        values: values.clone(),
        set_value: set_value.clone(),
        handle_submit: handle_submit.clone(),
    };
    provide_context(form_context);
    rsx! { {props.children} }
}

//hook to consume the FormContext
pub fn use_form_context() -> FormContext {
    use_context()
}