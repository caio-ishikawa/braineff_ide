extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
mod interpreter;

#[derive(Default, NwgUi)]
pub struct Editr { 
    #[nwg_control(size: (600, 600), position: (300, 300), title: "EDITR")]
    #[nwg_events(OnInit:[Editr::init_text])]
    window: nwg::Window,

    #[nwg_control(size:(600, 580), position: (0, 20))]
    code_edit: nwg::RichTextBox, 

    #[nwg_control(text:"Run", size: (200, 20), position: (20, 0))]
    #[nwg_events(OnButtonClick: [Editr::say_hello])]
    run_button: nwg::Button, 
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

    fn say_hello(&self){
        println!("{:?}", self.code_edit.para_format());
        let code = interpreter::lexer(&self.code_edit.text());
        let parsed_code = interpreter::parse(code);
        let mut output = interpreter::compile(parsed_code);
        let output_sliced = output.as_str();
        nwg::modal_info_message(&self.window, &output, &format!("OUTPUT: {}", output));
    }
}

fn main() {
    nwg::init().expect("Failed to initialize NWG");
    let _app = Editr::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
