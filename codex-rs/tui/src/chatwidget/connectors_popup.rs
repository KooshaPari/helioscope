use codex_chatgpt::connectors;

use crate::app_event::AppEvent;
use crate::bottom_pane::SelectionItem;
use crate::history_cell;

pub(super) fn connector_selection_item(connector: &connectors::AppInfo) -> SelectionItem {
    let connector_label = connectors::connector_display_label(connector);
    let connector_title = connector_label.clone();
    let link_description = connector_description(connector);
    let description = connector_brief_description(connector);
    let status_label = connector_status_label(connector);
    let search_value = format!("{connector_label} {}", connector.id);
    let mut item = SelectionItem {
        name: connector_label,
        description: Some(description),
        search_value: Some(search_value),
        ..Default::default()
    };
    let is_installed = connector.is_accessible;
    let selected_label = if is_installed {
        format!(
            "{status_label}. Press Enter to open the app page to install, manage, or enable/disable this app."
        )
    } else {
        format!("{status_label}. Press Enter to open the app page to install this app.")
    };
    let missing_label = format!("{status_label}. App link unavailable.");
    let instructions = if connector.is_accessible {
        "Manage this app in your browser."
    } else {
        "Install this app in your browser, then reload Codex."
    };
    if let Some(install_url) = connector.install_url.clone() {
        let app_id = connector.id.clone();
        let is_enabled = connector.is_enabled;
        let title = connector_title.clone();
        let instructions = instructions.to_string();
        let description = link_description.clone();
        item.actions = vec![Box::new(move |tx| {
            tx.send(AppEvent::OpenAppLink {
                app_id: app_id.clone(),
                title: title.clone(),
                description: description.clone(),
                instructions: instructions.clone(),
                url: install_url.clone(),
                is_installed,
                is_enabled,
            });
        })];
        item.dismiss_on_select = true;
        item.selected_description = Some(selected_label);
    } else {
        let missing_label_for_action = missing_label.clone();
        item.actions = vec![Box::new(move |tx| {
            tx.send(AppEvent::InsertHistoryCell(Box::new(
                history_cell::new_info_event(missing_label_for_action.clone(), None),
            )));
        })];
        item.dismiss_on_select = true;
        item.selected_description = Some(missing_label);
    }
    item
}

fn connector_brief_description(connector: &connectors::AppInfo) -> String {
    let status_label = connector_status_label(connector);
    match connector_description(connector) {
        Some(description) => format!("{status_label} · {description}"),
        None => status_label.to_string(),
    }
}

fn connector_status_label(connector: &connectors::AppInfo) -> &'static str {
    if connector.is_accessible {
        if connector.is_enabled {
            "Installed"
        } else {
            "Installed · Disabled"
        }
    } else {
        "Can be installed"
    }
}

fn connector_description(connector: &connectors::AppInfo) -> Option<String> {
    connector
        .description
        .as_deref()
        .map(str::trim)
        .filter(|description| !description.is_empty())
        .map(str::to_string)
}
