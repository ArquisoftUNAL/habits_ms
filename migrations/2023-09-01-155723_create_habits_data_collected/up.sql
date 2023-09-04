CREATE TABLE habit_data_collected (
    hab_dat_id UUID PRIMARY KEY,
    hab_dat_amount DECIMAL(10,2) NOT NULL,
    hab_dat_collected_at TIMESTAMP NOT NULL,
    hab_rec_id UUID NOT NULL,

    CONSTRAINT habit_data_collected_hab_rec_id_fk 
        FOREIGN KEY (hab_rec_id) 
            REFERENCES habit_recurrency(hab_rec_id)
);