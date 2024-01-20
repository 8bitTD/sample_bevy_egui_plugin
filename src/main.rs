use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::prelude::*;
mod ime;
#[derive(Resource)] 
pub struct MyApp{
    texts: Vec<String>,
}

impl Default for MyApp{
    fn default() -> Self{
        MyApp{
            texts: Vec::new(),
        }
    }
}
fn main() {
    App::new()   
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            position: WindowPosition::new(IVec2::new( 400, 200)),
            resolution: (440.0, 200.0).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(EguiPlugin)
    .add_plugins(ime::ImePlugin) 
    .insert_resource(MyApp::default())
    .add_systems(Startup, setup_system)
    .add_systems(Update, 
        (
            ui_system,
        ) 
    )      
    .run();
}

pub fn setup_system(
    mut egui_context: EguiContexts,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.ime_enabled = true;
    let mut txt_font = egui::FontDefinitions::default();
    txt_font.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Meiryo".to_owned());
    let fd = egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/Meiryo.ttc"));
    txt_font.font_data.insert("Meiryo".to_owned(), fd);
    egui_context.ctx_mut().set_fonts(txt_font); 
}

pub fn ui_system(
    mut contexts: EguiContexts, 
    mut app: ResMut<MyApp>, 
    mut ime: ResMut<ime::ImeManager>, 
) {
    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui|{
            if ui.button("追加").clicked(){
                app.texts.push(String::new());
            }
            if ui.button("削除").clicked(){
                if app.texts.len() != 0{
                    let n = app.texts.len()-1;
                    app.texts.remove(n);
                }
            }
        });
        for (u, t) in app.texts.iter_mut().enumerate(){
            ui.horizontal(|ui|{
                let teo = match u%2 == 0{
                    true => {ime.text_edit_singleline(t, 200.0, ui, ctx)},
                    _ =>    {ime.text_edit_multiline(t, 200.0, ui, ctx)},
                };
                if teo.response.changed(){
                    println!("id_{}: {}", u, t);
                }
            });
        }
    });
}