table! {
    app_user (id) {
        id -> Int8,
        first_name -> Text,
        last_name -> Text,
        username -> Text,
        email -> Text,
        phone_number -> Nullable<Text>,
        active -> Bool,
        created_at -> Timestamptz,
        created_by -> Text,
        updated_at -> Timestamptz,
        updated_by -> Text,
        version -> Int4,
    }
}

table! {
    country (id) {
        id -> Int4,
        code -> Text,
        name -> Text,
        dial_code -> Int2,
        currency -> Text,
    }
}

table! {
    currency (id) {
        id -> Int4,
        code -> Text,
        symbol -> Nullable<Text>,
        name -> Text,
        precision -> Int2,
        format -> Text,
    }
}

table! {
    date_format (id) {
        id -> Int4,
        c_format -> Text,
        date_picker_format -> Text,
        js_format -> Text,
    }
}

table! {
    datetime_format (id) {
        id -> Int4,
        c_format -> Text,
        js_format -> Text,
    }
}

table! {
    language (id) {
        id -> Int4,
        name -> Text,
        locale -> Text,
    }
}

table! {
    notification (id) {
        id -> Int8,
        created_at -> Timestamptz,
        subject -> Text,
        notification_type -> Text,
        from_address -> Nullable<Text>,
        body_html -> Nullable<Text>,
        body_plain_text -> Nullable<Text>,
    }
}

table! {
    notification_attachment (id) {
        id -> Int8,
        notification_id -> Int8,
        name -> Text,
        data -> Bytea,
    }
}

table! {
    notification_recipient (id) {
        id -> Int8,
        notification_id -> Int8,
        recipient_type -> Text,
        name -> Nullable<Text>,
        address -> Text,
    }
}

table! {
    role (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        created_at -> Timestamptz,
        created_by -> Text,
        updated_at -> Timestamptz,
        updated_by -> Text,
        version -> Int4,
    }
}

table! {
    permission (id) {
        id -> Int4,
        application -> Text,
        authority -> Text,
        description -> Text,
    }
}

table! {
    role_permission (role_id, permission_id) {
        role_id -> Int4,
        permission_id -> Int4,
    }
}

table! {
    timezone (id) {
        id -> Int4,
        name -> Text,
        gmt_offset -> Text,
        location -> Text,
    }
}

table! {
    user_credential (id) {
        id -> Int8,
        password_hash -> Nullable<Text>,
        expires_at -> Nullable<Timestamptz>,
        invalid_attempts -> Int4,
        locked -> Bool,
        activation_key -> Nullable<Text>,
        activation_key_expires_at -> Nullable<Timestamptz>,
        activated -> Bool,
        reset_key -> Nullable<Text>,
        reset_key_expires_at -> Nullable<Timestamptz>,
        reset_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
        version -> Int4,
    }
}

table! {
    user_role (user_id, role_id) {
        user_id -> Int8,
        role_id -> Int4,
    }
}

table! {
    user_token (id) {
        id -> Int8,
        user_id -> Int8,
        token_type -> Text,
        token -> Text,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

joinable!(notification_attachment -> notification (notification_id));
joinable!(notification_recipient -> notification (notification_id));
joinable!(role_permission -> role (role_id));
joinable!(role_permission -> permission (permission_id));
joinable!(user_credential -> app_user (id));
joinable!(user_role -> app_user (user_id));
joinable!(user_role -> role (role_id));
joinable!(user_token -> app_user (user_id));

allow_tables_to_appear_in_same_query!(
    app_user,
    country,
    currency,
    date_format,
    datetime_format,
    language,
    notification,
    notification_attachment,
    notification_recipient,
    role,
    role_permission,
    permission,
    timezone,
    user_credential,
    user_role,
    user_token,
);
