use yew::prelude::*;

use crate::cx;
use crate::helper::*;
use super::IconSize;

/// Several icons can be used together as a group.
pub struct IconGroup {
    link: ComponentLink<Self>,
    props: IconGroupProps,
    classes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct IconGroupProps {
    /// An html element type to render as root element.
    #[prop_or_else(|| "i".to_string())]
    pub root: String,
    /// Primary content.
    #[prop_or_default]
    pub children: Children,
    /// Additional classes.
    #[prop_or_else(|| None)]
    pub class_name: Option<String>,
    /// Size of the icon group.
    #[prop_or_else(|| None)]
    pub size: Option<IconSize>,
}

impl Component for IconGroup {
    type Message = ();
    type Properties = IconGroupProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let classes = props.derive_classes();
        Self {
            props,
            link,
            classes
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props != self.props {
            self.props = props;
            self.classes = self.props.derive_classes();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <@{ self.props.root.clone() }
              class=classes!(self.classes.as_slice())
            >
              { self.props.children.clone() }
            </@>
        }
    }
}

impl IconGroupProps {
    fn derive_classes(&self) -> Vec<String> {
        let Self {
            size,
            class_name,
            ..
        } = self;
        cx!(
            use_option(size),
            use_str("icons"),
            use_option(class_name)
        )
    }
}