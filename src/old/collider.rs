use crate::player::Player;
use crate::tilemap::TileMap;

use ggez::graphics;
use ggez::nalgebra as na;

struct KinematicBody {
    velocity: na::Vector2<f32>,
    rect: graphics::Rect,
    rect_old: graphics::Rect,
    jumping: bool,
    time_since_last_jump: f32,
}

impl KinematicBody {
    pub fn new(x: f32, y: f32,rect: graphics::Rect) -> KinematicBody {
        KinematicBody {
            velocity: na::zero(),
            rect:     rect,
            rect_old: rect,
            jumping:  false,
            time_since_last_jump: 0.0,
        }
    }

    fn top(&self) -> f32 {
        self.rect.y
    }

    fn right(&self) -> f32 {
        self.rect.x + self.rect.w
    }

    fn bottom(&self) -> f32 {
        self.rect.y + self.rect.h
    }

    fn left(&self) -> f32 {
        self.rect.x
    }

    fn set_top(&mut self, top: f32) {
        self.rect.y = top;
    }

    fn set_right(&mut self, right: f32) {
        self.rect.x = right - self.rect.w;
    }

    fn set_bottom(&mut self, bottom: f32) {
        self.rect.y = bottom - self.rect.h;
    }

    fn set_left(&mut self, left: f32) {
        self.rect.x = left;
    }

    fn top_left(&self) -> (f32, f32) {
        (self.left(), self.top())
    }

    fn top_right(&self) -> (f32, f32) {
        (self.right(), self.top())
    }

    fn bottom_right(&self) -> (f32, f32) {
        (self.right(), self.bottom())
    }

    fn bottom_left(&self) -> (f32, f32) {
        (self.left(), self.bottom())
    }

    pub fn move_left(&mut self) {
        self.velocity.x -= crate::PLAYER_VELOCITY.0;
    }

    pub fn move_right(&mut self) {
        self.velocity.x += crate::PLAYER_VELOCITY.0;
    }

    pub fn jump(&mut self) {
        if !self.jumping {
            self.jumping = true;
            self.velocity.y -= crate::PLAYER_VELOCITY.1;
        }
    }

    pub fn update(&mut self, _dt: f32) {

        self.rect_old = self.rect;

        self.rect.x += self.velocity.x;
        self.rect.y += self.velocity.y;
    }
}

pub struct ColliderEdge(bool,bool,bool,bool);

impl ColliderEdge {
    pub fn all() -> ColliderEdge {
        ColliderEdge(true,true,true,true)
    }

    pub fn none() -> ColliderEdge {
        ColliderEdge(false,false,false,false)
    }

    pub fn top(&mut self) -> ColliderEdge {
        ColliderEdge(self.0,self.1,self.2,true)
    }

    pub fn right(&mut self) -> ColliderEdge {
        ColliderEdge(self.0,self.1,true,self.3)
    }

    pub fn bottom(&mut self) -> ColliderEdge {
        ColliderEdge(self.0,true,self.2,self.3)
    }

    pub fn left(&mut self) -> ColliderEdge {
        ColliderEdge(true,self.1,self.2,self.3)
    }
}

impl From<ColliderEdge> for (bool,bool,bool,bool) {
    fn from(a: ColliderEdge) -> Self {
        (a.0,a.1,a.2,a.3)
    }
}

