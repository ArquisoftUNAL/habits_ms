CREATE TYPE hab_freq_type_enum AS ENUM(
    'daily', 'daily2',
    'weekly', 'weekly2', 
    'monthly', 'monthly2' 
);

CREATE TABLE habit (
    hab_id UUID PRIMARY KEY,
    hab_name VARCHAR(255) NOT NULL,
    hab_description VARCHAR(255) NOT NULL,
    hab_created_at TIMESTAMP NOT NULL,
    hab_updated_at TIMESTAMP NOT NULL,
    hab_is_yn BOOLEAN NOT NULL,
    hab_is_favorite BOOLEAN NOT NULL,
    hab_color VARCHAR(6) NOT NULL,
    hab_units VARCHAR(10) NOT NULL,

    hab_goal DECIMAL(10,2) NOT NULL,
    hab_freq_type hab_freq_type_enum NOT NULL,
    
    usr_id VARCHAR(24) NOT NULL,
    cat_id UUID NOT NULL,

    CONSTRAINT habit_cat_id_fk
        FOREIGN KEY (cat_id) 
            REFERENCES category(cat_id)
);