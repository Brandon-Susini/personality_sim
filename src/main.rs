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
#[derive(Debug, Component)]
struct N;
#[derive(Debug, Component)]
struct S;

#[derive(Debug, Component)]
struct Function(Component<N>,Component<S>);

#[derive(Debug, Component)]
struct Person;

#[derive(Debug, Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(print_people)
        .run();
}
fn sim_tick(){

}
//Move the mut to the type definition and see what happens.

fn add_people(mut commands: Commands){
    commands.spawn((Person,Name("Ne".to_string()), N));
    commands.spawn((Person, Name("Ni".to_string()), N));
    commands.spawn((Person, Name("Si".to_string()), S));
}


fn print_people(search: Query<&Name, (With<Person>,With<S>)>){
    for name in search.iter(){
        println!("{}",name.0);
    }
}
