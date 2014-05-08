use {
    BackEnd,
    Borrowed,
    CanTransform,
    Color,
    Draw,
    Field,
    HasTransform,
    Image,
    Matrix2d,
    Rectangle,
    RelativeRectangle,
    Value,
    View,
};
use vecmath::{
    identity,
    margin_rectangle,
    relative_rectangle,
};
use triangulation::{
    rect_tri_list_xy_f32,
    rect_tri_list_rgba_f32,
    rect_tri_list_uv_f32,
};

/// An image rectangle context.
pub struct ImageRectangleColorContext<'a> {
    /// Base/original transformation.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current image.
    pub image: Field<'a, Image>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> HasTransform<'a, Matrix2d> for ImageRectangleColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, ImageRectangleColorContext<'a>, Matrix2d> for ImageRectangleColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> Draw<'a> for ImageRectangleColorContext<'a> {
    fn draw<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let rect = self.rect.get();
            let color = self.color.get();
            let color: [f32, ..4] = [color[0], color[1], color[2], color[3]];
            let texture_id = self.image.get().texture_id;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 || back_end.has_texture_alpha(texture_id);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture_id);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                rect_tri_list_xy_f32(self.transform.get(), rect),
                rect_tri_list_rgba_f32(color),
                rect_tri_list_uv_f32(self.image.get())
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a> RelativeRectangle<'a> for ImageRectangleColorContext<'a> {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            rect: Value(margin_rectangle(self.rect.get(), m)),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            rect: Value(relative_rectangle(self.rect.get(), x, y)),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> View<'a> for ImageRectangleColorContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> ImageRectangleColorContext<'a> {
        ImageRectangleColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Borrowed(self.image.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

