INSERT INTO users (user_name, password_hash, user_role, tokens_valid_after)
VALUES 
    ('Juanito','12345678','User', 0),
    ('Pedrito','12345678','User', 0),
    ('Maria','12345678','User', 0);


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

INSERT INTO operation_ids VALUES 
('92390b1d-e4fb-4d4e-80f3-a0d5c780a505');

/*
FREE TO USE UUID FOR TESTING

b0447da8-5d55-4fea-96d5-ef47a3b331da
0168917a-1192-4f27-b1cb-6b2ea6eb79ea
832351b5-4ffa-454f-a36e-2d369b45edd4
62c71a3c-6447-451c-9d5d-877ddf1479f5
4367275d-70b7-428c-b363-5d68baec31e4
4d62ad15-1f15-4bc7-bc17-36dcb47d5ee9
c251d2a1-29f2-43ec-802f-2dcbec06b361
*/

INSERT INTO metadata_nodes(id, ip, port, node_status, last_heartbeat)
VALUES 
('15ece130-2784-4932-8bb1-9887f7046b46', '::1', 31415, 'Active', CURRENT_TIMESTAMP);

INSERT INTO storage_nodes(id, ip, port, node_status, last_heartbeat)
VALUES 
('6e56f659-2537-4746-a20e-4485df339931', '::1', 31416, 'Active', CURRENT_TIMESTAMP);


INSERT INTO document_storage_nodes(document_id,storage_node_id,is_verified,content_hash)
VALUES
('0e010c9e-217c-43e3-8fbf-50b64e2865c5','6e56f659-2537-4746-a20e-4485df339931','true','CONTENT HASH NO USADO XD, CONSIDERAR ELIMINAR'),
('a6c8e2b9-501c-4535-873a-ffea5f9dda9a','6e56f659-2537-4746-a20e-4485df339931','true','CONTENT HASH NO USADO XD, CONSIDERAR ELIMINAR'),
('92390b1d-e4fb-4d4e-80f3-a0d5c780a505','6e56f659-2537-4746-a20e-4485df339931','true','CONTENT HASH NO USADO XD, CONSIDERAR ELIMINAR'),
('18a90c6a-e8ba-4cbb-9a61-8d7f42db9f8e','6e56f659-2537-4746-a20e-4485df339931','true','CONTENT HASH NO USADO XD, CONSIDERAR ELIMINAR'),
('da507d7d-8d65-4933-aa7c-7210961906b7','6e56f659-2537-4746-a20e-4485df339931','true','CONTENT HASH NO USADO XD, CONSIDERAR ELIMINAR'),
('61815cc3-1e51-47af-82df-110ef9c98e26','6e56f659-2537-4746-a20e-4485df339931','true','CONTENT HASH NO USADO XD, CONSIDERAR ELIMINAR');