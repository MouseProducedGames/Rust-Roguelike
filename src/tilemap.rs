use super::multidim::Multidim;
use super::tiletype::TILE_TYPE_DATA;

type TileType = u32;
type Width = usize;
type Height = usize;

pub struct Tilemap
{
    tiles: Multidim<TileType>,
}

impl Tilemap
{
    pub fn new(width: Width, height: Height) -> Self
    {
        Self { tiles: Multidim::new( height, width ) }
    }
    
    pub fn bounds(&self) -> ( Width, Height)
    {
        let ( height, width ) = self.tiles.bounds();
        ( width, height )
    }

    pub fn passable(&self, pos_x: Width, pos_y: Height) -> bool
    {
        let tile_type = self.tile( pos_x, pos_y );
        TILE_TYPE_DATA[tile_type as usize].passable()
    }

    pub fn tile(&self, pos_x: Width, pos_y: Height) -> TileType
    {
        *self.tiles.value( pos_y, pos_x )
    }

    pub fn tile_mut<'a>(&'a mut self, pos_x: Width, pos_y: Height) -> &'a mut TileType
    {
        self.tiles.value_mut( pos_y, pos_x )
    }
}
