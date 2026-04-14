INSERT INTO users (username_hash, password_hash, user_role)
VALUES ('Admin', '12345678','Admin');

INSERT INTO topics (name)
VALUES
    ('Health and Life Sciences'), -- Ciencias de la Salud y Vida 
    ('Exact and Natural Sciences'), -- Ciencias Exactas y Naturales 
    ('Engineering and Technology'), -- Ingeniería y Tecnología 
    ('Social Sciences and Humanities'), -- Ciencias Sociales y Humanidades 
    ('Agricultural and Environmental Sciences'), -- Ciencias Agrarias y Ambientales 
    ('General'); --General

INSERT INTO sub_topics (topic_id, name)
VALUES
    -- CIENCIAS DE LA SALUD Y VIDA

    (1, 'Clinical Medicine'), -- Medicina Clínica 
    (1, 'Genetics and Genomics'), -- Genética y Genómica 
    (1, 'Neurosciences'), -- Neurociencias 
    (1, 'Pharmacology'), -- Farmacología 
    (1, 'Immunology'), -- Inmunología 
    (1, 'Oncology'), -- Oncología 
    (1, 'Molecular Biology'), -- Biología Molecular 
    (1, 'Biotechnology'), -- Biotecnología 
    (1, 'Microbiology'), -- Microbiología 
    (1, 'Virology'), -- Virología 
    (1, 'Epidemiology'), -- Epidemiología 
    (1, 'Public Health'), -- Salud Pública 
    (1, 'Nutrition and Dietetics'), -- Nutrición y Dietética 
    (1, 'Dentistry'), -- Odontología 
    (1, 'Nursing'), -- Enfermería 
    (1, 'Veterinary Medicine'), -- Veterinaria 
    (1, 'Botany'), -- Botánica 
    (1, 'Zoology'), -- Zoología 
    (1, 'Ecology'), -- Ecología 
    (1, 'Biomedicine'), -- Biomedicina 
    (1, 'Marine Biology'), -- Biología Marina

    -- CIENCIAS EXACTAS Y NATURALES 

    (2, 'Particle Physics'), -- Física de Partículas 
    (2, 'Astrophysics'), -- Astrofísica 
    (2, 'Cosmology'), -- Cosmología 
    (2, 'Organic Chemistry'), -- Química Orgánica 
    (2, 'Inorganic Chemistry'), -- Química Inorgánica 
    (2, 'Biochemistry'), -- Bioquímica 
    (2, 'Pure Mathematics'), -- Matemáticas Puras 
    (2, 'Statistics'), -- Estadística 
    (2, 'Geology'), -- Geología 
    (2, 'Paleontology'), -- Paleontología 
    (2, 'Meteorology'), -- Meteorología 
    (2, 'Oceanography'), -- Oceanografía 
    (2, 'Crystallography'), -- Cristalografía 
    (2, 'Thermodynamics'), -- Termodinámica 
    (2, 'Optics'), -- Óptica 
    (2, 'Quantum Mechanics'), -- Mecánica Cuántica 
    (2, 'Electromagnetism'), -- Electromagnetismo 

    -- INGENIERÍA Y TECNOLOGÍA 

    (3, 'Artificial Intelligence'), -- Inteligencia Artificial 
    (3, 'Robotics'), -- Robótica 
    (3, 'Civil Engineering'), -- Ingeniería Civil 
    (3, 'Mechanical Engineering'), -- Ingeniería Mecánica 
    (3, 'Electrical Engineering'), -- Ingeniería Eléctrica 
    (3, 'Nanotechnology'), -- Nanotecnología 
    (3, 'Materials Science'), -- Ciencia de Materiales 
    (3, 'Cybersecurity'), -- Ciberseguridad 
    (3, 'Telecommunications'), -- Telecomunicaciones 
    (3, 'Aerospace Engineering'), -- Ingeniería Aeroespacial 
    (3, 'Nuclear Energy'), -- Energía Nuclear 
    (3, 'Renewable Energy'), -- Energías Renovables 
    (3, 'Bioengineering'), -- Bioingeniería 
    (3, 'Quantum Computing'), -- Computación Cuántica 
    (3, 'Chemical Engineering'), -- Ingeniería Química 
    (3, 'Metallurgy'), -- Metalurgia 
    (3, 'Architecture and Urbanism'), -- Arquitectura y Urbanismo 

    -- CIENCIAS SOCIALES Y HUMANIDADES 

    (4, 'Cognitive Psychology'), -- Psicología Cognitiva 
    (4, 'Sociology'), -- Sociología 
    (4, 'Anthropology'), -- Antropología 
    (4, 'Economics and Finance'), -- Economía y Finanzas 
    (4, 'Political Science'), -- Ciencias Políticas 
    (4, 'Law and Jurisprudence'), -- Derecho y Jurisprudencia 
    (4, 'Archaeology'), -- Arqueología 
    (4, 'World History'), -- Historia Universal 
    (4, 'Linguistics'), -- Lingüística 
    (4, 'Philosophy'), -- Filosofía 
    (4, 'Pedagogy and Education'), -- Pedagogía y Educación 
    (4, 'Human Geography'), -- Geografía Humana 
    (4, 'Criminology'), -- Criminología 
    (4, 'Social Work'), -- Trabajo Social 
    (4, 'Communication and Media'), -- Comunicación y Medios 
    (4, 'International Relations'), -- Relaciones Internacionales 
    (4, 'Business Administration'), -- Administración de Empresas 

    -- CIENCIAS AGRARIAS Y AMBIENTALES

    (5, 'Agronomy'), -- Agronomía 
    (5, 'Forestry'), -- Silvicultura (Ciencias Forestales) 
    (5, 'Soil Science'), -- Ciencia del Suelo (Edafología) 
    (5, 'Hydrology'), -- Hidrología 
    (5, 'Environmental Sciences'), -- Ciencias Ambientales 
    (5, 'Waste Management'), -- Gestión de Residuos 
    (5, 'Climate Change'), -- Cambio Climático 
    (5, 'Aquaculture'), -- Acuicultura 
    (5, 'Food Security'), -- Seguridad Alimentaria 

    -- GENERAL
    (6, 'General');