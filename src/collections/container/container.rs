use crate::{cx, sui};

/// A container limits content to a maximum width.
pub struct Container {
    link: ComponentLink<Self>,
    props: ContainerProps,
    button_ref: NodeRef,
}

pub struct ContainerProps {
    /// Primary content.
    #[prop_or_default]
    pub children: Children,
    /// Additional classes.
    #[prop_or_else(|| None)]
    pub class_name: Option<String>,
    /// Shorthand for primary content.
    #[prop_or_else(|| None)]
    pub content: Option<String>,
    /// Container has no maximum width.
    #[prop_or(false)]
    pub fluid: bool,
    /// Reduce maximum width to more naturally accommodate text.
    #[prop_or(false)]
    pub text: bool,
    /// Align container text.
    #[prop_or_else(|| None)]
    pub text_align: Option<sui::TextAlign>,
}

impl Component for Button {
    type Message;
    type Properties = ContainerProps;

    fn create() {

    }

    fn update() {

    }

    fn change() {

    }

    fn view(&self) -> Html {

    }
}

impl ContainerProps {
    
}