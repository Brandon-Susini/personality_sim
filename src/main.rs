use bevy::prelude::*;
//mod person;
/*NAMING: Named just N to represent one atomic element of a function.
//N,S,T,F -> Names of elements
    N -> Fantasy (Conceptual)
        Looks at connections between objects and concepts, interested in novelity and understanding
    S -> Reality (Literal)
        |--------------------|
    --
    T -> Object Oriented. 
        Impersonal, Goal-oriented, Seeks Acheivement
    F -> People Oriented. 
        Values, Emotions, motivations, etc...
//E,I -> Attitudes: 
//  Extraverted (Objective) & Introverted (Expected)
*/
//#[derive(Debug, Component)]
/* struct N;
#[derive(Debug, Component)]
struct S;
#[derive(Debug, Component)]
struct T;
#[derive(Debug, Component)]
struct F; */
#[derive(Debug)]
enum Element{
    N,
    S,
    F,
    T,
}
#[derive(Debug,Component)]
struct Type{
    elements: Vec<Element>
}
//Type{elements: vec![Element::N]}
impl Type{
    fn new(element: Element) -> Type{
        Type{elements: vec!(element)}
    }
}


#[derive(Debug, Component)]
struct Person;

#[derive(Debug, Component)]
struct Name(String);

#[derive(Debug,Resource)]
struct SimTimer(Timer);
//Interface for elements.
/* trait Element{
    fn is_observer(&self) -> bool;
}
impl Element for N{
    fn is_observer(&self) -> bool{
        true
    }
} */
/* #[derive(Debug,Component)]
struct TimeKeeper{
    ticks_passed: i32
}
impl TimeKeeper{
    fn new()->TimeKeeper{
        TimeKeeper{ticks_passed: 0}
    }
    fn advance(&mut self){
        self.ticks_passed += 1;
    }
} */
#[derive(Debug,Component)]
struct TimeKeeper(i32);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SimTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_startup_system(start)
        .add_startup_system(add_people)
        .add_system(print_people)
        .add_system(sim_tick)
        .run();
}
fn start(mut commands: Commands){
    commands.spawn(TimeKeeper(0));
}
/*
    Potentially turn this if statement into a macro that takes an argument for the string to be printed.
    Look at the get_component_mut example and figure that out
    */
fn sim_tick(time: Res<Time>, mut timer: ResMut<SimTimer>, mut tk: Query<&mut TimeKeeper>){
    if timer.0.tick(time.delta()).just_finished(){
        //Every tick...
        for mut t in tk.iter_mut(){
            t.0 += 1;
            println!("Tick {}",t.0);
            //tk.get_component_mut(entity)
        }
    }
}
//Move the mut to the type definition and see what happens.
fn add_people(mut commands: Commands){
    /* commands.spawn((Person,Name("Ne".to_string()), N));
    commands.spawn((Person, Name("Ni".to_string()), N));
    commands.spawn((Person, Name("Si".to_string()), S)); */
    commands.spawn((Person,Name("Ne".to_string()), Type::new(Element::N)));
    //commands.spawn((Person, Name("Ni".to_string()), Element::N));
    //commands.spawn((Person, Name("Si".to_string()), Element::T));
}


fn print_people(search: Query<&Type, With<Person>>){
    for t in search.iter(){
        ()
        //println!("{:?}",t.elements[0]);
    }
}
