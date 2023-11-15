use isle_ecs::ecs::{
    System,
    IntoSystem,
    ECS,
};

fn main() {
    let mut ecs = ECS::new();
    ecs.add_system(my_system);
}

fn my_system() {

}
