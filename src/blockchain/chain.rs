use super::{block::Block};

pub struct Chain{ 
   vec:  Vec< Block>,
}
impl Chain {
   pub fn new ()-> Self{
       return Self{
        vec:  Vec::<Block>::new(),
       };
    }
   pub fn add(&mut self,block:   Block){
     return self.vec.push(  block);
    }

    pub fn insert(&mut self,index:usize,block:  Block){
      return   self.vec.insert(index,  block);
    }
    pub fn len(&self)->usize{
        return self.vec.len();
    }
    pub fn first(&mut self )->Option<& Block>{
        return self.vec.first();
    }
    pub fn last(&self)->Option<&Block>{
        return self.vec.last();
    }
    pub fn  get(&mut self,index:usize)->Option<&Block>{
        return self.vec.get(index);
    }
    pub fn iterate(&self)->  &Vec<Block>{
        return &self.vec;
    }
}