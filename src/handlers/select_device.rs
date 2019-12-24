use super::{
    super::app::{ActiveBlock, App},
    common_key_events,
};
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Library), None);
        }
        k if common_key_events::down_event(k) => {
            match &app.devices {
                Some(p) => {
                    if let Some(selected_device_index) = app.selected_device_index {
                        let next_index = common_key_events::on_down_press_handler(
                            &p.devices,
                            Some(selected_device_index),
                        );
                        app.selected_device_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        k if common_key_events::up_event(k) => {
            match &app.devices {
                Some(p) => {
                    if let Some(selected_device_index) = app.selected_device_index {
                        let next_index = common_key_events::on_up_press_handler(
                            &p.devices,
                            Some(selected_device_index),
                        );
                        app.selected_device_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        Key::Char('\n') => {
            if let (Some(devices), Some(index)) = (&app.devices, app.selected_device_index) {
                if let Some(device) = &devices.devices.get(index) {
                    match app.client_config.set_device_id(device.id.clone()) {
                        Ok(()) => {
                            app.pop_navigation_stack();
                        }
                        Err(e) => {
                            app.handle_error(e);
                        }
                    };
                }
            };
        }
        _ => {}
    }
}
