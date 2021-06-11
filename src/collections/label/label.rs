use either::Either;
use yew::prelude::*;

use crate::{cx, sui};
use crate::helper::*;
use crate::collections::Icon;
use crate::collections::label::LabelDetail;

/// A label displays content classification.
pub struct Label {
    link: ComponentLink<Self>,
    props: LabelProps,
    classes: Vec<String>,
}

pub enum LabelEvent {
    Click(MouseEvent),
    Remove(MouseEvent)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LabelAttachedPosition {
    Top,
    Bottom,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LabelCorner {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LabelPointing {
    Above,
    Below,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LabelRibbon {
    Right
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LabelProps {
    /// An html element type to render as root element.
    #[prop_or_else(|| "a".to_string())]
    pub root: String,
    /// A label can be active.
    #[prop_or(false)]
    pub active: bool,
    /// A label can attach to a content segment.
    #[prop_or_else(|| None)]
    pub attached: Option<LabelAttachedPosition>,
    /// A label can reduce its complexity.
    #[prop_or(false)]
    pub basic: bool,
    /// Primary content.
    #[prop_or_default]
    pub children: Children,
    /// A label can be circular.
    #[prop_or(false)]
    pub circular: bool,
    /// Additional classes.
    #[prop_or_else(|| None)]
    pub class_name: Option<String>,
    /// Color of the label.
    #[prop_or_else(|| None)]
    pub color: Option<sui::Colors>,
    /// Shorthand for primary content.
    #[prop_or_else(|| None)]
    pub content: Option<String>,
    /// A label can position itself in the corner of an element.
    #[prop_or_else(|| Either::Left(false))]
    pub corner: Either<bool, LabelCorner>,
    /// Shorthand for LabelDetail.
    #[prop_or_default]
    pub detail: ChildrenWithProps<LabelDetail>,
    /// Float above another element in the upper right corner.
    #[prop_or(false)]
    pub floating: bool,
    /// A horizontal label is formatted to label content along-side it horizontally.
    #[prop_or(false)]
    pub horizontal: bool,
    /// Shorthand for Icon.
    #[prop_or_default]
    pub icon: ChildrenWithProps<Icon>,

    // TODO: /// A label can be formatted to emphasize an image or prop can be used as shorthand for Image.
    // image: oneOfType([bool, customitemShorthand]),

    /// Called on click.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    /// Adds an "x" icon, called when "x" is clicked.
    #[prop_or_else(|| None)]
    pub on_remove: Option<Callback<MouseEvent>>,
    /// Icon to appear as the last child and trigger on_remove.
    #[prop_or_default]
    pub remove_icon: ChildrenWithProps<Icon>,
    /// A label can point to content next to it.
    #[prop_or_else(|| Either::Left(false))]
    pub pointing: Either<bool, LabelPointing>,
    /// A label can prompt for an error in your forms.
    #[prop_or(false)]
    pub prompt: bool,
    /// A label can appear as a ribbon attaching itself to an element.
    #[prop_or_else(|| Either::Left(false))]
    pub ribbon: Either<bool, LabelRibbon>,
    /// A label can have different sizes.
    #[prop_or_else(|| None)]
    pub size: Option<sui::Sizes>,
    /// A label can appear as a tag.
    #[prop_or(false)]
    pub tag: bool,
}

impl Component for Label {
    type Message = LabelEvent;
    type Properties = LabelProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let classes = props.derive_classes();
        Self { props, link, classes }
    }

    fn update(&mut self, event: Self::Message) -> bool {
        use LabelEvent::*;

        match event {
            Click(e) => self.props.on_click.emit(e.clone()),
            Remove(e) => {
                self.props.on_click.emit(e.clone());
                if let Some(cb) = self.props.on_remove.as_ref() {
                    cb.emit(e);
                }
            }
        }

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
        if !self.props.children.is_empty() {
            return html!{
                <@{ self.props.root.clone() }
                  class=classes!(self.classes.as_slice())
                  onclick=self.link.callback(|e| LabelEvent::Click(e))
                >
                  { self.props.children.clone() }
                </@>
            }
        }

        let remove_icon = if self.props.on_remove.is_some() {
            let cb = self.link.callback(|e| LabelEvent::Remove(e));
            if self.props.remove_icon.is_empty() {
                html! { <Icon name="delete" onclick={cb}/> }
            } else {
                let mut icon = self.props.remove_icon.iter().next().unwrap();
                icon.props.onclick = cb;
                html! { icon }
            }
        } else {
            html! {}
        };

        return html!(
            <@{ self.props.root.clone() }
              class=classes!(self.classes.as_slice())
              onclick=self.link.callback(|e| LabelEvent::Click(e))
            >
              {self.props.icon.clone()}
              // TODO: {typeof image !== 'boolean' && Image.create(image, { autoGenerateKey: false })}
              { match self.props.content {
                    Some(ref content) => html! { content.clone() },
                    None => html! {}
                }
              }
              {self.props.detail.clone()}
              {remove_icon}
            </@>
        )
    }
}

impl LabelProps {
    fn derive_classes(&self) -> Vec<String> {
        let Self {
            active,
            attached,
            basic,
            circular,
            class_name,
            color,
            corner,
            floating,
            horizontal,
            // TODO: image,
            pointing,
            prompt,
            ribbon,
            size,
            tag,
            ..
        } = self;

        let mut pointing_class = match pointing {
            Either::Left(b) => if *b {
                vec!["pointing".to_string()]
            } else {
                vec![]
            }
            Either::Right(p) => match *p {
                LabelPointing::Left | LabelPointing::Right => vec![p.as_ref().to_owned(), "pointing".to_string()],
                LabelPointing::Above | LabelPointing::Below => vec!["pointing".to_string(), p.as_ref().to_owned()],
            }
        };

        cx!(
            use_str("ui"),
            use_option(color),
            pointing_class,
            use_option(size),
            use_key(*active, "active"),
            use_key(*basic, "basic"),
            use_key(*circular, "circular"),
            use_key(*floating, "floating"),
            use_key(*horizontal, "horizontal"),
            // TODO: use_key(image == true, "image",
            use_key(*prompt, "prompt"),
            use_key(*tag, "tag"),
            use_key_or_option_and_key(corner, "corner"),
            use_key_or_option_and_key(ribbon, "ribbon"),
            use_option_and_key(attached, "attached"),
            use_str("label"),
            use_option(class_name)
        )
    }
}

impl From<LabelAttachedPosition> for &'static str {
    fn from(l: LabelAttachedPosition) -> Self {
        use LabelAttachedPosition::*;

        match l {
            Top => "top",
            Bottom => "bottom",
            TopRight => "top right",
            TopLeft => "top left",
            BottomLeft => "bottom left",
            BottomRight => "bottom right",
        }
    }
}

impl AsRef<str> for LabelAttachedPosition {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<LabelCorner> for &'static str {
    fn from(l: LabelCorner) -> Self {
        use LabelCorner::*;

        match l {
            Left => "left",
            Right => "right",
        }
    }
}

impl AsRef<str> for LabelCorner {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<LabelPointing> for &'static str {
    fn from(l: LabelPointing) -> Self {
        use LabelPointing::*;

        match l {
            Left => "left",
            Right => "right",
            Above => "above",
            Below => "below",
        }
    }
}

impl AsRef<str> for LabelPointing {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<LabelRibbon> for &'static str {
    fn from(l: LabelRibbon) -> Self {
        use LabelRibbon::*;

        match l {
            Right => "right",
        }
    }
}

impl AsRef<str> for LabelRibbon {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}
