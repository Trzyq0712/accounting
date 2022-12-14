use druid::widget::{Align, Button, Flex, Label};
use druid::{
    commands, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, FileDialogOptions,
    Handled, Lens, LocalizedString, Target, Widget, WindowDesc, Application,
};
mod lib;
use std::path::PathBuf;
use std::rc::Rc;

use crate::lib::Ops;

struct Delegate;

#[derive(Clone, Default, Data, Lens)]
struct AppState {
    selected_file: Rc<PathBuf>,
}

fn ui_builder() -> impl Widget<AppState> {
    let input = Label::dynamic(|data: &AppState, _| {
        format!("Path: {}", data.selected_file.to_str().unwrap())
    });

    let allowed_filetypes = FileDialogOptions::new();
    let open_btn = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(allowed_filetypes.clone()))
    });

    let buttons = lib::OPERATIONS.iter().map(|op| {
        Button::new(<lib::Ops as Into<&str>>::into(*op))
            .on_click(|ctx, _: &mut AppState, _| ctx.submit_command(op.into_selector().with(())))
    });

    let mut col = Flex::column();
    col.add_child(input);
    col.add_spacer(8.0);
    col.add_child(open_btn);
    col.add_spacer(20.0);
    for btn in buttons {
        col.add_child(btn);
        col.add_spacer(5.0);
    }
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

        if let Some(_) = cmd.get(Ops::Allegro.into_selector()) {
            let processor = lib::allegro::AllegroProcessor;
            use lib::Processor;
            let result = processor.process(data.selected_file.as_ref());
            let s = String::from_utf8(result.unwrap().into_inner().unwrap()).unwrap().replace(".", ",");
            let mut clipboard = Application::global().clipboard();
            clipboard.put_string(&s);
            println!("{:?}", s);
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
