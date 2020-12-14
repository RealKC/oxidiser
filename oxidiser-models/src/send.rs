use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Packet {
    #[serde(rename = "pong")]
    Pong,

    #[serde(rename = "auth")]
    Auth { username: String },

    #[serde(rename = "lobby-info")]
    LobbyInfo,
}

impl Packet {
    pub fn encoded(&self) -> Result<Vec<u8>, serde_json::Error> {
        let mut encoded = serde_json::to_vec(&self)?;
        encoded.push(b'\0');
        Ok(encoded)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pong() {
        println!("{}", serde_json::to_string(&Packet::Pong).unwrap());
    }

    #[test]
    fn test_auth() {
        println!(
            "{}",
            serde_json::to_string(&Packet::Auth {
                username: "foofooo".into()
            })
            .unwrap()
        )
    }
}
