use std::collections::HashMap;

use image::RgbaImage;

/// The database of all the resources used in the game.
pub struct Database {
    /// All the images used in the game and associated tile size.
    pub(crate) images: Vec<(RgbaImage, usize, usize)>,

    /// A map of image names to their index in the `images` vector.
    pub(crate) image_names: HashMap<String, usize>,

    /// All the tiles used in the game.
    pub(crate) tiles: Vec<Tile>,

    /// A map of tile names to their index in the `tiles` vector.
    pub(crate) tile_names: HashMap<String, usize>,

    /// All the block types used in the game.
    pub(crate) blocks: Vec<Block>,

    /// A map of block names to their index in the `blocks` vector.
    pub(crate) block_names: HashMap<String, usize>,
}

/// A tile is a rectangular region of an image.
///
/// The tile is specified by the image index and the UV coordinates of the tile
/// in the image. The image index is the index of the image in the `images`
/// vector of the `Database` which contains all the texture atlases.
pub struct Tile {
    /// Image index where tile is found.
    pub(crate) image_index: usize,

    /// UV coordinates of the tile in the image.
    pub(crate) uv0: (f32, f32),
    pub(crate) uv1: (f32, f32),
}

/// A block is a cube that is rendered in the game.
///
/// The block is specified by the tile indices of the front, back, top, bottom,
/// left and right sides of the block. The tile indices are the indices of the
/// tiles in the `tiles` vector of the `Database` which contains all the tiles
/// used in the game.
pub struct Block {
    /// The index of the tile that is used to render the front of the block.
    pub(crate) front: usize,

    /// The index of the tile that is used to render the back of the block.
    pub(crate) back: usize,

    /// The index of the tile that is used to render the top of the block.
    pub(crate) top: usize,

    /// The index of the tile that is used to render the bottom of the block.
    pub(crate) bottom: usize,

    /// The index of the tile that is used to render the left side of the block.
    pub(crate) left: usize,

    /// The index of the tile that is used to render the right side of the block.
    pub(crate) right: usize,
}

impl Database {
    pub(crate) fn new() -> Database {
        Database {
            images: Vec::new(),
            image_names: HashMap::new(),
            tiles: Vec::new(),
            tile_names: HashMap::new(),
            blocks: Vec::new(),
            block_names: HashMap::new(),
        }
    }
}
