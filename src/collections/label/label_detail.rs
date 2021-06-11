use yew::prelude::*;

use crate::cx;
use crate::helper::*;

pub struct LabelDetail {
    props: LabelDetailProps,
    classes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LabelDetailProps {
    /// An html element type to render as root element.
    #[prop_or_else(|| "div".to_string())]
    pub root: String,
    /// Primary content.
    #[prop_or_default]
    pub children: Children,
    /// Additional classes.
    #[prop_or_else(|| None)]
    pub class_name: Option<String>,
}

impl Component for LabelDetail {
    type Message = ();
    type Properties = LabelDetailProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let classes = props.derive_classes();
        Self { props, classes }
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
        html!{
            <@{ self.props.root.clone() }
              class=classes!(self.classes.as_slice())
            >
              { self.props.children.clone() }
            </@>
        }
    }
}

impl LabelDetailProps {
    fn derive_classes(&self) -> Vec<String> {
        cx!(
            use_str("detail"),
            use_option(&self.class_name)
        )
    }
}