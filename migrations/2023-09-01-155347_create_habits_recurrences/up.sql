

CREATE TYPE rec_data_type AS ENUM('daily', 'weekly', 'weekly2', 'monthly', 'monthly2' );

CREATE TABLE habit_recurrence (
    hab_rec_id UUID PRIMARY KEY,
    hab_rec_freq_type rec_data_type NOT NULL,
    hab_rec_goal DECIMAL(10,2) NOT NULL DEFAULT 0,
    hab_rec_freq_data DATE NOT NULL,
    hab_id UUID NOT NULL,

    CONSTRAINT habit_recurrence_hab_id_fk 
        FOREIGN KEY (hab_id) 
            REFERENCES habit(hab_id)           
);