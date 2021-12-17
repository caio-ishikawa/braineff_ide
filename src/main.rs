extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
mod interpreter;

#[derive(Default, NwgUi)]
pub struct Editr { 
    #[nwg_control(size: (600, 300), position: (300, 300), title: "output")]
    output_window: nwg::Window, 

    #[nwg_control(text:"output", parent: output_window)]
    output_label: nwg::RichLabel, 

    #[nwg_control(size: (600, 600), position: (300, 300), title: "EDITR", flags: "MAIN_WINDOW|VISIBLE")]
    #[nwg_events(OnInit:[Editr::init_text])]
    window: nwg::Window,

    #[nwg_control(size:(600, 560), position: (0, 40))]
    code_edit: nwg::RichTextBox, 


    #[nwg_control(text:"Run", size: (100, 40), position: (0, 0))]
    #[nwg_events(OnButtonClick: [Editr::run_code])]
    run_button: nwg::Button, 

    #[nwg_control(text:"Debug", size: (100, 40), position: (100, 0))]
    #[nwg_events(OnButtonClick: [Editr::debug])]
    debug_button: nwg::Button, 

}

impl Editr {
    fn init_text(&self) {
        let text = &self.code_edit;
        text.set_para_format(&nwg::ParaFormat { 
            numbering: Some(nwg::ParaNumbering::Arabic), 
            numbering_style: Some(nwg::ParaNumberingStyle::Plain), 
            numbering_tab: Some(300), 
            alignment: Some(nwg::ParaAlignment::Left),
            start_indent: Some(10),
            right_indent: Some(10), 
            line_spacing: Some(nwg::ParaLineSpacing::Single), 
            offset: Some(30), 
            rtl: Some(false),
            space_after: Some(30), 
            space_before: Some(30)
        });
    }

    fn run_code(&self){
        println!("{:?}", self.code_edit.para_format());
        let code = interpreter::lexer(&self.code_edit.text());
        let parsed_code = interpreter::parse(code);
        let output = interpreter::compile(parsed_code);
        let output_box = &self.output_label;
        output_box.set_text(&output);
        //nwg::simple_message("Output", &output);
    }

    fn debug(&self){ 
        println!("DEBUG MODE");
    }
}

fn main() {
    nwg::init().expect("Failed to initialize NWG");
    let _app = Editr::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
