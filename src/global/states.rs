use bevy::state::state::States;

/*
                                                                        STATES

                           +----------+
                           | AppState |  <-- PARENT
                           +----------+
                            /        \
                           /          *----------------------------------*
                          /                                               \
                        Menu                                             Game                                                   
                    +-----------+                                    +-----------+                                              
                    | MenuState |                                    | GameState |                                              
                    +-----------+                                    +-----------+                                              
                     /  |  \                                               |                                                    
                   /    |   \                                              |                                                    
                 /      |    \                                            / \                                                   
               /        |     \                                         /     \                                                 
             /         /       \                                      /         \
           Main     Options    ...?                                 /             \
                                                                  /                 \
                                                                /                     \
                                                              /                         \
                                                            Farm                      Kitchen                           
                                                        +-----------+             +--------------+                            +----------+
                                                        | FarmState |             | KitchenState |                            | SimState | <-- applies to both game states
                                                        +-----------+             +--------------+                            +----------+
                                                              |                    /      |      \                               /    \         
                                                             idk                  /       |       \                             /      \
                                                                                 /        |        \                           /        \
                                                                                /         |         \                         /          \
                                                                               /          |          \                     Running     Paused
                                                                          ShiftStart    Shift     ShiftEnd

*/

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)] // have to include this crap for states (except Default)
pub enum AppState {
    #[default]
    Menu,
    Game
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    #[default]
    Main,
    Options,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Farm,
    Kitchen,
}

#[derive(States, /*Default,*/ Debug, Clone, PartialEq, Eq, Hash)]
pub enum FarmState {
    // idk what to put here yet
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum KitchenState {
    #[default]
    ShiftStart,
    Shift,
    ShiftEnd,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimState {
    #[default]
    Running,
    Paused,
}
