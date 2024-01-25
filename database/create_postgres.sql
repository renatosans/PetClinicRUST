CREATE DATABASE pet_clinic;
\c pet_clinic


CREATE TABLE IF NOT EXISTS public.appointment (
    id SERIAL,
    date date NOT NULL,
    veterinarian integer NOT NULL,
    petowner integer NOT NULL
);

CREATE TABLE IF NOT EXISTS public.pet (
    id SERIAL,
    name character varying(120) NOT NULL,
    breed character varying(45) NOT NULL,
    age integer,
    owner integer,
	flag_removed bool NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.petowner (
    id SERIAL,
    name character varying(120) NOT NULL,
    birth_date date,
    email character varying(80) NOT NULL,
    phone character varying(45),
    address character varying(120) NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE pet
    add constraint fk_pet_owner
        foreign key (owner) references petowner (id);

CREATE TABLE IF NOT EXISTS public.treatment (
    id SERIAL,
    description character varying(120) NOT NULL,
    pet integer NOT NULL,
    veterinarian integer NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE treatment
    add constraint fk_pet_treatment
        foreign key (pet) references pet (id);

CREATE TABLE IF NOT EXISTS public.vaccination (
    id SERIAL,
    description character varying(120) NOT NULL,
    pet integer NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE vaccination
    add constraint fk_pet_vaccination
        foreign key (pet) references pet (id);

CREATE TABLE IF NOT EXISTS public.veterinarian (
    id SERIAL,
    name character varying(120) NOT NULL,
    "inscricaoCRMV" character varying(75) NOT NULL,
    PRIMARY KEY (id)
);

--
-- Data for table pet
--
INSERT INTO pet ("name", "breed", "age", "owner", "flag_removed") VALUES
('Bethoven', 'Saint Bernard', 7, NULL, false),
('Molly', 'Golden Retriever', 4, NULL, false),
('Yoshi', 'Shiba Inu', 2, NULL, false),
('Thor', 'Beagle', 9, NULL, false);

--
-- Data for table veterinarian
--

INSERT INTO veterinarian ("name", "inscricaoCRMV") VALUES
('CINTIA', 'SP 5091234567890'),
('GASPAR', 'SP 5141234567890'),
('JOSE', 'SP 5161234567890'),
('ANA CLAUDIA', 'SP 5191234567890');
