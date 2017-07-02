pub enum Target {
    Email(String),
    Device { ident: String },
    Client { ident: String },
    Channel { tag: String },
    None,
}

impl Target {
    pub fn to_json_field(&self) -> Option<(&'static str, String)> {
        match self {
            &Target::Email(ref v) => Some(("email", v.to_owned())),
            &Target::Device { ref ident } => Some(("device_iden",
            ident.to_owned())),
            &Target::Client { ref ident } => Some(("client_iden",
            ident.to_owned())),
            &Target::Channel { ref tag } => Some(("channel_tag", tag.to_owned())),
            _ => None
        }
    }
}

/*
impl Serialize for Target {
    fn serialize(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer {   
    match *self {
        Email(ref v) => {
            let s = serializer.serialize_struct_variant(
                "Target", 0, "Email", 1);
            s.serialize_field("email", v);
            s.end()
        }        
        Device { ident }  => {
            let s = serializer.serialize_struct_variant(
                "Target", 1, "Device", 1);
            s.serialize_field("device_iden", ident);
            s.end()
        },
        Client { ident } => {
            let s = serializer.serialize_struct_variant(
                "Target", 2, "Client", 1);
            s.serialize_field("client_iden", ident);
            s.end()
        },
        Channel { tag } => {
            serializer.serialize_struct_variant(
                "Target", 3, "Channel", 1);
            s.serialize_field("channel_tag", tag);
            s.end()
        },
        None => {
            s.serialize_none()
        },
    }
}
*/
