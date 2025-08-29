use web_sys::{WebGlProgram, WebGlRenderingContext};

pub struct Renderer {
    pub context: WebGlRenderingContext,
    pub program: WebGlProgram,
}

impl Renderer {
    pub fn new(context: WebGlRenderingContext, program: WebGlProgram) -> Self {
        Self { context, program }
    }

    pub fn clear(&self, background_color: [f32; 4]) {
        self.context.clear_color(background_color[0], background_color[1], background_color[2], background_color[3]);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn clear_3d(&self, background_color: [f32; 4]) {
        self.context.clear_color(background_color[0], background_color[1], background_color[2], background_color[3]);
        self.context.enable(WebGlRenderingContext::DEPTH_TEST);
        self.context.depth_func(WebGlRenderingContext::LESS);
        self.context.disable(WebGlRenderingContext::CULL_FACE); // Show both front and back faces
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    }

    // Shape-specific render methods removed - GraphicsEngine now calls shapes directly
}