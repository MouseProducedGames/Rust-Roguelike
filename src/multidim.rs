use std::default::Default;

pub struct Multidim<T> where T: Copy + Clone + Default
{
    values: Vec<T>,
    len0: usize,
    len1: usize,
}

impl<T> Multidim<T> where T: Copy + Clone + Default
{
    pub fn new(len0: usize, len1: usize) -> Self
    {
        let mut output: Self =
            Self {
                values: Vec::<T>::new(),
                len0: len0,
                len1: len1,
            };
        output.values.resize(len0 * len1, Default::default());
        return output;
    }
    
    pub fn bounds(&self) -> ( usize, usize )
    {
        ( self.len0(), self.len1(), )
    }

    pub fn len0(&self) -> usize { self.len0 }
    pub fn len1(&self) -> usize { self.len1 }

    pub fn value(&self, pos0: usize, pos1: usize) -> &T
    {
        &self.values[ self.get_index( pos0, pos1 ) ]
    }

    pub fn value_mut<'a>(&'a mut self, pos0: usize, pos1: usize) -> &'a mut T
    {
        let index = self.get_index( pos0, pos1 );
        &mut self.values[ index ]
    }

    fn get_index(&self, pos0: usize, pos1: usize) -> usize
    {
        ( pos0 * self.len1 ) + pos1
    }
}

/*
impl<T> Copy for Multidim<T> {}

impl<T> Clone for Multidim<T> where T: Copy + Clone + Default
{
    fn clone(&self) -> Self
    {
        let mut output: Self =
            Self {
                values: Vec::new(),
                len0: self.len0,
                len1: self.len1,
            };
        output.values.resize(self.len0 * self.len1, self.values);
        output;
    }
}

impl<T> Default for Multidim<T> where T: Copy + Clone + Default
{
    fn default() -> Self
    {
        Self {
            values: Vec::new(),
            len0: 0_usize,
            len1: 0_usize,
        }
    }
}
*/
