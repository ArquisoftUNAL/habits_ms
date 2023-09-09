

CREATE TYPE rec_data_type AS ENUM('DA', 'WE', '2W', 'MO', '2M' );

CREATE TABLE habit_recurrency (
    hab_rec_id UUID PRIMARY KEY,
    hab_rec_freq_type rec_data_type NOT NULL,
    hab_rec_freq_data DATE NOT NULL,
    hab_id UUID NOT NULL,

    CONSTRAINT habit_recurrency_hab_id_fk 
        FOREIGN KEY (hab_id) 
            REFERENCES habit(hab_id)           
);