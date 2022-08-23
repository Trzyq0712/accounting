use druid::widget::{Align, Button, Flex, Label};
use druid::{
    commands, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, FileDialogOptions,
    Handled, Lens, LocalizedString, Target, Widget, WindowDesc,
};
use std::path::PathBuf;
use std::rc::Rc;

struct Delegate;

#[derive(Clone, Default, Data, Lens)]
struct AppState {
    selected_file: Rc<PathBuf>,
}

fn ui_builder() -> impl Widget<AppState> {
    let input = Label::dynamic(|data: &AppState, _| format!("Path: {:?}", data.selected_file));

    let allowed_filetypes = FileDialogOptions::new();
    let open_btn = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(Command::new(
            druid::commands::SHOW_OPEN_PANEL,
            allowed_filetypes.clone(),
            Target::Auto,
        ))
    });

    let mut col = Flex::column();
    col.add_child(input);
    col.add_spacer(8.0);
    col.add_child(open_btn);
    Align::centered(col)
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.selected_file = Rc::new(file_info.path().to_path_buf());
            return Handled::Yes;
        }
        Handled::No
    }
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("open-save-demo").with_placeholder("Opening/Saving Demo"));
    let data = AppState::default();
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}
