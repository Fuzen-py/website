use actix_web::pred;
#[derive(Copy, Clone, Debug)]
pub enum Hosts {
    FuzenInfo,
    FuzenCafe,
    NekoClaims,
}

#[derive(Debug, Fail)]
pub enum HostError {
    #[fail(display = "Unknown Host: {}", host)]
    UnknownHost { host: String },
}

impl Hosts {
    pub fn filter(self, app: actix_web::App) -> actix_web::App {
        app.filter(self.create_pred())
    }
    pub fn create_pred<S: 'static>(self) -> pred::AnyPredicate<S> {
        match self {
            Hosts::FuzenInfo => {
                pred::Any(pred::Host("fuzen.info")).or(pred::Host("test.fuzen.info"))
            }
            Hosts::FuzenCafe => {
                pred::Any(pred::Host("fuzen.cafe")).or(pred::Host("test.fuzen.cafe"))
            }
            Hosts::NekoClaims => {
                pred::Any(pred::Host("neko.claims")).or(pred::Host("test.neko.claims"))
            }
        }
    }
}

impl std::convert::Into<::actix_web::App> for Hosts {
    fn into(self) -> actix_web::App {
        self.filter(::actix_web::App::new())
    }
}

impl<S: 'static> std::convert::Into<pred::AnyPredicate<S>> for Hosts {
    fn into(self) -> pred::AnyPredicate<S> {
        self.create_pred()
    }
}

impl std::fmt::Display for Hosts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}

impl std::str::FromStr for Hosts {
    type Err = HostError;
    fn from_str(s: &str) -> Result<Hosts, HostError> {
        match s {
            "fuzen.info" | "test.fuzen.info" => Ok(Hosts::FuzenInfo),
            "fuzen.cafe" | "test.fuzen.cafe" => Ok(Hosts::FuzenCafe),
            "neko.claims" | "test.neko.claims" => Ok(Hosts::NekoClaims),
            _ => Err(HostError::UnknownHost {
                host: String::from(s),
            }),
        }
    }
}

impl std::convert::AsRef<str> for Hosts {
    fn as_ref(&self) -> &str {
        match self {
            Hosts::FuzenCafe => "fuzen.cafe",
            Hosts::FuzenInfo => "fuzen.info",
            Hosts::NekoClaims => "neko.claims",
        }
    }
}
