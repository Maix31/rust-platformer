use ggez::{
    GameResult,
    Context,
    graphics::{
        Rect,
        spritebatch::SpriteBatch,
        Image,
        DrawParam,
    },
};

use std::path::Path;

use tiled;

pub struct TileMap {
    map: tiled::Map,
    tile_rects: Vec<Rect>,
    spritebatch: SpriteBatch,
}

impl TileMap {
    pub fn new(ctx: &mut Context, _path: &Path) -> GameResult<Self> {
        
        let map = tiled::parse_file(&Path::new("C:/Users/darkm/Documents/Rust/rust-platformer/resources/tiled/world_1_1.tmx")).expect("Couldn't parse tilemap");
        
        let cur_tileset = map.get_tileset_by_gid(1).unwrap();
        let cur_tileset_image = &cur_tileset.images[0];
        let width_iter = (cur_tileset.margin..(cur_tileset_image.width as u32 - cur_tileset.margin)).step_by((cur_tileset.tile_width + cur_tileset.spacing) as usize);
        let height_iter = (cur_tileset.margin..(cur_tileset_image.height as u32 - cur_tileset.margin)).step_by((cur_tileset.tile_height + cur_tileset.spacing) as usize);

        // let tileset_path = path
        //     .parent()
        //     .expect("Could not find parent directory of current tile map, are we somehow above the root?")
        //     .join(cur_tileset_image.source.clone());

        let tileset_image = Image::new(ctx, "/tiled/NES - Super Mario Bros - Tileset.png").expect("Could not load image file");

        let mut tile_rects = Vec::new();

        for y in height_iter.clone() {
            for x in width_iter.clone() {
                let tileset_width = tileset_image.width() as f32;
                let tileset_height = tileset_image.height() as f32;
                let tile_width = cur_tileset.tile_width as f32;
                let tile_height = cur_tileset.tile_height as f32;
                let x = x as f32;
                let y = y as f32;
                tile_rects.push(ggez::graphics::Rect::new(x / tileset_width, y / tileset_height, tile_width / tileset_width, tile_height / tileset_height));
            }
        }

        Ok(TileMap { map, tile_rects, spritebatch: SpriteBatch::new(tileset_image.clone()) })
    }

    pub fn draw(&mut self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        self.spritebatch.clear();
        self.spritebatch.set_filter(ggez::graphics::FilterMode::Nearest);
        let bounds = ggez::graphics::screen_coordinates(ctx);
        let scale = 1.0;
        for layer in &self.map.layers {
            for (y, row) in layer.tiles.iter().cloned().enumerate() {
                for (x, tile) in row.iter().cloned().enumerate() {
                    if tile == 0 {
                        continue;
                    }
                
                    let tile = tile - 1;
                    let tile_rect = self.tile_rects[tile as usize];
                    let x = x as f32 * 16. * scale;
                    let y = y as f32 * 16. * scale;
                    if !(bounds.left() - 16. * scale < x && x < bounds.right() + 16. * scale && bounds.top() - 16. * scale < y && y < bounds.bottom() + 16. * scale) {
                        continue;
                    }

                    let p = DrawParam::new().src(tile_rect).dest([x, y]).scale([scale, scale]);
                    self.spritebatch.add(p);
                }
            }
        }
        ggez::graphics::draw(ctx, &self.spritebatch, param)
    }
}