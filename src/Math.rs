use macroquad::prelude::*;
use crate::qtree;
use qtree::Quadrant;

#[derive(Clone, Copy)]
pub struct Vector2{
    pub x: f32,
    pub y: f32,
}

pub trait Edges{
    fn get_left(&self) -> f32;
    fn get_right(&self) -> f32;
    fn get_top(&self) -> f32;
    fn get_bottom(&self) -> f32;
}


#[derive(Clone, Copy)]
pub struct Rect{
    pub pos: Vector2,    
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Edges for Rect{
    fn get_left(&self) -> f32{
        return self.pos.x - self.w / 2.0;
    }
    fn get_right(&self) -> f32{
        return self.pos.x + self.w / 2.0;
    }
    fn get_top(&self) -> f32{
        return self.pos.y - self.h / 2.0;
    }
    fn get_bottom(&self) -> f32{
        return self.pos.y + self.h / 2.0;
    }
}
impl Rect{
    pub fn contatins(&self, v: Vector2) -> bool{
        if self.get_left() <= v.x && v.x <= self.get_right() &&
        self.get_top() <= v.y && v.y <= self.get_bottom(){
            return true;
        }else{
            return false;
        }
    }

    pub fn intersects(&self, r: Rect) -> bool{
        return
            !(self.get_right() < r.get_left() || r.get_right() < self.get_left() ||
            self.get_bottom() < r.get_top() || r.get_top() < self.get_top());

    }
    
    pub fn x_distance_from(&self, v: Vector2) -> f32{
        if self.get_left() <= v.x && v.x <= self.get_right(){
            return 0.0;
        }else{
            let a = v.x - self.get_left();
            let b = v.x - self.get_right();
            return get_min(a.abs(), b.abs());
        }
    }

    pub fn y_distance_from(&self, v: Vector2) -> f32{
        if self.get_top() <= v.y && v.y <= self.get_bottom(){
            return 0.0;
        }else{
            let a = v.y - self.get_top();
            let b = v.y - self.get_bottom();
            return get_min(a.abs(), b.abs());
        }
    }
    pub fn distance_from(&self, v: Vector2) -> f32{
        let a = self.x_distance_from(v);
        let b = self.y_distance_from(v);

        let d = get_pow2(a) + get_pow2(b);
        
        return d.sqrt();
    }

    pub fn Subdivivde_Rect(&self,q: Quadrant)-> Rect{
        match q{
            Quadrant::b1=> return Rect{
                pos:Vector2{
                    x: self.pos.x + self.w / 4.0, 
                    y: self.pos.y - self.h / 4.0,
                }, 
                w: self.w / 2.0, 
                h: self.h / 2.0,
                color: BLUE,
            },
            Quadrant::b2=> return Rect{
                pos:Vector2{
                    x: self.pos.x - self.w / 4.0, 
                    y: self.pos.y - self.h / 4.0,
                }, 
                w: self.w / 2.0, 
                h: self.h / 2.0,
                color: BLUE,
            },
            Quadrant::b3=> return Rect{
                pos:Vector2{
                    x: self.pos.x + self.w / 4.0, 
                    y: self.pos.y + self.h / 4.0,
                }, 
                w: self.w / 2.0, 
                h: self.h / 2.0,
                color: BLUE,
            },
            Quadrant::b4=> return Rect{
                pos:Vector2{
                    x: self.pos.x - self.w / 4.0, 
                    y: self.pos.y + self.h / 4.0,
                }, 
                w: self.w / 2.0, 
                h: self.h / 2.0,
                color: BLUE,
            },
            
        }
    }
}

pub trait CircleMethods{
    fn MoveCircle(&mut self, w: f32, h: f32);
    fn ColourChange(&mut self, change: bool);
    fn CircleIntersect(&self, c: Circle) -> bool;
}
#[derive(Clone, Copy)]
pub struct Circle{
    pub index: i32,
    pub pos: Vector2,
    pub r: f32,
    pub r2: f32,
    pub color: Color,
    pub mov_x: f32,
    pub mov_y: f32,
}
impl CircleMethods for Circle{
    fn MoveCircle(&mut self, w: f32, h: f32){
        if self.pos.x <= 0.0 || self.pos.x > w
        {
            self.mov_x = self.mov_x * -1.0;
            self.pos.x = self.pos.x + self.mov_x;
        }else 
        {
            self.pos.x = self.pos.x + self.mov_x;
        }

        if self.pos.y <= 0.0 || self.pos.y > h{
            self.mov_y = self.mov_y * -1.0;
            self.pos.y = self.pos.y + self.mov_y;
        }else{
            self.pos.y = self.pos.y + self.mov_y;
        }
    }

    fn ColourChange(&mut self, change: bool){
        if change{
            self.color = GREEN;
        }else {
            self.color = RED;
        }
    }

    fn CircleIntersect(&self, c: Circle)-> bool{
        //(R0 - R1)^2 <= (x0 - x1)^2 + (y0 - y1)^2 <= (R0 + R1)^2
        let lower = get_pow2(self.r - c.r);
        let upper = get_pow2(self.r + c.r);
        let d = get_pow2(c.pos.x - self.pos.x) + get_pow2(c.pos.y - self.pos.y);

        if d>= lower && d <=upper{
            return true;
        }else{
            return false;
        }
    }
}
impl Circle{
    fn contatins(&self, v: Vector2) -> bool{               
        let d = get_pow2(v.x - self.pos.x) + get_pow2(v.y - self.pos.y);
        return d <= self.r2;           
    }

    fn intersects(&self, r: Rect) -> bool{
        let mut xDist = r.pos.x - self.pos.x;
        xDist = xDist.abs();

        let mut yDist = r.pos.y - self.pos.y;
        yDist = yDist.abs();

        let rad = self.r;

        let w = r.w / 2.0;
        let h = r.h / 2.0;

        let edge = get_pow2(xDist -w) + get_pow2(yDist - h); 

        if xDist > (rad + w) || yDist > (rad + h){
            return false;
        }else if xDist <= w || yDist <= h{
            return true;
        }else{
            return edge <=self.r2;
        }
    }
    
}
pub fn get_min(a: f32, b: f32) -> f32{
    if a < b{
        return a;
    }else{
        return b;
    }
}

pub fn get_pow2(a: f32) -> f32{
    return a * a;
}