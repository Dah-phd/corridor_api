table! {
    users (user) {
        user -> Nullable<Text>,
        password -> Text,
        email -> Text,
        active -> Bool,
    }
}
