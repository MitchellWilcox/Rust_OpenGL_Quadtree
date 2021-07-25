use macroquad::prelude::*;
extern crate rand;
use rand::Rng;

mod qtree;
mod Math;
use Math::CircleMethods;
use Math::Vector2;
use Math::Rect;
use qtree::QuadTree;

fn window_conf() -> Conf {
    Conf {

        window_title: "RustY".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    let mut _circle_list: Vec<Math::Circle> = Vec::new();
    let mut rng = rand::thread_rng();

    let mut toggle = false;

    for _n in 0..1001
    {
        let rad = rng.gen_range(5.0..13.0);
        _circle_list.push( Math::Circle{
            index: _n,
            pos: Math::Vector2{
                x: rng.gen_range(0.0..screen_width()),
                y: rng.gen_range(0.0..screen_height())
            },
            r: rad,
            r2: rad * 2.0,
            color: RED,
            mov_x: rng.gen_range(-2.0..2.0),
            mov_y: rng.gen_range(-2.0..2.0),
        });

    }
    let rect = Rect{           
        pos: Vector2{
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        },
        w: screen_width(),
        h: screen_height(),
        color: BLUE,            
    };

    // let mut index = 0;
    // println!("num rects in qtree {}", qt.get_num_all_qtrees(index));
    loop {
        clear_background(BLACK);

        //input
        if is_key_pressed(KeyCode::F){
            toggle = !toggle;
        }
        //logic
        //move cirlce
        for c in _circle_list.iter_mut()
        {
           c.MoveCircle(screen_width(), screen_height());
        }

        //make quadtree
        let mut qt = QuadTree::new(rect, 4);
        if toggle{
            for _i in 0.._circle_list.len(){
                qt.add_point(_circle_list[_i]);
            }
            for _i in 0.._circle_list.len(){
                _circle_list[_i].ColourChange(false);
                if qt.query(_circle_list[_i]){
                    _circle_list[_i].ColourChange(true);
                }
            }
        }else{
            //do simple collision       
            for _i in 0.._circle_list.len(){
                _circle_list[_i].ColourChange(false);
                for _j in 0.._circle_list.len(){
                    if _i == _j{  
                        continue;                  
                    }else if _circle_list[_i].CircleIntersect(_circle_list[_j]){
                        _circle_list[_i].ColourChange(true);
                        break;
                    }else{
                        continue;
                    }
                }
            }            
        }
       

        //render
        for c in _circle_list.iter()
        {
            draw_circle(c.pos.x, c.pos.y, c.r, c.color);
        }
        qt.draw_rect();

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle_lines(0.0, 0.0, 120.0, 60.0, 3.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_rectangle(0.0, 0.0, 120.0, 30.0, BLACK);
        if toggle{
            draw_text("Quadtree", 10.0, 20.0, 30.0, DARKGRAY);
        }else{
            draw_text("2D loop", 10.0, 20.0, 30.0, DARKGRAY);
        }

        next_frame().await
    }
}



