extern crate regex;
extern crate xml;

use parse_file;

use self::xml::reader::{EventReader,XmlEvent};

pub fn parse(string_content: &String) ->
    Result<Vec<parse_file::ParseFileResult>, parse_file::Error> {
    lazy_static! {
        static ref RE_CONNECTION_STRING: regex::Regex = regex::Regex::new(r"<connectionStrings(?:.*?)>([\s\S]*)</connectionStrings(?:.*?)>").unwrap();
    }
    lazy_static! {
        static ref RE_CS_VALUE: regex::Regex = regex::Regex::new(r"(?i).*data source=(.*?);.*initial catalog=(.*?);.*").unwrap();
    }
    lazy_static!{
    	static ref RE_SERVER_VALUE: regex::Regex = regex::Regex::new(r"(?i)(\.|dabel69(\.corp\.altengroup\.dir)?)\\sqlexpress").unwrap();
    }
    
    lazy_static!{
    	static ref RE_WRONG_DB_NAME: regex::Regex = regex::Regex::new(r"^.*_...$").unwrap();
    }

    let mut result = Vec::<parse_file::ParseFileResult>::new();

    let opt_capture = RE_CONNECTION_STRING.captures(string_content.as_str());
    if opt_capture.is_some() {
        let c0 = opt_capture.unwrap().at(0).unwrap();
        let list = EventReader::from_str(c0);
        for e in list {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    if name.local_name == "add" {
                        let mut pfr = parse_file::ParseFileResult {
                            is_good: false,
                            cs_name: Default::default(),
                            server_name: Default::default(),
                            db_name: Default::default(),
                        };
                        let mut buff_value : String = Default::default();
                        for attr in attributes.into_iter() {
                            let val = attr.name.local_name.as_str();
                            match val {
                                "name" => { pfr.cs_name = attr.value; }
                                "connectionString" => { buff_value = attr.value; }
                                _ => { }
                            };
                        }
                        if let Some(val) =  RE_CS_VALUE.captures(&*buff_value) {
                        	pfr.is_good = true;
                            pfr.server_name = String::from(val.at(1).unwrap());
                            pfr.db_name = String::from(val.at(2).unwrap());
                            if !RE_SERVER_VALUE.is_match(val.at(1).unwrap()) {
                            	pfr.is_good = false;
                            }
                            if RE_WRONG_DB_NAME.is_match(val.at(2).unwrap()) {
                            	pfr.is_good = false;
                            }
                        }
                        result.push(pfr);

                    }
                }
                _ => {}
            };
        }
    }
    if result.len() == 0 {
        return Err(parse_file::Error::CannotFindConnectionString);
    }
    return Ok(result);
}
