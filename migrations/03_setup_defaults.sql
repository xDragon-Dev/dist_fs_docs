INSERT INTO users (name, password_hash, role, tokens_valid_after)
--password = 12345678
VALUES ('Admin', '$argon2id$v=19$m=19456,t=2,p=1$oOZzP3BDke48Q8c+GXdaaQ$4kvCXNWylP4+dkGvlzDbDik6aa2zYZbMCIbdsb0SPJM','Admin',0);

INSERT INTO topics (name, scope)
VALUES
    ('Health and Life Sciences','Global'), -- Ciencias de la Salud y Vida 
    ('Exact and Natural Sciences','Global'), -- Ciencias Exactas y Naturales 
    ('Engineering and Technology','Global'), -- Ingeniería y Tecnología 
    ('Social Sciences and Humanities','Global'), -- Ciencias Sociales y Humanidades 
    ('Agricultural and Environmental Sciences','Global'), -- Ciencias Agrarias y Ambientales 
    ('General','Global'); --General

INSERT INTO sub_topics (name, scope)
VALUES
    -- CIENCIAS DE LA SALUD Y VIDA

    ('Clinical Medicine','Global'), -- Medicina Clínica 
    ('Genetics and Genomics','Global'), -- Genética y Genómica 
    ('Neurosciences','Global'), -- Neurociencias 
    ('Pharmacology','Global'), -- Farmacología 
    ('Immunology','Global'), -- Inmunología 
    ('Oncology','Global'), -- Oncología 
    ('Molecular Biology','Global'), -- Biología Molecular 
    ('Biotechnology','Global'), -- Biotecnología 
    ('Microbiology','Global'), -- Microbiología 
    ('Virology','Global'), -- Virología 
    ('Epidemiology','Global'), -- Epidemiología 
    ('Public Health','Global'), -- Salud Pública 
    ('Nutrition and Dietetics','Global'), -- Nutrición y Dietética 
    ('Dentistry','Global'), -- Odontología 
    ('Nursing','Global'), -- Enfermería 
    ('Veterinary Medicine','Global'), -- Veterinaria 
    ('Botany','Global'), -- Botánica 
    ('Zoology','Global'), -- Zoología 
    ('Ecology','Global'), -- Ecología 
    ('Biomedicine','Global'), -- Biomedicina 
    ('Marine Biology','Global'), -- Biología Marina

    -- CIENCIAS EXACTAS Y NATURALES 

    ('Particle Physics','Global'), -- Física de Partículas 
    ('Astrophysics','Global'), -- Astrofísica 
    ('Cosmology','Global'), -- Cosmología 
    ('Organic Chemistry','Global'), -- Química Orgánica 
    ('Inorganic Chemistry','Global'), -- Química Inorgánica 
    ('Biochemistry','Global'), -- Bioquímica 
    ('Pure Mathematics','Global'), -- Matemáticas Puras 
    ('Statistics','Global'), -- Estadística 
    ('Geology','Global'), -- Geología 
    ('Paleontology','Global'), -- Paleontología 
    ('Meteorology','Global'), -- Meteorología 
    ('Oceanography','Global'), -- Oceanografía 
    ('Crystallography','Global'), -- Cristalografía 
    ('Thermodynamics','Global'), -- Termodinámica 
    ('Optics','Global'), -- Óptica 
    ('Quantum Mechanics','Global'), -- Mecánica Cuántica 
    ('Electromagnetism','Global'), -- Electromagnetismo 

    -- INGENIERÍA Y TECNOLOGÍA 

    ('Artificial Intelligence','Global'), -- Inteligencia Artificial 
    ('Robotics','Global'), -- Robótica 
    ('Civil Engineering','Global'), -- Ingeniería Civil 
    ('Mechanical Engineering','Global'), -- Ingeniería Mecánica 
    ('Electrical Engineering','Global'), -- Ingeniería Eléctrica 
    ('Nanotechnology','Global'), -- Nanotecnología 
    ('Materials Science','Global'), -- Ciencia de Materiales 
    ('Cybersecurity','Global'), -- Ciberseguridad 
    ('Telecommunications','Global'), -- Telecomunicaciones 
    ('Aerospace Engineering','Global'), -- Ingeniería Aeroespacial 
    ('Nuclear Energy','Global'), -- Energía Nuclear 
    ('Renewable Energy','Global'), -- Energías Renovables 
    ('Bioengineering','Global'), -- Bioingeniería 
    ('Quantum Computing','Global'), -- Computación Cuántica 
    ('Chemical Engineering','Global'), -- Ingeniería Química 
    ('Metallurgy','Global'), -- Metalurgia 
    ('Architecture and Urbanism','Global'), -- Arquitectura y Urbanismo 

    -- CIENCIAS SOCIALES Y HUMANIDADES 

    ('Cognitive Psychology','Global'), -- Psicología Cognitiva 
    ('Sociology','Global'), -- Sociología 
    ('Anthropology','Global'), -- Antropología 
    ('Economics and Finance','Global'), -- Economía y Finanzas 
    ('Political Science','Global'), -- Ciencias Políticas 
    ('Law and Jurisprudence','Global'), -- Derecho y Jurisprudencia 
    ('Archaeology','Global'), -- Arqueología 
    ('World History','Global'), -- Historia Universal 
    ('Linguistics','Global'), -- Lingüística 
    ('Philosophy','Global'), -- Filosofía 
    ('Pedagogy and Education','Global'), -- Pedagogía y Educación 
    ('Human Geography','Global'), -- Geografía Humana 
    ('Criminology','Global'), -- Criminología 
    ('Social Work','Global'), -- Trabajo Social 
    ('Communication and Media','Global'), -- Comunicación y Medios 
    ('International Relations','Global'), -- Relaciones Internacionales 
    ('Business Administration','Global'), -- Administración de Empresas 

    -- CIENCIAS AGRARIAS Y AMBIENTALES

    ('Agronomy','Global'), -- Agronomía 
    ('Forestry','Global'), -- Silvicultura (Ciencias Forestales) 
    ('Soil Science','Global'), -- Ciencia del Suelo (Edafología) 
    ('Hydrology','Global'), -- Hidrología 
    ('Environmental Sciences','Global'), -- Ciencias Ambientales 
    ('Waste Management','Global'), -- Gestión de Residuos 
    ('Climate Change','Global'), -- Cambio Climático 
    ('Aquaculture','Global'), -- Acuicultura 
    ('Food Security','Global'), -- Seguridad Alimentaria 

    -- GENERAL
    ('General','Global');