--Creates all related tables in the database
CREATE TABLE  users (
    username_hash TEXT PRIMARY KEY, 
    password_hash TEXT NOT NULL,
    user_role role NOT NULL DEFAULT 'User'
);

CREATE TABLE topics (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE sub_topics (
    id SERIAL PRIMARY KEY,
    topic_id INT REFERENCES topics(id) ON DELETE CASCADE, -- Un sub-tema pertenece a un tema
    name TEXT NOT NULL UNIQUE,

    CONSTRAINT sub_topics_id_topic_id_key 
        UNIQUE (id, topic_id)
);

CREATE TABLE scientific_documents (
    id UUID PRIMARY KEY, --DEFAULT gen_random_uuid()
    posted_by TEXT NOT NULL REFERENCES users(username_hash),
    title TEXT NOT NULL,
    authors TEXT[] NOT NULL,
    abstract TEXT NOT NULL,
    keywords TEXT[] NOT NULL,
    document_type document_type NOT NULL,
    publication_date TIMESTAMPTZ NOT NULL, --DEFAULT CURRENT_TIMESTAMP
    language TEXT NOT NULL
);

CREATE TABLE document_topics (
    document_id UUID REFERENCES scientific_documents(id) ON DELETE CASCADE,
    topic_id INT REFERENCES topics(id) ON DELETE CASCADE,
    PRIMARY KEY (document_id, topic_id)
);

CREATE TABLE document_sub_topics (
    document_id UUID NOT NULL,
    sub_topic_id INT NOT NULL,
    
    PRIMARY KEY (document_id, sub_topic_id)
);

CREATE TABLE metadata_nodes (
    id UUID PRIMARY KEY,
    ip INET NOT NULL,
    port INT NOT NULL DEFAULT 31416,
    node_status status NOT NULL,
    last_heartbeat TIMESTAMPTZ NOT NULL -- DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE storage_nodes (
    id UUID PRIMARY KEY,
    ip INET NOT NULL,
    port INT NOT NULL DEFAULT 31416,
    node_status status NOT NULL,
    last_heartbeat TIMESTAMPTZ NOT NULL-- DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE document_storage_nodes (
    document_id UUID REFERENCES scientific_documents(id) ON DELETE CASCADE,
    storage_node_id UUID REFERENCES storage_nodes(id) ON DELETE CASCADE, 
    is_verified BOOLEAN NOT NULL, -- DEFAULT false
    content_hash TEXT NOT NULL,
    PRIMARY KEY (document_id, storage_node_id)
);

CREATE TABLE operation_ids (
    operation UUID PRIMARY KEY DEFAULT gen_random_uuid()
)