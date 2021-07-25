use macroquad::prelude::*;
use crate::Math;
use Math::Rect;
use Math::Vector2;
use Math::Circle;
use Math::CircleMethods;

pub enum Quadrant{
    b1,b2,b3,b4,
}

#[derive(Clone, Copy)]
pub struct Points{
    pub c: Circle,
}
impl Points{
    pub fn new(c: Circle)->Points{
        Points{
            c,
        }        
    }
}

pub struct QuadTree{
    pub r: Rect,
    pub capacity: usize,
    pub divided: bool,
    pub points: Vec<Points>,
    pub children_option: Option<Vec<QuadTree>>,
}

impl QuadTree{
    pub fn new(r: Rect, capacity: usize)-> QuadTree{
        QuadTree{
            r,
            capacity,
            divided: false,
            points: Vec::new(),
            children_option: None,
        }
    }
    
    pub fn get_child_count(&self) -> usize
    {        
        return self.points.len();
    }

    pub fn get_num_all_qtrees(&self, mut i: i32) -> i32{
        if let None = self.children_option{
            return 0;
        }

        if let Some(children) = &self.children_option{ 
            for q in children.iter(){
                i = i + q.get_num_all_qtrees(i);
            }
            return i;
        }else
        {
            return 0;
        }
    }
    
    pub fn add_point(&mut self, c: Circle)->bool{
        if !self.r.contatins(c.pos){
            return false;
        }
        if self.get_child_count() < self.capacity{
            for p in &self.points{
                if p.c.index == c.index{                    
                    return false;
                }
            }
            self.points.push(Points{
                c,
            });
            return true;
        }       
        if !self.divided{
            self.subdivide();
        }
       
        match self.children_option.as_mut(){
            Some(children) => {
                
                if children[0].add_point(c){
                    return true;
                }else if children[1].add_point(c){
                    return true;
                }else if children[2].add_point(c){
                    return true;
                }else if children[3].add_point(c){
                    return true;
                }
            },
            None => (),
        }

        return false;
    }
   
   pub fn query(&self, c: Circle) ->bool{
        if !self.r.contatins(c.pos){
            return false;
        }

        for p in &self.points{
            if p.c.index == c.index{                    
               continue;
            }else if p.c.CircleIntersect(c)
            {
                return true;
            }
        }

        if let Some(children) = &self.children_option{
            for q in children.iter(){
                if q.query(c){
                    return true;
                }
            }
        }
        return false;
   }

   pub fn draw_rect(&self){
        let x = self.r.pos.x - self.r.w / 2.0;
        let y = self.r.pos.y - self.r.h / 2.0;
        draw_rectangle_lines(x, y, self.r.w-2.0, self.r.h-2.0, 3.0, self.r.color);
        if let Some(children) = &self.children_option{
            for q in children.iter(){
                q.draw_rect();
            }
        }
   }

   fn subdivide(&mut self){
        let q1: QuadTree = QuadTree::new(self.r.Subdivivde_Rect(Quadrant::b1), self.capacity);        
        self.add_new_tree(q1);

        let q2: QuadTree = QuadTree::new(self.r.Subdivivde_Rect(Quadrant::b2), self.capacity);        
        self.add_new_tree(q2);

        let q3: QuadTree = QuadTree::new(self.r.Subdivivde_Rect(Quadrant::b3), self.capacity);        
        self.add_new_tree(q3);

        let q4: QuadTree = QuadTree::new(self.r.Subdivivde_Rect(Quadrant::b4), self.capacity);        
        self.add_new_tree(q4);
        
        self.divided = true;
   }

   fn add_new_tree(&mut self, qt: QuadTree)
    {
        //is Vec is None create new vec
        if let None = self.children_option
        {
            self.children_option = Some(Vec::with_capacity(self.capacity));
        }

        //if vec not None push new quadtree
        if let Some(children) = &mut self.children_option
        {
            children.push(qt);                
        }        
    }
}



