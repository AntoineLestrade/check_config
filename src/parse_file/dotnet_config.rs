extern crate regex;
extern crate xml;

use parse_file;

use self::xml::reader::{EventReader,XmlEvent};

pub fn parse(string_content: &String, parsing_options: &super::super::parser_options::ParsingOptions) ->
    Result<Vec<parse_file::ParseFileResult>, parse_file::Error> {
    lazy_static! {
        static ref RE_CONNECTION_STRING: regex::Regex = regex::Regex::new(r"<connectionStrings(?:.*?)>([\s\S]*)</connectionStrings(?:.*?)>").unwrap();
    }
    lazy_static! {
        static ref RE_CS_VALUE: regex::Regex = regex::Regex::new(r"(?i).*data source=(.*?);.*initial catalog=(.*?);.*").unwrap();
    }
    let re_server_value: regex::Regex = regex::Regex::new(parsing_options.regex_server_value.as_str()).unwrap();
    let re_db_value: regex::Regex = regex::Regex::new(parsing_options.regex_database_value.as_str()).unwrap();
    
    let mut result = Vec::<parse_file::ParseFileResult>::new();

    let opt_capture = RE_CONNECTION_STRING.captures(string_content.as_str());
    if opt_capture.is_some() {
        let c0 = opt_capture.unwrap().get(0).unwrap().as_str();
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
                            pfr.server_name = String::from(val.get(1).unwrap().as_str());
                            pfr.db_name = String::from(val.get(2).unwrap().as_str());
                            if re_server_value.is_match(val.get(1).unwrap().as_str()) != parsing_options.regex_server_inverse {
                            	pfr.is_good = false;
                            }
                            if re_db_value.is_match(val.get(2).unwrap().as_str()) != parsing_options.regex_database_inverse {
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

#[cfg(test)]
mod tests {
    use parse_file::dotnet_config;
    
    #[test]
    fn test_default() {
        let content = r##"<?xml version="1.0"?>
    <!-- 
        Note: As an alternative to hand editing this file you can use the 
        Web Site Administration Tool to configure settings for your application. Use
        the Web site->Asp.Net Configuration option in Visual Studio.
        A full list of settings and comments can be found in 
        machine.config.comments usually located in 
        \Windows\Microsoft.Net\Framework\v2.x\Config 
    -->
    <configuration xmlns="http://schemas.microsoft.com/.NetConfiguration/v2.0">
        <appSettings/>
        <connectionStrings>
        <add name="SampleSqlServer" connectionString="Data Source=dabel69.corp.altengroup.dir\sqlexpress;Integrated Security=SSPI;Initial Catalog=Northwind;" />
        </connectionStrings>
        <system.web>
            <!-- 
                Set compilation debug="true" to insert debugging 
                symbols into the compiled page. Because this 
                affects performance, set this value to true only 
                during development.
            -->
            <compilation debug="false"/>
            <!--
                The <authentication> section enables configuration 
                of the security authentication mode used by 
                ASP.NET to identify an incoming user. 
            -->
            <authentication mode="Windows"/>
            <!--
                The <customErrors> section enables configuration 
                of what to do if/when an unhandled error occurs 
                during the execution of a request. Specifically, 
                it enables developers to configure html error pages 
                to be displayed in place of a error stack trace.

            <customErrors mode="RemoteOnly" defaultRedirect="GenericErrorPage.htm">
                <error statusCode="403" redirect="NoAccess.htm"/>
                <error statusCode="404" redirect="FileNotFound.htm"/>
            </customErrors>
            -->
        </system.web>
    </configuration>"##;
        let config = super::super::super::parser_options::ParsingOptions {
            regex_server_value:  r"(?i)(\.|dabel69(\.corp\.altengroup\.dir)?)\\sqlexpress".to_string(),
            regex_server_inverse: false,
            regex_database_value: r"^.*_ALE$".to_string(),
            regex_database_inverse: true,
        };
        let result = dotnet_config::parse(&content.to_string(), &config);
        match result {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false, "Config file is considered as wrong, it should be good");
            }
        }
    }
}