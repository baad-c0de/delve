use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use knuffel::{parse, Decode};
use tracing::{error, info};

use crate::resources::{Block, Tile};

use super::Database;

/// Encapsulates the possible sections in a KDL file.
///
/// Delve uses KDL files to declare resources. KDL is a simple, human-readable
/// data format that is easy to parse and write.
///
/// The main structure of our KDL files is as follows:
///
/// ```kdl
/// // Lists all the texture atlases that are used in this file.
/// textures {
///     texture ..
///     ..
/// }
///
/// // Lists all the tiles that are used in this file.  They will reference
/// // textures from the textures section and provides names for themselves.
/// tiles {
///     tile ..
///     ..
/// }
///
/// // Lists all the game block types that are used in this file.  They will
/// // reference tiles from the tiles section and provides names for themselves.
/// blocks {
///     block ..
///     ..
/// }
/// ```
#[derive(Decode, Debug)]
pub enum KdlTopLevel {
    Textures(KdlTextures),
    Tiles(KdlTiles),
    Blocks(KdlBlocks),
}

/// Represents a `textures` section in the KDL file.
///
/// This provides a list of texture atlases that are used in this file.
///
/// Each entry in this section is a `texture` entry of the format:
///
/// ```kdl
/// texture "<name>" "<file path>" <width> <height>
/// ```
///
/// The width and height are the dimensions of the texture atlas in tiles, which are 16x16 pixels.
/// Therefore the texture width must be `16*<width>` and its height `16*<height>`.
///
#[derive(Decode, Debug)]
pub struct KdlTextures {
    #[knuffel(children(name = "texture"))]
    textures: Vec<KdlTexture>,
}

/// Represents a `texture` entry in the KDL file within the `textures` section.
#[derive(Decode, Debug)]
pub struct KdlTexture {
    /// The name of the texture atlas.
    #[knuffel(argument)]
    name: String,

    /// The path to the texture atlas image file relative to the KDL file.
    #[knuffel(argument)]
    path: PathBuf,

    /// The width of the texture atlas in tiles.
    #[knuffel(argument)]
    tile_width: usize,

    /// The height of the texture atlas in tiles.
    #[knuffel(argument)]
    tile_height: usize,
}

/// Represents a `tiles` section in the KDL file.
///
/// This provides a list of tiles that are used in this file.
///
/// Each entry in this section is a `tile` entry of the format:
///
/// ```kdl
/// tile "<name>" "<texture name>" <x> <y>
/// ```
///
/// The `texture name` is the name of the texture atlas that this tile is in.
/// The `x` and `y` are the coordinates of the tile in the texture atlas in
/// tiles.
///
#[derive(Decode, Debug)]
pub struct KdlTiles {
    /// The list of tiles that are declared in a `tiles` section.
    #[knuffel(children(name = "tile"))]
    tiles: Vec<KdlTile>,
}

/// Represents a `tile` entry in the KDL file within the `tiles` section.
#[derive(Decode, Debug)]
pub struct KdlTile {
    /// The name of the tile.
    #[knuffel(argument)]
    name: String,

    /// The name of the texture atlas that this tile is in.
    #[knuffel(argument)]
    texture_name: String,

    /// The x coordinate of the tile in the texture atlas in tiles.
    #[knuffel(argument)]
    x: usize,

    /// The y coordinate of the tile in the texture atlas in tiles.
    #[knuffel(argument)]
    y: usize,
}

/// Represents a `blocks` section in the KDL file.
///
/// This provides a list of blocks that are used in this file.
///
/// Each entry in this section is a `block` entry in this format:
///
/// ```kdl
/// block "<name>" <area>="<tile name>"
/// ```
///
/// The `area` can be one of `tiles`, `sides`, `top`, `bottom`, `front`, `back`,
/// `left`, `right`.  The `tile name` is the name of the tile that should be
/// used for that area.
///
/// The `tiles` area is used for blocks that have a single tile for all sides.
/// The `sides` area is used for blocks that have a single tile for all sides
/// (except top and bottom) so the top and bottom areas have to be specified
/// separately.  The `top`, `bottom`, `front`, `back`, `left`, `right` areas are
/// used for blocks that have different tiles for each side.
///
#[derive(Decode, Debug)]
pub struct KdlBlocks {
    /// The list of blocks that are declared in a `blocks` section.
    #[knuffel(children(name = "block"))]
    blocks: Vec<KdlBlock>,
}

/// Represents a `block` entry in the KDL file within the `blocks` section.
/// This can be either a block with a single tile or a block with multiple
/// tiles.
#[derive(Decode, Debug)]
pub struct KdlBlock {
    #[knuffel(argument)]
    name: String,

    #[knuffel(argument)]
    tiles: Option<String>,

    #[knuffel(argument)]
    sides: Option<String>,

    #[knuffel(argument)]
    top: Option<String>,

    #[knuffel(argument)]
    bottom: Option<String>,

    #[knuffel(argument)]
    front: Option<String>,

    #[knuffel(argument)]
    back: Option<String>,

    #[knuffel(argument)]
    left: Option<String>,

    #[knuffel(argument)]
    right: Option<String>,
}

/// Returns the path to the given file name in the `data` directory.
fn get_path<P>(file_name: P) -> PathBuf
where
    P: AsRef<Path>,
{
    PathBuf::from("data").join(file_name)
}

