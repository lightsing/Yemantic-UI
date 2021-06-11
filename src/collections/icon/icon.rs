use yew::prelude::*;

use crate::cx;
use crate::sui;
use crate::helper::*;
use either::Either;

/// An icon is a glyph used to represent something else.
pub struct Icon {
    link: ComponentLink<Self>,
    props: IconProps,
    classes: Vec<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Display an icon on top of another icon in corner.
/// Defaults to `BottomRight`.
pub enum IconCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IconRotate {
    Clockwise,
    Counterclockwise
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IconSize {
    Mini,
    Tiny,
    Small,
    Large,
    Big,
    Huge,
    Massive,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct IconProps {
    /// An html element type to render as root element.
    #[prop_or_else(|| "i".to_string())]
    pub root: String,
    /// Formatted to appear bordered.
    #[prop_or(false)]
    pub bordered: bool,
    /// Icon can formatted to appear circular.
    #[prop_or(false)]
    pub circular: bool,
    /// Additional classes.
    #[prop_or_else(|| None)]
    pub class_name: Option<String>,
    /// Color of the icon.
    #[prop_or_else(|| None)]
    pub color: Option<sui::Colors>,
    /// Icons can display a smaller corner icon.
    #[prop_or_else(|| Either::Left(false))]
    pub corner: Either<bool, IconCorner>,
    /// Show that the icon is inactive.
    #[prop_or(false)]
    pub disabled: bool,
    /// Fitted, without space to left or right of Icon.
    #[prop_or(false)]
    pub fitted: bool,
    /// Icon can be flipped.
    #[prop_or_else(|| None)]
    pub flipped: Option<sui::Flip>,
    /// Formatted to have its colors inverted for contrast.
    #[prop_or(false)]
    pub inverted: bool,
    /// Icon can be formatted as a link.
    #[prop_or(false)]
    pub link: bool,
    /// Icon can be used as a simple loader.
    #[prop_or(false)]
    pub loading: bool,
    /// Name of the icon.
    pub name: String,
    /// Icon can rotated.
    #[prop_or_else(|| None)]
    pub rotated: Option<IconRotate>,
    /// Size of the icon.
    #[prop_or_else(|| None)]
    pub size: Option<IconSize>,
    /// Icon can have an aria label.
    #[prop_or_else(|| None)]
    pub aria_hidden: Option<String>,
    /// Icon can have an aria label.
    #[prop_or_else(|| None)]
    pub aria_label: Option<String>,
    /// Children element.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for Icon {
    type Message = MouseEvent;
    type Properties = IconProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let classes = props.derive_classes();
        Self {
            link,
            props,
            classes,
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
              onclick=self.link.callback(|e| e)
              aria-hidden=self.props.get_aria_hidden()
              aria-label=self.props.aria_label.clone()
            >
              { self.props.children.clone() }
            </@>
        }
    }
}

impl IconProps {
    fn derive_classes(&self) -> Vec<String> {
        let Self {
            color,
            name,
            size,
            bordered,
            circular,
            disabled,
            fitted,
            inverted,
            link,
            loading,
            corner,
            flipped,
            rotated,
            class_name,
            ..
        } = self;

        cx!(
            use_option(color),
            use_str(name),
            use_option(size),
            use_key(*bordered, "bordered"),
            use_key(*circular, "circular"),
            use_key(*disabled, "disabled"),
            use_key(*fitted, "fitted"),
            use_key(*inverted, "inverted"),
            use_key(*link, "link"),
            use_key(*loading, "loading"),
            use_key_or_option_and_key(corner, "corner"),
            use_option_and_key(flipped, "flipped"),
            use_option_and_key(rotated, "rotated"),
            use_str("icon"),
            use_option(class_name)
        )
    }

    fn get_aria_hidden(&self) -> Option<String> {
        if self.aria_label.is_none() {
            return Some("true".to_string());
        } else {
            self.aria_hidden.clone()
        }
    }
}

impl From<IconCorner> for &'static str {
    fn from(i: IconCorner) -> Self {
        use IconCorner::*;

        match i {
            TopLeft => "top left",
            TopRight => "top right",
            BottomLeft => "bottom left",
            BottomRight => "bottom right",
        }
    }
}

impl AsRef<str> for IconCorner {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl Default for IconCorner {
    fn default() -> Self {
        Self::BottomRight
    }
}

impl From<IconRotate> for &'static str {
    fn from(i: IconRotate) -> Self {
        use IconRotate::*;

        match i {
            Clockwise => "clockwise",
            Counterclockwise => "Counterclockwise",
        }
    }
}

impl AsRef<str> for IconRotate {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<IconSize> for &'static str {
    fn from(i: IconSize) -> Self {
        use IconSize::*;

        match i {
            Mini => "mini",
            Tiny => "tiny",
            Small => "small",
            Large => "large",
            Big => "big",
            Huge => "huge",
            Massive => "massive",
        }
    }
}

impl AsRef<str> for IconSize {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}