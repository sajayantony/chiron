use lifec::{
    editor::RuntimeEditor,
    plugins::{Plugin, Project, ThunkContext},
    Runtime,
};
use crate::{
    host::Host, create_runtime,
};

/// Lab component hosts a runtime
#[derive(Default)]
pub struct Lab;

impl Plugin<ThunkContext> for Lab {
    fn symbol() -> &'static str {
        "lab"
    }

    fn description() -> &'static str {
        "Starts a lab runtime w/ {project_src}"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        context.clone().task(|cancel_source| {
            let tc = context.clone();
            async move {
                if let Some(project_src) = tc.as_ref().find_text("project_src") {
                    if let Some(project) = Project::load_file(project_src) {
                        let runtime = create_runtime(project);
                        let mut extension = Host(
                            RuntimeEditor::new(runtime), 
                            false
                        );

                        let block_symbol = "lab";
                        Runtime::start_with(
                            &mut extension, 
                            block_symbol, 
                            &tc, 
                            cancel_source
                        );
                    }
                }

                Some(tc)
            }
        })
    }
}
