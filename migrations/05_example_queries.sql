INSERT INTO users (username_hash, password_hash, user_role)
VALUES 
    ('Juanito','12345678','User'),
    ('Pedrito','12345678','User'),
    ('Maria','12345678','User');


INSERT INTO scientific_documents (id, posted_by, title, authors, abstract, keywords, document_type, publication_date, language)
VALUES
    (
        '0e010c9e-217c-43e3-8fbf-50b64e2865c5', 
        'Juanito', 
        'Titulo de documento 1', 
        ARRAY['Lorem', 'ipsum', 'dolor', 'sit', 'amet', 'consectetur'], 
        'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Laboriosam asperiores non nihil omnis quia, nostrum.', 
        ARRAY['adipisicing', 'elit', 'Ex', 'corrupti', 'consectetur'], 
        'OriginalArticle', 
        CURRENT_TIMESTAMP, 
        'Spanish'
    ),
    (
        'a6c8e2b9-501c-4535-873a-ffea5f9dda9a', 
        'Juanito', 
        'Titulo de documento 2', 
        ARRAY['dolor', 'nesciunt', 'hic', 'itaque', 'possimus', 'rerum'], 
        'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Laboriosam asperiores non nihil omnis quia, nostrum.', 
        ARRAY['autem', 'iusto', 'ut', 'impedit', 'optio', 'totam'], 
        'Review', 
        CURRENT_TIMESTAMP, 
        'Spanish'
    ),
    (
        '92390b1d-e4fb-4d4e-80f3-a0d5c780a505', 
        'Pedrito', 
        'Titulo de documento 3', 
        ARRAY['laboriosam', 'accusantium', 'Excepturi', 'laudantium'], 
        'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Laboriosam asperiores non nihil omnis quia, nostrum.', 
        ARRAY['Nam', 'quibusdam', 'itaque'], 
        'CaseReport', 
        CURRENT_TIMESTAMP, 
        'Spanish'
    ),
    (
        '18a90c6a-e8ba-4cbb-9a61-8d7f42db9f8e', 
        'Pedrito', 
        'Titulo de documento 4', 
        ARRAY['Nam', 'quibusdam', 'itaque'], 
        'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Laboriosam asperiores non nihil omnis quia, nostrum.', 
        ARRAY['Lorem', 'ipsum', 'dolor', 'sit', 'amet', 'consectetur'], 
        'Letter', 
        CURRENT_TIMESTAMP, 
        'Spanish'
    ),
    (
        'da507d7d-8d65-4933-aa7c-7210961906b7', 
        'Maria', 
        'Titulo de documento 5', 
        ARRAY['laboriosam', 'accusantium', 'Excepturi', 'laudantium'], 
        'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Laboriosam asperiores non nihil omnis quia, nostrum.', 
        ARRAY['adipisicing', 'elit', 'Ex', 'corrupti', 'consectetur'], 
        'ConferencePaper', 
        CURRENT_TIMESTAMP, 
        'Spanish'
    ),
    (
        '61815cc3-1e51-47af-82df-110ef9c98e26', 
        'Maria', 
        'Titulo de documento 6', 
        ARRAY['autem', 'iusto', 'ut', 'impedit', 'optio', 'totam'], 
        'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Laboriosam asperiores non nihil omnis quia, nostrum.', 
        ARRAY['dolor', 'nesciunt', 'hic', 'itaque', 'possimus', 'rerum'], 
        'Thesis', 
        CURRENT_TIMESTAMP, 
        'Spanish'
    );

INSERT INTO document_topics (document_id, topic_id)
VALUES
    ('0e010c9e-217c-43e3-8fbf-50b64e2865c5', 1),
    ('a6c8e2b9-501c-4535-873a-ffea5f9dda9a', 2),
    ('92390b1d-e4fb-4d4e-80f3-a0d5c780a505', 3),
    ('18a90c6a-e8ba-4cbb-9a61-8d7f42db9f8e', 4),
    ('da507d7d-8d65-4933-aa7c-7210961906b7', 5),
    ('61815cc3-1e51-47af-82df-110ef9c98e26', 6);


INSERT INTO document_sub_topics (document_id, sub_topic_id)
VALUES
    ('0e010c9e-217c-43e3-8fbf-50b64e2865c5', 1),
    ('0e010c9e-217c-43e3-8fbf-50b64e2865c5', 2),
    ('0e010c9e-217c-43e3-8fbf-50b64e2865c5', 3),

    ('a6c8e2b9-501c-4535-873a-ffea5f9dda9a', 22),
    ('a6c8e2b9-501c-4535-873a-ffea5f9dda9a', 23),
    ('a6c8e2b9-501c-4535-873a-ffea5f9dda9a', 24),

    ('92390b1d-e4fb-4d4e-80f3-a0d5c780a505', 39),
    ('92390b1d-e4fb-4d4e-80f3-a0d5c780a505', 40),
    ('92390b1d-e4fb-4d4e-80f3-a0d5c780a505', 41),

    ('18a90c6a-e8ba-4cbb-9a61-8d7f42db9f8e', 56),
    ('18a90c6a-e8ba-4cbb-9a61-8d7f42db9f8e', 57),
    ('18a90c6a-e8ba-4cbb-9a61-8d7f42db9f8e', 58),

    ('da507d7d-8d65-4933-aa7c-7210961906b7', 73),
    ('da507d7d-8d65-4933-aa7c-7210961906b7', 74),
    ('da507d7d-8d65-4933-aa7c-7210961906b7', 75),

    ('61815cc3-1e51-47af-82df-110ef9c98e26', 82);