use minifb::{Key, Scale, Window, WindowOptions};
use rand::Rng;

struct Automaton {
    rule: u8,
    states: Vec<Vec<bool>>,
    age: usize,
}

impl Automaton {
    fn new(rule: u8, width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut states = vec![vec![false; width]; height];

        states[height - 1].fill_with(|| rng.gen());

        Self {
            rule,
            states,
            age: 0,
        }
    }

    fn eval(&self, left: bool, center: bool, right: bool) -> bool {
        let i = (usize::from(left) << 2) + (usize::from(center) << 1) + usize::from(right);
        self.rule & (1 << i) != 0
    }

    fn step(&mut self) {
        let last = &self.states[self.states.len() - 1];
        let mut next = vec![false; last.len()];

        // Update interior cells
        let n = last.len();
        for i in 1..(n - 1) {
            next[i] = self.eval(last[i - 1], last[i], last[i + 1]);
        }

        // Compute states for the edge cells
        next[0] = self.eval(false, last[0], last[1]);
        next[n - 1] = self.eval(last[n - 2], last[n - 1], false);

        self.states.push(next);
        self.states.remove(0);
        self.age += 1;
    }

    // Check to see if the most recent state is uniform
    fn uniform(&self) -> bool {
        let last = &self.states[self.states.len() - 1];
        last.iter().all(|x| *x) || last.iter().all(|x| !*x)
    }

    // Check to see if there is periodicity in recent states
    fn periodic(&self) -> bool {
        const MAX_PERIOD: usize = 10;

        let n = self.states.len() - 1;
        (1..MAX_PERIOD).any(|p| (0..p).all(|i| self.states[n - i] == self.states[n - i - p]))
    }

    // How many generations has this population been alive?
    fn age(&self) -> usize {
        self.age
    }

    fn render(&self) -> Vec<u32> {
        const BLACK: u32 = 0x00000000;
        const WHITE: u32 = 0x00FFFFFF;

        self.states
            .iter()
            .map(|r| r.iter().map(|&b| if b { WHITE } else { BLACK }))
            .flatten()
            .collect()
    }
}

fn rand_automaton(reason: &str, width: usize, height: usize) -> Automaton {
    println!("{}", reason);

    let rule: u8 = rand::thread_rng().gen();
    println!("rule: {}", rule);

    Automaton::new(rule, width, height)
}

fn main() {
    const WIDTH: usize = 240;
    const HEIGHT: usize = 320;
    const MAX_AGE: usize = 5 * HEIGHT;

    let mut window = Window::new(
        "Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            //scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    window.set_target_fps(60);
    window.set_background_color(0, 0, 0);

    let mut buffer = rand_automaton("initial", WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.step();

        if buffer.uniform() {
            buffer = rand_automaton("reached uniform state", WIDTH, HEIGHT)
        } else if buffer.periodic() {
            buffer = rand_automaton("reached periodic state", WIDTH, HEIGHT)
        } else if buffer.age() > MAX_AGE {
            buffer = rand_automaton("reached max age", WIDTH, HEIGHT)
        }

        window
            .update_with_buffer(&buffer.render(), WIDTH, HEIGHT)
            .unwrap();
    }
}
