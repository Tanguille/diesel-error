diesel::table! {
    role_permission (id) {
        id -> Int4,
        role_id -> Int4,
        directory_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        actions -> Array<Nullable<Text>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
