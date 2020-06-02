table! {
    use diesel::sql_types::*;
    use crate::types::*;

    account (id) {
        id -> Int4,
        cached_name -> Nullable<Varchar>,
        acc_type -> Account_type,
        email -> Nullable<Varchar>,
        install_id -> Nullable<Int4>,
        first_login -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::types::*;

    build (id) {
        id -> Int4,
        project -> Int4,
        category -> Varchar,
        ser -> Int4,
        branch -> Varchar,
        sha -> Bpchar,
        path -> Varchar,
        created -> Timestamp,
        creator -> Int4,
        cause -> Build_cause,
        pr_number -> Nullable<Int4>,
        pr_head -> Nullable<Int4>,
        complete -> Bool,
        phar -> Nullable<Int4>,
        raw_log -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::types::*;

    login_history (sess) {
        sess -> Bpchar,
        account -> Nullable<Int4>,
        ip -> Varchar,
        auth -> Timestamp,
        touch -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::types::*;

    project (id) {
        id -> Int4,
        owner -> Int4,
        repo -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::types::*;

    raw_file (id) {
        id -> Int4,
        mime -> Varchar,
        filename -> Varchar,
        source -> Varchar,
        downloads -> Int4,
        created -> Timestamp,
        touch -> Timestamp,
        expiry -> Interval,
        auth -> Int4,
        pref_tabs -> Bool,
        pref_pages -> Bool,
        pref_icons -> Bool,
        pref_dark -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::types::*;

    repo (id) {
        id -> Int4,
        owner -> Int4,
        cached_name -> Varchar,
        private -> Bool,
        fork -> Bool,
    }
}

joinable!(build -> account (creator));
joinable!(build -> project (project));
joinable!(login_history -> account (account));
joinable!(project -> account (owner));
joinable!(project -> repo (repo));
joinable!(repo -> account (owner));

allow_tables_to_appear_in_same_query!(
    account,
    build,
    login_history,
    project,
    raw_file,
    repo,
);
