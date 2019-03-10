type TileType = u32;
type Width = usize;
type Height = usize;

pub struct Tilemap
{
    tiles: Vec<TileType>,
    height: Height,
    width: Width,
}

impl Tilemap
{
    pub fn new(width: Width, height: Height) -> Self
    {
        let mut output: Self = Self { tiles: Vec::new(), width: width, height: height };
        output.tiles.resize(width * height, 0);
        return output;
    }
    
    pub fn get_bounds(&self) -> ( Width, Height)
    {
        return ( self.width, self.height, );
    }

    pub fn get_tile(&self, pos_x: Width, pos_y: Height) -> TileType
    {
        return self.tiles[ self.get_index( pos_x, pos_y ) ];
    }

    pub fn set_tile(&mut self, pos_x: Width, pos_y: Height, tile_type: TileType)
    {
        let index = self.get_index( pos_x, pos_y);
        self.tiles[ index ] = tile_type;
    }

    fn get_index(&self, pos_x: Width, pos_y: Height) -> usize
    {
        return ( pos_y as usize * self.width as usize ) + pos_x as usize;
    }
}
