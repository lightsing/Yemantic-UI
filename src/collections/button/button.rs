use either::Either;
use yew::prelude::*;

use crate::{cx, sui};
use crate::helper::*;
use crate::collections::{Icon, Label};

/// A Button indicates a possible user action.
/// Also see [`Form`], [`Icon`](crate::collections::Icon), [`Label`]
pub struct Button {
    link: ComponentLink<Self>,
    props: ButtonProps,
    button_ref: NodeRef,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ButtonAnimationType {
    Fade,
    Vertical
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ButtonAttachedPosition {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ButtonLabelPosition {
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ButtonProps {
    /// An html element type to render as root element.
    #[prop_or_else(|| "button".to_string())]
    pub root: String,
    /// A button can show it is currently the active user selection.
    #[prop_or(false)]
    pub active: bool,
    /// A button can animate to show hidden content.
    #[prop_or_else(|| Either::Left(false))]
    pub animated: Either<bool, ButtonAnimationType>,
    /// A button can be attached to other content.
    #[prop_or_else(|| Either::Left(false))]
    pub attached: Either<bool, ButtonAttachedPosition>,
    /// A basic button is less pronounced.
    #[prop_or(false)]
    pub basic: bool,
    /// Primary content.
    #[prop_or_default]
    pub children: Children,
    /// A button can be circular.
    #[prop_or(false)]
    pub circular: bool,
    /// Additional classes.
    #[prop_or_else(|| None)]
    pub class_name: Option<String>,
    /// A button can have different colors
    #[prop_or_else(|| None)]
    pub color: Option<sui::Colors>,
    /// A button can reduce its padding to fit into tighter spaces.
    #[prop_or(false)]
    pub compact: bool,
    /// Shorthand for primary content.
    #[prop_or_else(|| None)]
    pub content: Option<String>,
    /// A button can show it is currently unable to be interacted with.
    #[prop_or(false)]
    pub disabled: bool,
    /// A button can be aligned to the left or right of its container.
    #[prop_or_else(|| None)]
    pub floated: Option<sui::Float>,
    /// A button can take the width of its container.
    #[prop_or(false)]
    pub fluid: bool,
    /// Add an Icon by pass an <Icon />.
    #[prop_or_default]
    pub icon: ChildrenWithProps<Icon>,
    /// A button can be formatted to appear on dark backgrounds.
    #[prop_or(false)]
    pub inverted: bool,
    /// Add a Label by pass a <Label />.
    #[prop_or_default]
    pub label: ChildrenWithProps<Label>,
    /// A labeled button can format a Label or Icon to appear on the left or right.
    #[prop_or_default]
    pub label_position: ButtonLabelPosition,
    /// A button can show a loading indicator.
    #[prop_or(false)]
    pub loading: bool,
    /// A button can hint towards a negative consequence.
    #[prop_or(false)]
    pub negative: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// A button can hint towards a positive consequence.
    #[prop_or(false)]
    pub positive: bool,
    /// A button can be formatted to show different levels of emphasis.
    #[prop_or(false)]
    pub primary: bool,
    /// The role of the HTML element.
    #[prop_or_else(|| None)]
    pub role: Option<String>,
    /// A button can be formatted to show different levels of emphasis.
    #[prop_or(false)]
    pub secondary: bool,
    /// A button can have different sizes.
    #[prop_or_else(|| None)]
    pub size: Option<sui::Sizes>,
    /// A button can receive focus.
    #[prop_or_else(|| None)]
    pub tab_index: Option<isize>,
    /// A button can be formatted to toggle on and off.
    #[prop_or(false)]
    pub toggle: bool,
}

impl Component for Button {
    type Message = MouseEvent;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            button_ref: NodeRef::default()
        }
    }

    fn update(&mut self, event: Self::Message) -> ShouldRender {
        if self.props.disabled {
            event.prevent_default();
        } else {
            self.props.onclick.emit(event);
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }


    fn view(&self) -> Html {
        let root_element = self.props.get_element_type();
        let aria_pressed = if self.props.toggle {
            Some(format!("{}", self.props.active))
        } else {
            None
        };
        let tab_index = format!("{}", self.props.get_tab_index());
        if !self.props.label.is_empty() {
            let button_classes = cx!(
                use_str("ui"),
                self.props.derive_base_classes(),
                use_str("button"),
                use_option(&self.props.class_name)
            );
            let container_classes = cx!(
                use_str("ui"),
                self.props.derive_labeled_class(),
                use_str("button"),
                use_option(&self.props.class_name),
                self.props.derive_wrapper_classes()
            );


            html! {
                <@{ root_element }
                  class=classes!(container_classes.as_slice())
                  onclick=self.link.callback(|e| e)
                >
                  {
                      if self.props.label_position == ButtonLabelPosition::Left && !self.props.label.is_empty() {
                          html! { <>{self.props.label.clone()}</> }
                      } else {
                          html! {}
                      }
                  }
                  <button
                    ref=self.button_ref.clone()
                    class=classes!(button_classes.as_slice())
                    aria-pressed=aria_pressed
                    disabled=self.props.disabled
                    tabindex=tab_index
                  >
                    {self.props.icon.clone()}
                    {
                        match self.props.content {
                            Some(ref content) => html! { content.clone() },
                            None => html! {}
                        }
                    }
                  </button>
                  {
                      if self.props.label_position == ButtonLabelPosition::Right && !self.props.label.is_empty() {
                          html! { <>{self.props.label.clone()}</> }
                      } else {
                          html! {}
                      }
                  }
                </@>
            }
        } else {
           let classes = cx!(
                use_str("ui"),
                self.props.derive_base_classes(),
                self.props.derive_wrapper_classes(),
                self.props.derive_labeled_class(),
                use_str("button"),
                use_option(&self.props.class_name)
            );


            html!{
                <@{ root_element }
                  class=classes!(classes.as_slice())
                  aria-pressed={ if self.props.toggle { Some(format!("{}", self.props.active)) } else { None } }
                  disabled=self.props.disabled
                  onclick=self.link.callback(|e| e)
                  role=self.props.get_aria_role()
                  tabindex=tab_index
                >
                  {
                      if !self.props.children.is_empty() {
                          html! { <>{self.props.children.clone()}</> }
                      } else {
                          html! {
                              <>
                                { self.props.icon.clone() }
                                {
                                    match self.props.content {
                                        Some(ref content) => html! { content.clone() },
                                        None => html! {}
                                    }
                                }
                              </>
                          }
                      }
                  }
                </@>
            }
        }

    }
}

impl ButtonProps {
    fn derive_base_classes(&self) -> Vec<String> {
        let Self {
            active,
            animated,
            attached,
            basic,
            circular,
            color,
            compact,
            fluid,
            inverted,
            loading,
            negative,
            positive,
            primary,
            secondary,
            size,
            toggle,
            ..
        } = self;

        cx!(
            use_option(color),
            use_option(size),
            use_key(*active, "active"),
            use_key(*basic, "basic"),
            use_key(*circular, "circular"),
            use_key(*compact, "compact"),
            use_key(*fluid, "fluid"),
            use_key(self.has_icon_class(), "icon"),
            use_key(*inverted, "inverted"),
            use_key(*loading, "loading"),
            use_key(*negative, "negative"),
            use_key(*positive, "positive"),
            use_key(*primary, "primary"),
            use_key(*secondary, "secondary"),
            use_key(*toggle, "toggle"),
            use_key_or_option_and_key(animated, "animated"),
            use_key_or_option_and_key(attached, "attached")
        )
    }

    fn derive_labeled_class(&self) -> Vec<String> {
        let label = if !self.label.is_empty() {
            Some(self.label_position)
        } else {
            None
        };
        use_option_and_key(&label, "labeled")
    }

    fn derive_wrapper_classes(&self) -> Vec<String> {
        cx!(
            use_key(self.disabled, "disabled"),
            use_option_and_key(&self.floated, "floated")
        )
    }

    fn get_element_type(&self) -> String {
        if self.root != "button" {
            return self.root.clone()
        }
        let Self {
            attached,
            label,
            ..
        } = self;
        if !attached.unset() || !label.is_empty() {
            return "div".to_string()
        }
        return self.root.clone()
    }

    fn get_aria_role(&self) -> Option<String> {
        match self.role {
            None => if self.get_element_type() != "button" {
                Some("button".to_string())
            } else {
                None
            }
            Some(ref role) => Some(role.clone()),
        }
    }

    fn get_tab_index(&self) -> isize {
        let Self { disabled, tab_index, .. } = self;
        match tab_index {
            Some(i) => *i,
            None => if *disabled {
                -1
            } else if self.get_element_type() == "div" {
                0
            } else {
                0
            }
        }
    }

    fn has_icon_class(&self) -> bool {
        if !self.icon.is_empty() {
            true
        } else {
            //FIXME: self.children
            false
        }
    }
}

impl From<ButtonAnimationType> for &'static str {
    fn from(b: ButtonAnimationType) -> Self {
        use ButtonAnimationType::*;

        match b {
            Fade => "fade",
            Vertical => "vertical",
        }
    }
}

impl AsRef<str> for ButtonAnimationType {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<ButtonAttachedPosition> for &'static str {
    fn from(b: ButtonAttachedPosition) -> Self {
        use ButtonAttachedPosition::*;

        match b {
            Left => "left",
            Right => "right",
            Top => "top",
            Bottom => "bottom",
        }
    }
}

impl AsRef<str> for ButtonAttachedPosition {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<ButtonLabelPosition> for &'static str {
    fn from(b: ButtonLabelPosition) -> Self {
        use ButtonLabelPosition::*;

        match b {
            Left => "left",
            Right => "right",
        }
    }
}

impl AsRef<str> for ButtonLabelPosition {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl Default for ButtonLabelPosition {
    fn default() -> Self {
        Self::Left
    }
}