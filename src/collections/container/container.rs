use crate::cx;
use crate::helper::*;
use crate::sui;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ContainerProps {
    #[prop_or_else(|| "div".to_string())]
    pub root: String,

    /** Primary content. */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes. */
    #[prop_or_else(|| None)]
    pub class_name: Option<String>,

    #[prop_or_else(|| None)]
    pub content: Option<String>,

    /** Container has no maximum width. */
    #[prop_or(false)]
    pub fluid: bool,

    /** Reduce maximum width to more naturally accommodate text. */
    #[prop_or(false)]
    pub text: bool,

    /** Align container text. */
    #[prop_or("justified")]
    pub text_align: &'static str,
}

impl ContainerProps {
    fn derive_classes(&self) -> Vec<String> {
        let Self { 
            class_name,
            text,
            fluid,
            text_align,
             ..
        } = self;

        cx!(
            use_str("ui"),
            use_key(*text, "text"),
            use_key(*fluid, "fluid"),
            use_text_align(*text_align),
            use_str("container"),
            use_option(class_name)
        )
    }
}

pub struct Container {
    props: ContainerProps,
    link: ComponentLink<Self>,
    classes: Vec<String>,
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let classes = props.derive_classes();
        Self {
            link,
            props,
            classes,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.classes = self.props.derive_classes();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if !self.props.children.is_empty() {
            html! {
                <@ { self.props.root.clone() }
                    class=classes!(self.classes.as_slice())
                >
                    { self.props.children.clone() }
                </@>
            }
        } else {
            html! {
                <@ { self.props.root.clone() }
                    class=classes!(self.classes.as_slice())
                >
                    {
                        self.props
                            .content
                            .as_ref()
                            .map(|content| html! { content.clone() })
                            .unwrap_or(html! {})
                    }
                </@>
            }
        }
    }
}
