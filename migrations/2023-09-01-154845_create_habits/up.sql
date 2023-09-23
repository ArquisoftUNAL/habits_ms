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

    -- Save next time this habit frequency will restart
    hab_next_closure_date DATE NOT NULL,
    
    usr_id VARCHAR(24) NOT NULL,
    cat_id UUID NOT NULL,

    CONSTRAINT habit_cat_id_fk
        FOREIGN KEY (cat_id) 
            REFERENCES category(cat_id)
);

CREATE FUNCTION get_next_closure_date(
    freq_type hab_freq_type_enum,
    prev_closure_date DATE
) 
    RETURNS DATE
    LANGUAGE plpgsql
    AS
    $$
    BEGIN
        CASE freq_type
            WHEN 'daily' THEN
                RETURN prev_closure_date + INTERVAL '1 day';
            WHEN 'daily2' THEN
                RETURN prev_closure_date + INTERVAL '2 day';
            WHEN 'weekly' THEN
                RETURN prev_closure_date + INTERVAL '1 week';
            WHEN 'weekly2' THEN
                RETURN prev_closure_date + INTERVAL '2 week';
            WHEN 'monthly' THEN
                RETURN prev_closure_date + INTERVAL '1 month';
            WHEN 'monthly2' THEN
                RETURN prev_closure_date + INTERVAL '2 month';
        END CASE;
    END;
    $$