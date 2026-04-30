use ratatui::prelude::Stylize as _;
use ratatui::text::Line;
use ratatui::text::Span;

#[derive(Clone)]
pub(crate) struct ModelMigrationCopy {
    pub heading: Vec<Span<'static>>,
    pub content: Vec<Line<'static>>,
    pub can_opt_out: bool,
    pub markdown: Option<String>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn migration_copy_for_models(
    current_model: &str,
    target_model: &str,
    model_link: Option<String>,
    migration_copy: Option<String>,
    migration_markdown: Option<String>,
    target_display_name: String,
    target_description: Option<String>,
    can_opt_out: bool,
) -> ModelMigrationCopy {
    if let Some(migration_markdown) = migration_markdown {
        return ModelMigrationCopy {
            heading: Vec::new(),
            content: Vec::new(),
            can_opt_out,
            markdown: Some(fill_migration_markdown(
                &migration_markdown,
                current_model,
                target_model,
            )),
        };
    }

    let heading_text = Span::from(format!(
        "Codex just got an upgrade. Introducing {target_display_name}."
    ))
    .bold();
    let description_line: Line<'static>;
    if let Some(migration_copy) = &migration_copy {
        description_line = Line::from(migration_copy.clone());
    } else {
        description_line = target_description
            .filter(|desc| !desc.is_empty())
            .map(Line::from)
            .unwrap_or_else(|| {
                Line::from(format!(
                    "{target_display_name} is recommended for better performance and reliability."
                ))
            });
    }

    let mut content = vec![];
    if migration_copy.is_none() {
        content.push(Line::from(format!(
            "We recommend switching from {current_model} to {target_model}."
        )));
        content.push(Line::from(""));
    }

    if let Some(model_link) = model_link {
        content.push(Line::from(vec![
            format!("{description_line} Learn more about {target_display_name} at ").into(),
            model_link.cyan().underlined(),
        ]));
        content.push(Line::from(""));
    } else {
        content.push(description_line);
        content.push(Line::from(""));
    }

    if can_opt_out {
        content.push(Line::from(format!(
            "You can continue using {current_model} if you prefer."
        )));
    } else {
        content.push(Line::from("Press enter to continue".dim()));
    }

    ModelMigrationCopy {
        heading: vec![heading_text],
        content,
        can_opt_out,
        markdown: None,
    }
}

fn fill_migration_markdown(template: &str, current_model: &str, target_model: &str) -> String {
    template
        .replace("{model_from}", current_model)
        .replace("{model_to}", target_model)
}
