// External includes
use std::any::TypeId;
use std::cmp::{ Eq, PartialEq };
use std::hash::{ Hash, Hasher };

// Internal includes
use super::linear::Position;
use super::multidim::Multidim;
use super::tiletype::{ TILE_TYPE_INDEX_VOID, TILE_TYPE_DATA };

type TileType = u32;
type Width = usize;
type Height = usize;

pub struct Tilemap
{
    id: TypeId,
    tiles: Multidim<TileType>,
}

impl Tilemap
{
    pub fn new(width: Width, height: Height) -> Self
    {
        Self {
            id: TypeId::of::<Tilemap>(),
            tiles: Multidim::new( height, width )
        }
    }
    
    pub fn bounds(&self) -> ( Width, Height)
    {
        let ( height, width ) = self.tiles.bounds();
        ( width, height )
    }

    pub fn is_in_bounds(&self, pos_x: Width, pos_y: Height) -> bool
    {
        let ( width, height ) = self.bounds();
        ( (pos_x >= width) || ( pos_y >= height ) ) == false
    }
    
    pub fn is_pos_in_bounds(&self, pos: Position) -> bool
    {
        let ( width, height ) = self.bounds();
        (
            (pos.x < 0) || (pos.x as usize >= width) ||
                (pos.y < 0) || (pos.y as usize >= height)
        ) == false
    }

    pub fn passable(&self, pos_x: Width, pos_y: Height) -> bool
    {
        if self.is_in_bounds( pos_x, pos_y )
        {
            TILE_TYPE_DATA[ self.tile( pos_x, pos_y ) as usize ].passable()
        }
        else
        {
            TILE_TYPE_DATA[ TILE_TYPE_INDEX_VOID as usize ].passable()
        }
    }

    pub fn passable_pos(&self, pos: Position) -> bool
    {
        if self.is_pos_in_bounds(pos)
        {
            self.passable( pos.x as usize, pos.y as usize )
        }
        else
        {
            TILE_TYPE_DATA[ TILE_TYPE_INDEX_VOID as usize ].passable()
        }
    }

    pub fn tile(&self, pos_x: Width, pos_y: Height) -> TileType
    {
        if self.is_in_bounds( pos_x, pos_y )
        {
            *self.tiles.value( pos_y, pos_x )
        }
        else
        {
            TILE_TYPE_INDEX_VOID
        }
    }

    pub fn tile_mut<'b>(&'b mut self, pos_x: Width, pos_y: Height) -> &'b mut TileType
    {
            self.tiles.value_mut( pos_y, pos_x )
    }
    
    pub fn tile_pos(&self, pos: Position) -> TileType
    {
        if self.is_pos_in_bounds( pos )
        {
            self.tile(pos.x as usize, pos.y as usize)
        }
        else
        {
            TILE_TYPE_INDEX_VOID
        }
    }

    pub fn transparent(&self, pos_x: Width, pos_y: Height) -> bool
    {
        if self.is_in_bounds( pos_x, pos_y )
        {
            TILE_TYPE_DATA[ self.tile( pos_x, pos_y ) as usize ].transparent()
        }
        else
        {
            TILE_TYPE_DATA[ TILE_TYPE_INDEX_VOID as usize ].transparent()
        }
    }

    pub fn transparent_pos(&self, pos: Position) -> bool
    {
        if self.is_pos_in_bounds(pos)
        {
            self.transparent( pos.x as usize, pos.y as usize )
        }
        else
        {
            TILE_TYPE_DATA[ TILE_TYPE_INDEX_VOID as usize ].transparent()
        }
    }

}

impl Hash for Tilemap
{
  fn hash<H: Hasher>(&self, state: &mut H)
  {
      self.id.hash(state);
  }
}

impl PartialEq for Tilemap
{
  fn eq(&self, other: &Tilemap) -> bool
  {
      self.id == other.id
  }
}

impl Eq for Tilemap {}
