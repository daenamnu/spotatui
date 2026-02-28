use crate::core::app::App;
use crate::infra::network::sync::{ControlMode, PartyStatus};
use crate::infra::network::IoEvent;
use crate::tui::event::Key;

pub fn handler(key: Key, app: &mut App) {
  match app.party_status {
    PartyStatus::Disconnected | PartyStatus::Connecting => {
      handle_disconnected_menu(key, app);
    }
    PartyStatus::Hosting => {
      handle_hosting_menu(key, app);
    }
    PartyStatus::Joined => {
      handle_joined_menu(key, app);
    }
  }
}

fn handle_disconnected_menu(key: Key, app: &mut App) {
  if app.party_input.is_empty() && !app.party_status.eq(&PartyStatus::Connecting) {
    match key {
      Key::Esc => {
        app.pop_navigation_stack();
      }
      Key::Char('1') | Key::Char('h') => {
        app.dispatch(IoEvent::StartParty(ControlMode::HostOnly));
      }
      Key::Char('2') | Key::Char('j') | Key::Char('J') => {
        // Switch to "Enter code" view (one space so the code-entry UI is shown).
        app.party_input = vec![' '];
        app.party_input_idx = 0;
      }
      Key::Enter => {
        app.dispatch(IoEvent::StartParty(ControlMode::HostOnly));
      }
      _ => {}
    }
  } else {
    handle_code_input(key, app);
  }
}

fn code_alphanumeric_len(party_input: &[char]) -> usize {
  party_input.iter().filter(|c| c.is_alphanumeric()).count()
}

fn handle_code_input(key: Key, app: &mut App) {
  match key {
    Key::Esc => {
      app.party_input.clear();
      app.party_input_idx = 0;
    }
    Key::Enter => {
      let code: String = app
        .party_input
        .iter()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect();
      if code.len() == 6 {
        app.dispatch(IoEvent::JoinParty(code));
        app.party_input.clear();
        app.party_input_idx = 0;
      }
    }
    Key::Backspace => {
      if app.party_input_idx > 0 {
        app.party_input_idx -= 1;
        app.party_input.remove(app.party_input_idx);
      }
    }
    Key::Char(c) if c.is_alphanumeric() && code_alphanumeric_len(&app.party_input) < 6 => {
      app
        .party_input
        .insert(app.party_input_idx, c.to_ascii_uppercase());
      app.party_input_idx += 1;
    }
    _ => {}
  }
}

fn handle_hosting_menu(key: Key, app: &mut App) {
  match key {
    Key::Esc => {
      app.pop_navigation_stack();
    }
    Key::Char('l') | Key::Char('L') => {
      app.dispatch(IoEvent::LeaveParty);
    }
    Key::Char('c') | Key::Char('C') => {
      if let Some(session) = &app.party_session {
        let new_mode = match session.control_mode {
          ControlMode::HostOnly => ControlMode::SharedControl,
          ControlMode::SharedControl => ControlMode::HostOnly,
        };
        if let Some(session) = &mut app.party_session {
          session.control_mode = new_mode;
        }
      }
    }
    _ => {}
  }
}

fn handle_joined_menu(key: Key, app: &mut App) {
  match key {
    Key::Esc => {
      app.pop_navigation_stack();
    }
    Key::Char('l') | Key::Char('L') => {
      app.dispatch(IoEvent::LeaveParty);
    }
    _ => {}
  }
}
