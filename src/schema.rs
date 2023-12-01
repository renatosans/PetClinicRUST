// @generated automatically by Diesel CLI.

/*
diesel::table! {
    appointment (id) {
        id -> Int4,
        date -> Nullable<Date>,
        veterinarian -> Int4,
        petowner -> Int4,
    }
}

diesel::table! {
    pet (id) {
        id -> Int4,
        #[max_length = 120]
        name -> Varchar,
        #[max_length = 45]
        breed -> Nullable<Varchar>,
        age -> Nullable<Int4>,
        owner -> Nullable<Int4>,
        flag_removed -> Bool,
    }
}

diesel::table! {
    petowner (id) {
        id -> Int4,
        #[max_length = 120]
        name -> Varchar,
        birth_date -> Nullable<Date>,
        #[max_length = 80]
        email -> Varchar,
        #[max_length = 45]
        phone -> Nullable<Varchar>,
        #[max_length = 120]
        address -> Varchar,
    }
}

diesel::table! {
    treatment (id) {
        id -> Int4,
        #[max_length = 120]
        description -> Varchar,
        pet -> Int4,
        veterinarian -> Int4,
    }
}

diesel::table! {
    vaccination (id) {
        id -> Int4,
        #[max_length = 120]
        description -> Varchar,
        pet -> Int4,
    }
}

diesel::table! {
    veterinarian (id) {
        id -> Int4,
        #[max_length = 120]
        name -> Varchar,
        #[max_length = 75]
        inscricaoCRMV -> Varchar,
    }
}

diesel::joinable!(treatment -> pet (pet));
diesel::joinable!(treatment -> veterinarian (veterinarian));
diesel::joinable!(vaccination -> pet (pet));

diesel::allow_tables_to_appear_in_same_query!(
    appointment,
    pet,
    petowner,
    treatment,
    vaccination,
    veterinarian,
);
*/
