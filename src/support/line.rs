use druid::kurbo::Line;
use druid::widget::Painter;
use druid::RenderContext;

use crate::theme;

pub fn hline<T>() -> Painter<T> {
    Painter::new(|ctx, _: &T, env| {
        let rect = ctx.size().to_rect();
        let max_y = rect.height() - 0.5;
        let line = Line::new((0.0, max_y), (rect.width(), max_y));

        // ctx.fill(rect, &env.get(theme::BACKGROUND));
        ctx.stroke(line, &env.get(theme::SIDEBAR_EDGE_STROKE), 1.0);
    })
}
