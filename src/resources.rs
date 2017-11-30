use std::env;

use opengl_graphics::glyph_cache::GlyphCache;

/// Additional resources needed for the game
pub struct Resources {
    pub font: GlyphCache<'static>
}

impl Resources {
    /// Initialize and return the `Resources`
    pub fn new() -> Resources {
        let exe_directory = env::current_exe().unwrap().parent().unwrap().to_owned();
        Resources {
            font: GlyphCache::new(&exe_directory.join("resources/FiraMono-Bold.ttf")).unwrap()
        }
    }
}