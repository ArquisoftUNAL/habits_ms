CREATE TABLE habit_data_collected (
    hab_dat_id UUID PRIMARY KEY,
    hab_dat_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    hab_dat_collected_at DATE NOT NULL,
    hab_id UUID NOT NULL,

    --- CONSTRAINTS
    UNIQUE (hab_id, hab_dat_collected_at), -- only one record per day

    CONSTRAINT habit_data_collected_hab_id_fk 
        FOREIGN KEY (hab_id) 
            REFERENCES habit(hab_id)
);