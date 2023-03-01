use mki::{Action, bind_button, InhibitEvent, Mouse, State};

pub fn setup(){
    bind_button(Mouse::Left,Action{
        callback: Box::new(|e,s|{
            if let State::Released = s {

            }
        }),
        inhibit: InhibitEvent::No,
        defer: false,
        sequencer: false,
    })
}