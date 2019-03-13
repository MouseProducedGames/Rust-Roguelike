// External includes
use std::any::TypeId;
use std::cmp::{ Eq, PartialEq };
use std::hash::{ Hash, Hasher };

// Internal includes
use crate::rrl_math::Position;
use super::super::multidim::Multidim;
use super::mapping::Mapping;
use super::tiletype::{ TILE_TYPE_INDEX_VOID, TILE_TYPE_DATA, TileTypeData };

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

    pub fn height(&self) -> usize
    {
        self.tiles.len0()
    }
    
    pub fn width(&self) -> usize
    {
        self.tiles.len1()
    }
    
    pub fn _passable(&self, pos_x: Width, pos_y: Height) -> bool
    {
        self._tile( pos_x, pos_y ).passable()
    }

    pub fn passable_pos(&self, pos: Position) -> bool
    {
        self.tile_pos( pos ).passable()
    }

    pub fn _tile(&self, pos_x: Width, pos_y: Height) -> TileTypeData
    {
        TILE_TYPE_DATA[ self.tile_type( pos_x, pos_y ) as usize ]
    }

    pub fn tile_pos(&self, pos: Position) -> TileTypeData
    {
        TILE_TYPE_DATA[ self.tile_type_pos( pos ) as usize ]
    }
    
    pub fn tile_type(&self, pos_x: Width, pos_y: Height) -> TileType
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

    pub fn tile_type_mut(&mut self, pos_x: Width, pos_y: Height) -> &mut TileType
    {
        self.tiles.value_mut( pos_y, pos_x )
    }
    
    pub fn tile_type_pos(&self, pos: Position) -> TileType
    {
        if self.is_pos_in_bounds( pos )
        {
            self.tile_type(pos.x as usize, pos.y as usize)
        }
        else
        {
            TILE_TYPE_INDEX_VOID
        }
    }

    pub fn _transparent(&self, pos_x: Width, pos_y: Height) -> bool
    {
        self._tile( pos_x, pos_y ).transparent()
    }

    pub fn transparent_pos(&self, pos: Position) -> bool
    {
        self.tile_pos( pos ).transparent()
    }

}

impl Hash for Tilemap
{
  fn hash<H: Hasher>(&self, state: &mut H)
  {
      self.id.hash(state);
  }
}

impl Mapping for Tilemap
{
    fn height( &self ) -> usize
    {
        Tilemap::height ( self )
    }

    fn width( &self ) -> usize
    {
        Tilemap::width ( self )
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
