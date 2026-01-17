use ggez::{Context,ContextBuilder};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::event::{self,EventHandler};
use ggez::glam::Vec2;

use rust_sim::node::Node;
use rust_sim::leach;
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

            let node = Node::new(idx,Vec2::new(pos_x, pos_y));
            nodes.push(node);
        }

        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(0.,0.),
            30.,
            0.1,
            Color::WHITE).unwrap();

        Self{
            nodes,
            mesh,
            round: 0
        }
    }
}

impl EventHandler for WSN{

    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {

        if ctx.time.check_update_time(2) {
            self.round += 1;
            leach::reset(&mut self.nodes);
            leach::build(&mut self.nodes, self.round);

            println!("Round {}", self.round);
        }
        Ok(())
    }    


    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(34, 40, 49));

        for node in &self.nodes {
            let color = if node.is_ch && node.is_alive {
                Color::from_rgb(255, 105, 105)

            } else if node.is_alive {
                Color::from_rgb(255, 245, 225)
            } else {
                Color::RED        
            };

            canvas.draw(
                &self.mesh,
                DrawParam::default()
                .dest(node.position)
                .color(color),
            );

        }

        canvas.finish(ctx)?;
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

    let state = WSN::new(&mut ctx,50);

    event::run(ctx, event_loop, state);


}