fn resolve(player: &mut Player, block: &ColliderEdge, block_pos: (f32, f32)) {
    match block 
    {
            ColliderEdge(false,false,false,false) => {},                                                       //0
            ColliderEdge(false,false,false, true) => {    collide_top   (player, block_pos.1      )        ;}, //1
            ColliderEdge(false,false, true,false) => {    collide_right (player, block_pos.0 + 1.0)        ;}, //2
            ColliderEdge(false,false, true, true) => { if collide_top   (player, block_pos.1      ) {return;}  //3
                                                          collide_right (player, block_pos.0 + 1.0)        ;}, 
            ColliderEdge(false, true,false,false) => {    collide_bottom(player, block_pos.1 + 1.0)        ;}, //4
            ColliderEdge(false, true,false, true) => { if collide_top   (player, block_pos.1      ) {return;}
                                                          collide_bottom(player, block_pos.1 + 1.0)        ;},
            ColliderEdge(false, true, true,false) => { if collide_right (player, block_pos.0 + 1.0) {return;} //6
                                                          collide_bottom(player, block_pos.1 + 1.0)        ;},
            ColliderEdge(false, true, true, true) => { if collide_top   (player, block_pos.1      ) {return;}  //7
                                                       if collide_right (player, block_pos.0 + 1.0) {return;}
                                                          collide_bottom(player, block_pos.1 + 1.0)        ;},
            ColliderEdge( true,false,false,false) => {    collide_left  (player, block_pos.0      )        ;}, //8
            ColliderEdge( true,false,false, true) => { if collide_top   (player, block_pos.1      ) {return;}  //9
                                                          collide_left  (player, block_pos.0      )        ;}, 
            ColliderEdge( true,false, true,false) => { if collide_left  (player, block_pos.0      ) {return;} //10
                                                          collide_right (player, block_pos.0 + 1.0)        ;}, 
            ColliderEdge( true,false, true, true) => { if collide_top   (player, block_pos.1      ) {return;} //11      
                                                       if collide_left  (player, block_pos.0      ) {return;}
                                                          collide_right (player, block_pos.0 + 1.0)        ;}, 
            ColliderEdge( true, true,false,false) => { if collide_left  (player, block_pos.0      ) {return;} //12
                                                          collide_bottom(player, block_pos.1 + 1.0)        ;},             
            ColliderEdge( true, true,false, true) => { if collide_top   (player, block_pos.1      ) {return;} //13
                                                       if collide_left  (player, block_pos.0      ) {return;}
                                                          collide_bottom(player, block_pos.1 + 1.0)        ;},
            ColliderEdge( true, true, true,false) => { if collide_left  (player, block_pos.0      ) {return;} //14
                                                       if collide_right (player, block_pos.0      ) {return;}
                                                          collide_bottom(player, block_pos.1 + 1.0)        ;},
            ColliderEdge( true, true, true, true) => { if collide_top   (player, block_pos.1      ) {return;} //15
                                                       if collide_left  (player, block_pos.0      ) {return;}
                                                       if collide_right (player, block_pos.0 + 1.0) {return;}
                                                          collide_bottom(player, block_pos.1 + 1.0)        ;},
    }
}

fn collide_bottom(player: &mut Player, bottom: f32) -> bool {
    if player.top() < bottom && player.rect_old.top() >= bottom {
        player.set_top(bottom);
        player.velocity.y = 0.0;
    
        true
    } else {
        false
    }
}

fn collide_left(player: &mut Player, left: f32) -> bool {
    if player.right() > left && player.rect_old.right() <= left{
        player.set_right(left - 0.01);
        player.velocity.x = 0.0;
        
        true
    } else {
        false
    }
}

fn collide_right(player: &mut Player, right: f32) -> bool {
    if player.left() < right && player.rect_old.left() >= right{
        player.set_left(right);
        player.velocity.x = 0.0;

        true
    } else {
        false
    }
}

fn collide_top(player: &mut Player, top: f32) -> bool {
    if player.bottom() > top && player.rect_old.bottom() <= top {
        player.set_bottom(top - 0.01);
        player.velocity.y = 0.0;
        player.jumping = false;

        true
    } else {
        false
    }
}

pub fn collide(player: &mut Player, tilemap: &TileMap) {

    //check for colisions with Game World
    // X-AXIS
    if player.left()  < 0.0                      { player.set_left(0.0);                       player.velocity.x = 0.0;}
    if player.right() > crate::GAME_WIDTH as f32 { player.set_right(crate::GAME_WIDTH as f32); player.velocity.x = 0.0;}

    // Y-AXIS
    if player.top()    < 0.0                       { player.set_top(0.0);                          player.velocity.y = 0.0;}
    if player.bottom() > crate::GAME_HEIGHT as f32 { player.set_bottom(crate::GAME_HEIGHT as f32); player.velocity.y = 0.0; player.jumping = false; }

    let player_corner = player.top_left();
    let block_pos = ((player_corner.0) as usize, (player_corner.1) as usize);
    let block = &tilemap[(block_pos.0 , block_pos.1)].collider;
    resolve(player, block, ((block_pos.0 as f32),(block_pos.1 as f32)));

    let player_corner = player.top_right();
    let block_pos = ((player_corner.0) as usize, (player_corner.1) as usize);
    let block = &tilemap[(block_pos.0 as usize, block_pos.1 as usize)].collider;
    resolve(player, block, ((block_pos.0 as f32),(block_pos.1 as f32)));

    let player_corner = player.bottom_left();
    let block_pos = ((player_corner.0 )as usize, (player_corner.1) as usize);
    let block = &tilemap[(block_pos.0 as usize, block_pos.1 as usize)].collider;
    resolve(player, block, ((block_pos.0 as f32),(block_pos.1 as f32)));

    let player_corner = player.bottom_right();
    let block_pos = ((player_corner.0) as usize, (player_corner.1) as usize);
    let block = &tilemap[(block_pos.0 as usize, block_pos.1 as usize)].collider;
    resolve(player, block, ((block_pos.0 as f32),(block_pos.1 as f32)));
}