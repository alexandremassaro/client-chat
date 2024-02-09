use mongodb::bson::{DateTime, oid::ObjectId};

struct User {
    _id: Option<ObjectId>,
    username: String,
    nickname: String,
    created_at: DateTime,
    text_services: Vec<TextService>,
}

struct TextService {
    service_type: TextServiceType,
    contact_info: String,
}

enum TextServiceType {
    Facebook,
    Instagram,
    Sms,
    Whatsapp,
}