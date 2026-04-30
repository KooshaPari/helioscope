use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use super::Renderable;
use super::RenderableItem;

pub struct FlexChild<'a> {
    flex: i32,
    child: RenderableItem<'a>,
}

pub struct FlexRenderable<'a> {
    children: Vec<FlexChild<'a>>,
}

/// Lays out children in a column, with the ability to specify a flex factor for each child.
///
/// Children with flex factor > 0 will be allocated the remaining space after the non-flex children,
/// proportional to the flex factor.
impl<'a> FlexRenderable<'a> {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn push(&mut self, flex: i32, child: impl Into<RenderableItem<'a>>) {
        self.children.push(FlexChild {
            flex,
            child: child.into(),
        });
    }

    /// Loosely inspired by Flutter's Flex widget.
    ///
    /// Ref https://github.com/flutter/flutter/blob/3fd81edbf1e015221e143c92b2664f4371bdc04a/packages/flutter/lib/src/rendering/flex.dart#L1205-L1209
    fn allocate(&self, area: Rect) -> Vec<Rect> {
        let mut allocated_rects = Vec::with_capacity(self.children.len());
        let mut child_sizes = vec![0; self.children.len()];
        let mut allocated_size = 0;
        let mut total_flex = 0;

        // 1. Allocate space to non-flex children.
        let max_size = area.height;
        let mut last_flex_child_idx = 0;
        for (i, FlexChild { flex, child }) in self.children.iter().enumerate() {
            if *flex > 0 {
                total_flex += flex;
                last_flex_child_idx = i;
            } else {
                child_sizes[i] = child
                    .desired_height(area.width)
                    .min(max_size.saturating_sub(allocated_size));
                allocated_size += child_sizes[i];
            }
        }
        let free_space = max_size.saturating_sub(allocated_size);
        // 2. Allocate space to flex children, proportional to their flex factor.
        let mut allocated_flex_space = 0;
        if total_flex > 0 {
            let space_per_flex = free_space / total_flex as u16;
            for (i, FlexChild { flex, child }) in self.children.iter().enumerate() {
                if *flex > 0 {
                    // Last flex child gets all the remaining space, to prevent a rounding error
                    // from not allocating all the space.
                    let max_child_extent = if i == last_flex_child_idx {
                        free_space - allocated_flex_space
                    } else {
                        space_per_flex * *flex as u16
                    };
                    let child_size = child.desired_height(area.width).min(max_child_extent);
                    child_sizes[i] = child_size;
                    allocated_flex_space += child_size;
                }
            }
        }

        let mut y = area.y;
        for size in child_sizes {
            let child_area = Rect::new(area.x, y, area.width, size);
            allocated_rects.push(child_area);
            y += child_area.height;
        }
        allocated_rects
    }
}

impl<'a> Renderable for FlexRenderable<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.allocate(area)
            .into_iter()
            .zip(self.children.iter())
            .for_each(|(rect, child)| {
                child.child.render(rect, buf);
            });
    }

    fn desired_height(&self, width: u16) -> u16 {
        self.allocate(Rect::new(0, 0, width, u16::MAX))
            .last()
            .map(|rect| rect.bottom())
            .unwrap_or(0)
    }

    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        self.allocate(area)
            .into_iter()
            .zip(self.children.iter())
            .find_map(|(rect, child)| child.child.cursor_pos(rect))
    }
}
