
use super::action::Action;

#[derive(Default,Copy,Clone)]
pub(crate) struct State{
    pub(crate) is_reverse: bool,
    pub(crate) is_acceleration: bool,
}

impl State{
    pub(crate) fn be_reverse(&mut self){
        self.is_reverse = !self.is_reverse;
    }
    pub(crate) fn be_acceleration(&mut self){
        self.is_acceleration = !self.is_acceleration;
    }
    pub(crate) fn assemble(&self,cmd:char) -> Vec<Action>{
        match cmd{
            'M' => self.move_assemble(),
            'L' => self.turn_left_assemble(),
            'R' => self.turn_right_assemble(),
            _ => Vec::new(),
        }
    }
    fn acceleration_move(&self,actions: &mut Vec<Action>){
        if self.is_acceleration{
            let direction = if self.is_reverse{-1} else{1};
            let action = Action::Forward(direction);
            actions.push(action);
        }
    }
    fn move_assemble(&self)-> Vec<Action>{
        let mut actions:Vec<Action>= Vec::new();
        self.acceleration_move(&mut actions);
        let direction = if self.is_reverse{-1} else {1};
        let action = Action::Forward(direction);
        actions.push(action);

        actions
    }

    fn turn_left_assemble(&self) -> Vec<Action>{
        let mut actions = Vec::new();
        self.acceleration_move(&mut actions);
        let turn_action = if self.is_reverse{
            Action::TurnRight
        }else{
            Action::TurnLeft
        };
        actions.push(turn_action);

        actions
    }

    fn turn_right_assemble(&self)-> Vec<Action>{
        let mut actions = Vec::new();
        self.acceleration_move(&mut actions);
        let turn_action = if self.is_reverse{
            Action::TurnLeft
        }else{
            Action::TurnRight
        };
        actions.push(turn_action);

        actions
    }
}