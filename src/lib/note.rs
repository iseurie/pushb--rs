use std::io::Write;
use curl::easy::{Easy, List};
use serde::ser::{Serializer, Serialize, SerializeStruct};
use super::{Push, Target, Error};
use serde_json;

pub struct Note {
    pub target: Target,
    pub title: String,
    pub body: String,
}

impl Default for Note {
    fn default() -> Note {
        Note {
            target: Target::None,
            title: "".to_string(),
            body: "".to_string(),
        }
    }
}

impl Serialize for Note {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer {
        let mut s = serializer.serialize_struct("Note", 3)?;
        s.serialize_field(&"type", &"note")?;
        s.serialize_field(&"title", &self.title)?;
        s.serialize_field(&"body", &self.body)?;
        if let Some(ref t) = self.target.to_json_field() {
            s.serialize_field(t.0, &t.1)?;
        } 
        s.end()
    }
}

impl Push for Note {
    fn push(&self, key: &str) -> super::Result<()> {
        let mut reply = Vec::new();
        let mut easy = Easy::new();
        easy.url("https://api.pushbullet.com/v2/pushes")?;
        easy.custom_request("POST")?;
        let mut headers = List::new();
        headers.append("Content-Type: application/json")?;
        headers.append(&("Access-Token: ".to_string() + key))?;
        easy.http_headers(headers)?;
        {
            let mut handle = easy.transfer();
            handle.read_function(|into| {
                let mut sv = serde_json::to_vec(&self).unwrap();
                Ok(sv.write(into).unwrap())
            })?;
            handle.write_function(|data| {
                reply.extend_from_slice(data);
                Ok(data.len())
            })?;
            handle.perform()?; 
        }
        let status = easy.response_code().unwrap();
        if status != 200 {
            let err = serde_json::from_slice(&reply).ok();
            Err(Error::Api(status, err))
        } else { Ok(()) }
    }
}
