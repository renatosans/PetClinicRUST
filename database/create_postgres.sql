--
-- PostgreSQL database dump
--

-- Dumped from database version 16.0 (Debian 16.0-1.pgdg120+1)
-- Dumped by pg_dump version 16.0 (Debian 16.0-1.pgdg120+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: pet; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.pet (
    id integer NOT NULL,
    name character varying(120) NOT NULL,
    breed character varying(45) NOT NULL,
    age integer
);


ALTER TABLE public.pet OWNER TO postgres;

--
-- Name: pet_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.pet_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.pet_id_seq OWNER TO postgres;

--
-- Name: pet_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.pet_id_seq OWNED BY public.pet.id;


--
-- Name: petowner; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.petowner (
    id integer NOT NULL,
    name character varying(120) NOT NULL,
    email character varying(45) NOT NULL,
    address character varying(45) NOT NULL
);


ALTER TABLE public.petowner OWNER TO postgres;

--
-- Name: petowner_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.petowner_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.petowner_id_seq OWNER TO postgres;

--
-- Name: petowner_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.petowner_id_seq OWNED BY public.petowner.id;


--
-- Name: treatment; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.treatment (
    id integer NOT NULL,
    description character varying(120) NOT NULL,
    pet integer NOT NULL,
    veterinarian integer NOT NULL
);


ALTER TABLE public.treatment OWNER TO postgres;

--
-- Name: treatment_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.treatment_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.treatment_id_seq OWNER TO postgres;

--
-- Name: treatment_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.treatment_id_seq OWNED BY public.treatment.id;


--
-- Name: vaccination; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.vaccination (
    id integer NOT NULL,
    description character varying(120) NOT NULL,
    pet integer NOT NULL
);


ALTER TABLE public.vaccination OWNER TO postgres;

--
-- Name: vaccination_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.vaccination_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.vaccination_id_seq OWNER TO postgres;

--
-- Name: vaccination_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.vaccination_id_seq OWNED BY public.vaccination.id;


--
-- Name: veterinarian; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.veterinarian (
    id integer NOT NULL,
    name character varying(120) NOT NULL,
    "inscricaoCRMV" character varying(75) NOT NULL
);


ALTER TABLE public.veterinarian OWNER TO postgres;

--
-- Name: veterinarian_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.veterinarian_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.veterinarian_id_seq OWNER TO postgres;

--
-- Name: veterinarian_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.veterinarian_id_seq OWNED BY public.veterinarian.id;


--
-- Name: pet id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.pet ALTER COLUMN id SET DEFAULT nextval('public.pet_id_seq'::regclass);


--
-- Name: petowner id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.petowner ALTER COLUMN id SET DEFAULT nextval('public.petowner_id_seq'::regclass);


--
-- Name: treatment id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.treatment ALTER COLUMN id SET DEFAULT nextval('public.treatment_id_seq'::regclass);


--
-- Name: vaccination id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vaccination ALTER COLUMN id SET DEFAULT nextval('public.vaccination_id_seq'::regclass);


--
-- Name: veterinarian id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.veterinarian ALTER COLUMN id SET DEFAULT nextval('public.veterinarian_id_seq'::regclass);


--
-- Data for Name: pet; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.pet (id, name, breed, age) FROM stdin;
\.


--
-- Data for Name: petowner; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.petowner (id, name, email, address) FROM stdin;
\.


--
-- Data for Name: treatment; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.treatment (id, description, pet, veterinarian) FROM stdin;
\.


--
-- Data for Name: vaccination; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.vaccination (id, description, pet) FROM stdin;
\.


--
-- Data for Name: veterinarian; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.veterinarian (id, name, "inscricaoCRMV") FROM stdin;
\.


--
-- Name: pet_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.pet_id_seq', 1, false);


--
-- Name: petowner_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.petowner_id_seq', 1, false);


--
-- Name: treatment_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.treatment_id_seq', 1, false);


--
-- Name: vaccination_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.vaccination_id_seq', 1, false);


--
-- Name: veterinarian_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.veterinarian_id_seq', 1, false);


--
-- Name: pet pet_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.pet
    ADD CONSTRAINT pet_pkey PRIMARY KEY (id);


--
-- Name: petowner petowner_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.petowner
    ADD CONSTRAINT petowner_pkey PRIMARY KEY (id);


--
-- Name: treatment treatment_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.treatment
    ADD CONSTRAINT treatment_pkey PRIMARY KEY (id);


--
-- Name: vaccination vaccination_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vaccination
    ADD CONSTRAINT vaccination_pkey PRIMARY KEY (id);


--
-- Name: veterinarian veterinarian_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.veterinarian
    ADD CONSTRAINT veterinarian_pkey PRIMARY KEY (id);


--
-- Name: fk_treatement_veterinarian_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fk_treatement_veterinarian_idx ON public.treatment USING btree (veterinarian);


--
-- Name: fk_treatment_pet_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fk_treatment_pet_idx ON public.treatment USING btree (pet);


--
-- Name: fk_vaccination_pet_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fk_vaccination_pet_idx ON public.vaccination USING btree (pet);


--
-- Name: treatment fk_treatement_veterinarian; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.treatment
    ADD CONSTRAINT fk_treatement_veterinarian FOREIGN KEY (veterinarian) REFERENCES public.veterinarian(id);


--
-- Name: treatment fk_treatment_pet; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.treatment
    ADD CONSTRAINT fk_treatment_pet FOREIGN KEY (pet) REFERENCES public.pet(id);


--
-- Name: vaccination fk_vaccination_pet; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vaccination
    ADD CONSTRAINT fk_vaccination_pet FOREIGN KEY (pet) REFERENCES public.pet(id);


--
-- PostgreSQL database dump complete
--

