use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
  Default,
  Destructive,
  Outline,
  Secondary,
  Ghost,
  Link,
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
  Default,
  Sm,
  Lg,
  Icon,
}

//props for the button component
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
  #[props(default = ButtonVariant::Default)]
  variant: ButtonVariant,
  #[props(default = ButtonSize::Default)]
  size: ButtonSize,
  #[props(default = false)]
  disabled: bool,
  #[props(optional)]
  class: Option<&'static str>,
  #[props(optional)]
  onclick: Option<EventHandler<MouseEvent>>,
  children: Element,
}

#[component]
pub fn ButtonComponent(props: ButtonProps) -> Element {
  let base_class: &str = "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

  //variant styles
  let variant_classes = match props.variant {
    ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
    ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
    ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
    ButtonVariant::Secondary =>  "bg-secondary text-secondary-foreground hover:bg-secondary/80",
    ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
    ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
  };

  //variant sizes
  let size_classes = match props.size {
    ButtonSize::Default => "h-10 px-4 py-2",
    ButtonSize::Sm => "h-9 rounded-md px-3",
    ButtonSize::Lg => "h-11 rounded-md px-8",
    ButtonSize::Icon => "h-10 w-10",
  };

  //combine all classes
  let classes = format!(
    "{} {} {} {}",
    base_class, 
    variant_classes,
    size_classes,
    props.class.unwrap_or("")
  );

  rsx! {
    button {
      class: "{classes}",
      disabled: "{props.disabled}",
      onclick: move |evt| {
        if let Some(onclick) = props.onclick {
          onclick.call(evt);
        }
      },
      
    }
  }
    
}
