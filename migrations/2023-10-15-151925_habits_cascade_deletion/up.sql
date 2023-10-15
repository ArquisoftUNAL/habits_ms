-- Make habits table delete on cascade habitsdata
ALTER TABLE habit_data_collected
    DROP CONSTRAINT IF EXISTS habit_data_collected_hab_id_fk,
    ADD CONSTRAINT habit_data_collected_hab_id_fk
        FOREIGN KEY (hab_id)
            REFERENCES habit(hab_id)
            ON DELETE CASCADE;
