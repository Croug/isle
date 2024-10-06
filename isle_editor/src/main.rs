use isle::prelude::*;

fn main() {
    let mut flow = Flow::new()
        .with_default_plugins()
        .build();

    flow.run();
}