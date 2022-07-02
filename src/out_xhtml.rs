use std::fs::File;
use std::io::Write;
use crate::Token;

pub struct Out_xhtml {
    filename: String,
    lst: Vec<Token>,
    index: usize
}

impl Out_xhtml {
    pub fn new(name: String, tkn_vec: Vec<Token>) -> Out_xhtml {
        Out_xhtml {
            filename: name,
            lst: tkn_vec,
            index: 0
        }
    }

    pub fn prt_out(&mut self) -> std::io::Result<()> {
        self.filename.push_str(".xhtml");
        let mut file = File::create(&self.filename)?; 

        write!(file, "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n");
        write!(file, "<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">\n");
        write!(file, "<head>\n<title>\nX Formatted file</title>\n</head>\n<body bgcolor=\"navy\" text=\"orange\" link=\"orange\" vlink=\"orange\">\n<font face=\"Courier New\">\n ");
        
        let mut num_tab = 0;
        
        for i in &self.lst { 
            let mut count = 0;
            if i.token_type == "identifier" {
                write!(file, "<font color=\"yellow\">{}</font> ", i.text);
            }
            else if i.token_type == "floatconstant" || i.token_type == "intconstant" {
                write!(file, "<font color=\"aqua\"><b>{}</b></font> ", i.text);
            }
            else {
                write!(file, "<font color=\"white\"><b>{}</b></font> ", i.text);
                if i.text == ";" {
                    write!(file, "<br />\n"); 
                    if &self.lst[self.index+1].text != "}" && num_tab > 0 {
                        write!(file, "&nbsp;&nbsp; &nbsp;");

                    }
                }
                else if i.text == "}" {
                    write!(file, "<br />\n");
                    num_tab -= 1;
                }
                else if i.text == "{" {
                    write!(file, "<br />\n&nbsp;&nbsp; &nbsp;");
                    
                    while count < num_tab {
                        write!(file, " &nbsp; &nbsp;");
                        count += 1;
                    }
                    num_tab += 1;
                }
            }
            self.index += 1;
        }

        write!(file, "</font>\n</body>\n</html>\n");
        Ok(())
    }
}

