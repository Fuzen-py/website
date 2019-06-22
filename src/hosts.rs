use std::str::FromStr;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Hosts {
    FuzenInfo,
    FuzenCafe,
    NekoClaims,
}

#[derive(Debug, failure::Fail)]
pub enum HostError {
    #[fail(display = "Unknown Host: {}", host)]
    UnknownHost { host: String },
}

impl std::fmt::Display for Hosts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}

impl FromStr for Hosts {
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

impl actix_web::guard::Guard for Hosts {
    fn check(&self, request: &actix_web::dev::RequestHead) -> bool {
        request
            .headers
            .get("HOST")
            .and_then(|host| host.to_str().ok())
            .and_then(|host| Hosts::from_str(&host).ok())
            .and_then(|host| Some(host == *self))
            .unwrap_or(false)
    }
}

impl std::convert::Into<String> for Hosts {
    fn into(self) -> String {
        self.to_string()
    }
}
