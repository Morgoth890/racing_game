use amethyst::{
    SimpleState,
    GameData,
    StateData,
    StateEvent,
    SimpleTrans,
    Trans,
    input::{is_close_requested, is_key_down},
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    ecs::Entity,
    winit::VirtualKeyCode,
};

use crate::rgame::MyState;
use crate::util::delete_hierarchy;

const BUTTON_START: &str = "start";
const BUTTON_QUIT: &str = "quit";

#[derive(Default, Debug)]
pub struct WelcomeState {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_quit: Option<Entity>
}

impl SimpleState for WelcomeState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ui_root = Some(data.world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(entity) = self.ui_root {
            delete_hierarchy(entity, data.world).expect("Failed to remove menu")
        }
        self.ui_root = None;
        self.button_start = None;
        self.button_quit = None;
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent
    ) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent {
                               event_type: UiEventType::Click,
                               target
                           }) => {
                if Some(target) == self.button_start {
                    Trans::Switch(Box::new(MyState::default()))
                } else if Some(target) == self.button_quit {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        if self.button_start.is_none() || self.button_quit.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_quit = ui_finder.find(BUTTON_QUIT);
            });
        }

        Trans::None
    }
}

