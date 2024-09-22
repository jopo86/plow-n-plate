use bevy::state::state::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)] // have to include this crap for states (except Default)
pub enum State {
    Menu(Menu),
    Game(Game),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Menu {
    Main,
    Options,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Game {
    Farm,
    Kitchen,
}

/*
! This doesn't apply anymore but I'm keeping it bc it took a lot of work :(

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
