CREATE TABLE habit (
    hab_id UUID PRIMARY KEY,
    hab_name VARCHAR(255) NOT NULL,
    hab_description VARCHAR(255) NOT NULL,
    hab_created_at TIMESTAMP NOT NULL,
    hab_updated_at TIMESTAMP NOT NULL,
    hab_type VARCHAR(10) NOT NULL,
    hab_is_favorite BOOLEAN NOT NULL,
    user_id UUID NOT NULL UNIQUE
);