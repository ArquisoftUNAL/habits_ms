CREATE TABLE habit (
    hab_id UUID PRIMARY KEY,
    hab_name VARCHAR(255) NOT NULL,
    hab_description VARCHAR(255) NOT NULL,
    hab_created_at TIMESTAMP NOT NULL,
    hab_updated_at TIMESTAMP NOT NULL,
    hab_type VARCHAR(10) NOT NULL,
    hab_is_favorite BOOLEAN NOT NULL,
    hab_color VARCHAR(6) NOT NULL,
    hab_units VARCHAR(10) NOT NULL,
    usr_id VARCHAR(24) NOT NULL,
    cat_id UUID NOT NULL,

    CONSTRAINT habit_cat_id_fk
        FOREIGN KEY (cat_id) 
            REFERENCES category(cat_id)
);