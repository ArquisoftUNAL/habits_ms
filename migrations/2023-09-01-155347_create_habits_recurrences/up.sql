CREATE TABLE habit_recurrency (
    hab_rec_id UUID PRIMARY KEY,
    hab_rec_frequency_type VARCHAR(255) NOT NULL,
    hab_rec_frequency_data TIMESTAMP NOT NULL,
    hab_id UUID NOT NULL,

    CONSTRAINT habit_recurrency_hab_id_fk 
        FOREIGN KEY (hab_id) 
            REFERENCES habit(hab_id)
);