/// Process a KDL file and add its information to the database.
pub fn read_kdl<P>(file_name: P) -> Option<Database>
where
    P: AsRef<Path>,
{
    let path_name = get_path(file_name);
    let text = fs::read_to_string(&path_name).ok()?;
    let path_string = path_name.as_os_str().to_str()?;
    let info = parse::<Vec<KdlTopLevel>>(path_string, &text).ok()?;

    let mut database = Database::new();

    for info in info {
        match info {
            KdlTopLevel::Textures(textures) => process_textures(&mut database, textures),
            KdlTopLevel::Tiles(tiles) => process_tiles(&mut database, tiles),
            KdlTopLevel::Blocks(blocks) => process_blocks(&mut database, blocks),
        }
    }

    Some(database)
}

/// Process the `textures` section of a KDL file.
///
/// This adds the texture atlases to the database.
fn process_textures(db: &mut Database, textures: KdlTextures) {
    for texture in textures.textures {
        let Ok(file) = File::open(get_path(&texture.path)) else {
            error!("Unable to find texture: {:?}", texture.path);
            continue;
        };
        let reader = BufReader::new(file);
        let Ok(image) = image::load(reader, image::ImageFormat::Png) else {
            error!("Unable to load texture: {:?}", texture.path);
            continue;
        };

        let handle = db.images.len();
        info!(
            "Adding texture ({handle}): {:?} ({:?})",
            texture.name, texture.path
        );
        db.images
            .push((image.to_rgba8(), texture.tile_width, texture.tile_height));
        db.image_names.insert(texture.name, handle);
    }
}

/// Process the `tiles` section of a KDL file.
///
/// This adds the tiles to the database.
fn process_tiles(db: &mut Database, tiles: KdlTiles) {
    for tile in tiles.tiles {
        let Some(&image_index) = db.image_names.get(&tile.texture_name) else {
            error!("Invalid image name ({:?}) for tile {:?}", tile.texture_name, tile.name);
            continue;
        };
        let image_data = &db.images[image_index];
        let image_width = image_data.0.width() as f32;
        let image_height = image_data.0.height() as f32;
        let tile_width = image_data.1 as f32;
        let tile_height = image_data.2 as f32;
        let pixel_width = 1.0 / image_width;
        let pixel_height = 1.0 / image_height;
        let tile_x = tile.x as f32;
        let tile_y = tile.y as f32;
        let u0 = (tile_x * tile_width + 0.5) * pixel_width;
        let v0 = (tile_y * tile_height + 0.5) * pixel_height;
        let u1 = ((tile_x + 1.0) * tile_width - 0.5) * pixel_width;
        let v1 = ((tile_y + 1.0) * tile_height - 0.5) * pixel_height;

        let handle = db.tiles.len();
        info!(
            "Adding tile ({handle}): {:?} [({:.3}, {:.3}) - ({:.3}, {:.3})]",
            tile.name, u0, v0, u1, v1
        );
        db.tiles.push(Tile {
            image_index,
            uv0: (u0, v0),
            uv1: (u1, v1),
        });
        db.tile_names.insert(tile.name, handle);
    }
}

/// Process the `blocks` section of a KDL file.
///
/// This adds the blocks to the database.
fn process_blocks(db: &mut Database, blocks: KdlBlocks) {
    fn on_tile<F>(db: &mut Database, possible_tile: Option<String>, f: F)
    where
        F: FnOnce(usize),
    {
        if let Some(name) = possible_tile {
            if let Some(tile) = db.tile_names.get(&name).cloned() {
                f(tile);
            }
        }
    }

    for block in blocks.blocks {
        // Deal with all the sides
        let mut front = None;
        let mut back = None;
        let mut left = None;
        let mut right = None;
        let mut top = None;
        let mut bottom = None;

        // Deal with the `tiles` attribute and set all the sides to the same tile index.
        on_tile(db, block.tiles, |tile| {
            front = Some(tile);
            back = Some(tile);
            left = Some(tile);
            right = Some(tile);
            top = Some(tile);
            bottom = Some(tile);
        });

        // Deal with the `sides` attribute and set the sides to the given tile indices.
        on_tile(db, block.sides, |tile| {
            front = Some(tile);
            back = Some(tile);
            left = Some(tile);
            right = Some(tile);
        });

        // Deal with the individual sides.
        on_tile(db, block.front, |tile| front = Some(tile));
        on_tile(db, block.back, |tile| back = Some(tile));
        on_tile(db, block.left, |tile| left = Some(tile));
        on_tile(db, block.right, |tile| right = Some(tile));
        on_tile(db, block.top, |tile| top = Some(tile));
        on_tile(db, block.bottom, |tile| bottom = Some(tile));

        // Handle any missing sides.
        if front.is_none()
            || back.is_none()
            || left.is_none()
            || right.is_none()
            || top.is_none()
            || bottom.is_none()
        {
            error!("Block type `{}` is missing a side definition", block.name);
            continue;
        }

        let front = front.unwrap();
        let back = back.unwrap();
        let left = left.unwrap();
        let right = right.unwrap();
        let top = top.unwrap();
        let bottom = bottom.unwrap();

        // Add the block to the database.
        let handle = db.blocks.len();
        info!(
            "Adding block ({handle}): {:?} ({:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            block.name, front, back, left, right, top, bottom
        );
        db.blocks.push(Block {
            front,
            back,
            left,
            right,
            top,
            bottom,
        });
        db.block_names.insert(block.name, handle);
    }
}
