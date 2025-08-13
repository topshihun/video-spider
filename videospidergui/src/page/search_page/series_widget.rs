use iced::Rectangle;
use iced::advanced::Layout;
use iced::advanced::Widget;
use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer;
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::mouse::Cursor;
use iced::widget::image;
use iced::{Element, Length, Size};
use std::path::PathBuf;

pub struct SeriesWidget {
    img: image::Handle,
    text: String,
}

impl SeriesWidget {
    pub fn new(img_path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        Self {
            img: image::Handle::from_path(img_path),
            text: text.into(),
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for SeriesWidget
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, _limits: &Limits) -> Node {
        // img and text minimal size
        let image_size = iced::Size::new(64.0, 64.0);
        Node::new(image_size)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        // TODO: draw image and text
    }
}

impl<'a, Message, Theme, Renderer> From<SeriesWidget> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn from(value: SeriesWidget) -> Self {
        Self::new(value)
    }
}
