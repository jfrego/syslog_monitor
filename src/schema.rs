// @generated automatically by Diesel CLI.

diesel::table! {
    extensions (mac) {
        mac -> Text,
        extension -> Integer,
        domain -> Text,
        timer -> Text,
        mail -> Bool,
    }
}
