use ggez::{Context,ContextBuilder,GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::event::{self,EventHandler};
use ggez::glam::Vec2;

use rust_sim::node::Node;
use rust_sim::leach;
use rust_sim::config::INITIAL_ENERGY;
use rand::Rng;

const SCREEN_W: f32 = 1200.0;
const SCREEN_H: f32 = 720.0;

struct WSN{
    nodes: Vec<Node>,
    mesh: Mesh,
    round: i64,
}

impl WSN{
    fn new(ctx: &mut Context ,n_nodes: usize) -> Self{
        
        let mut rng = rand::rng();
        let mut nodes: Vec<Node> = Vec::new();

        for idx in 0..n_nodes{
            let pos_x = rng.random::<f32>() * SCREEN_W;
            let pos_y = rng.random::<f32>() * SCREEN_H;

            let node = Node::new(idx,Vec2::new(pos_x, pos_y), INITIAL_ENERGY);
            nodes.push(node);
        }

        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(0.,0.),
            30.,
            0.1,
            Color::BLUE).unwrap();

        Self{
            nodes,
            mesh,
            round: 0
        }
    }
}

impl EventHandler for WSN{

    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {

        self.round += 1;
        leach::build(&mut self.nodes, self.round);
        println!("{:?}",self.nodes);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {

        let mut canvas = graphics::Canvas::from_frame(ctx,Color::BLACK);

        for node in self.nodes.iter(){
            if node.is_ch && node.is_alive {
                canvas.draw(&self.mesh,DrawParam::default()
                    .color(Color::GREEN)
                    .dest(node.position));
            }

            if node.is_alive{
                canvas.draw(&self.mesh, node.position);
            }

        }

        Ok(())
    }
}


fn main(){

    let (mut ctx , event_loop) = ContextBuilder::new("WSN","madhav")
        .window_mode(
            ggez::conf::WindowMode::default()
            .dimensions(SCREEN_W, SCREEN_H))
        .window_setup(
            ggez::conf::WindowSetup::default()
            .title("WSN")
        )
        .build().unwrap();

    let state = WSN::new(&mut ctx,10);

    event::run(ctx, event_loop, state);


}
