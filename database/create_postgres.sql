
CREATE TABLE public.pet (
    id SERIAL,
    name character varying(120) NOT NULL,
    breed character varying(45),
    age integer,
    owner integer NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE public.petowner (
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

CREATE TABLE public.treatment (
    id SERIAL,
    description character varying(120) NOT NULL,
    pet integer NOT NULL,
    veterinarian integer NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE treatment
    add constraint fk_pet_treatment
        foreign key (pet) references pet (id);

CREATE TABLE public.vaccination (
    id SERIAL,
    description character varying(120) NOT NULL,
    pet integer NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE vaccination
    add constraint fk_pet_vaccination
        foreign key (pet) references pet (id);

CREATE TABLE public.veterinarian (
    id SERIAL,
    name character varying(120) NOT NULL,
    "inscricaoCRMV" character varying(75) NOT NULL,
    PRIMARY KEY (id)
);

--
-- Data for table pet
--
INSERT INTO pet ("name", "breed", "age", "owner") VALUES
('Bethoven', 'Saint Bernard', 7, NULL),
('Molly', 'Golden Retriever', 4, NULL),
('Yoshi', 'Shiba Inu', 2, NULL),
('Thor', 'Beagle', 9, NULL);

--
-- Data for table veterinarian
--

INSERT INTO veterinarian ("name", "inscricaoCRMV") VALUES
('CAROLINA CRISTINA DE SOUZA', 'SP 28509/V'),
('VITOR CUNHA GASPAR', 'SP 28514/V'),
('PEDRO ALVES DE MENDONCA NETO', 'SP 28516/V'),
('PATRICIA MIDORE NAKADAKARI', 'SP 28519/V');
