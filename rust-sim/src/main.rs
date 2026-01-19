use ggez::{
    conf,
    event::{self, EventHandler},
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh},
    Context, ContextBuilder, GameError,
};
use ggez::glam::Vec2;

use rand::Rng;

use rust_sim::config::{
    AREA_HEIGHT, AREA_WIDTH, CYCLE_LENGTH, NUM_NODES, SCREEN_HEIGHT, SCREEN_WIDTH, TO_PIXEL_SCALE,
};
use rust_sim::leach;
use rust_sim::node::Node;

/// Main game state for the Wireless Sensor Network visualization using ggez.
pub struct WSN {
    /// All sensor nodes in the network
    nodes: Vec<Node>,

    /// Reusable mesh for drawing nodes (simple circle)
    node_mesh: Mesh,

    /// Current simulation round
    round: u64,

    /// Number of currently alive nodes (energy > 0)
    alive_nodes: usize,
}

impl WSN {
    /// Creates a new WSN simulation state.
    pub fn new(ctx: &mut Context) -> Result<Self, GameError> {
        let mut rng = rand::rng();

        // 1. Generate random positions and create nodes
        let mut nodes: Vec<Node> = (0..NUM_NODES)
            .map(|id| {
                let x = rng.random_range(0..AREA_WIDTH);
                let y = rng.random_range(0..AREA_HEIGHT);
                Node::new(id, Vec2::new(x, y))
            })
            .collect();

        // 2. (Optional) Compute simple one-hop neighbors
        // Note: This can be quite expensive for large NUM_NODES → consider spatial partitioning
        for i in 0..nodes.len() {
            let pos_i = nodes[i].position;
            let range_i = nodes[i].transmission_range;

            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }
                let dist = (pos_i - nodes[j].position).length();
                if dist <= range_i {
                    nodes[i].neighbours.push(j);
                }
            }
        }

        // 3. Create reusable circle mesh for all nodes
        let node_mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::ZERO,
            6.0,          
            0.1,
            Color::WHITE,
        )?;

        Ok(Self {
            nodes,
            node_mesh,
            round: 0,
            alive_nodes: NUM_NODES,
        })
    }
}

impl EventHandler for WSN {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // Run simulation logic at ~60 FPS (controlled by check_update_time)
        while ctx.time.check_update_time(60) {
            // Early exit when all nodes are dead
            if self.alive_nodes == 0 {
                ctx.request_quit();
                return Ok(());
            }

            // Reset eligibility every cycle (typical LEACH behavior)
            if self.round % CYCLE_LENGTH as u64 == 0 {
                for node in &mut self.nodes {
                    node.eligible = true;
                }
                println!("Round {:4} | Alive: {:3}", self.round, self.alive_nodes);
            }

            self.round += 1;

            // Run LEACH protocol phases
            leach::reset(&mut self.nodes);
            leach::build(&mut self.nodes, self.round as i64, &mut self.alive_nodes);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(34, 40, 49)); // dark background

        for node in &self.nodes {
            let color = if !node.is_alive {
                Color::from_rgb(180, 60, 60)    // dead - dark red
            } else if node.is_cluster_head {
                Color::from_rgb(89, 172, 119)   // cluster head - green
            } else {
                Color::from_rgb(245, 235, 200)  // normal alive node - light yellow
            };

            canvas.draw(
                &self.node_mesh,
                DrawParam::default()
                    .dest(node.position * TO_PIXEL_SCALE)
                    .color(color),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> Result<(), GameError> {
    let (mut ctx, event_loop) = ContextBuilder::new("w_sn_simulation", "madhav")
        .window_setup(
            conf::WindowSetup::default()
                .title("Wireless Sensor Network - LEACH Visualization")
                .vsync(true),
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
                .resizable(false),
        )
        .build()?;

    let state = WSN::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
