--Creates all related tables in the database
CREATE TABLE  users (
    user TEXT PRIMARY KEY, 
    password_hash TEXT NOT NULL,
    role role NOT NULL, -- DEFAULT 'User'
    tokens_valid_after BIGINT NOT NULL
);

CREATE TABLE topics (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_by TEXT REFERENCES users(user),
    scope scope NOT NULL,

    CONSTRAINT unique_topic_per_user UNIQUE NULLS NOT DISTINCT (name, created_by, scope)
);

CREATE TABLE sub_topics (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_by TEXT REFERENCES users(user),
    scope scope NOT NULL,

    CONSTRAINT unique_sub_topic_per_user UNIQUE NULLS NOT DISTINCT (name, created_by, scope)
);

CREATE TABLE scientific_documents (
    id UUID PRIMARY KEY, -- DEFAULT gen_random_uuid()
    posted_by TEXT NOT NULL REFERENCES users(user),
    title TEXT NOT NULL,
    authors TEXT[] NOT NULL,
    abstract TEXT NOT NULL,
    keywords TEXT[] NOT NULL,
    document_type document_type NOT NULL,
    publication_date TIMESTAMPTZ NOT NULL, -- DEFAULT CURRENT_TIMESTAMP
    language TEXT NOT NULL
);

CREATE TABLE document_topics (
    document_id UUID REFERENCES scientific_documents(id) ON DELETE CASCADE,
    topic_id INT REFERENCES topics(id) ON DELETE CASCADE,
    PRIMARY KEY (document_id, topic_id)
);

CREATE TABLE document_sub_topics (
    document_id UUID REFERENCES scientific_documents(id) ON DELETE CASCADE,
    sub_topic_id INT REFERENCES sub_topics(id) ON DELETE CASCADE,
    PRIMARY KEY (document_id, sub_topic_id)
);

CREATE TABLE metadata_nodes (
    id UUID PRIMARY KEY,
    ip INET NOT NULL,
    port INT NOT NULL,
    node_status status NOT NULL,
    last_heartbeat TIMESTAMPTZ NOT NULL -- DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE storage_nodes (
    id UUID PRIMARY KEY,
    ip INET NOT NULL,
    port INT NOT NULL,
    node_status status NOT NULL,
    last_heartbeat TIMESTAMPTZ NOT NULL -- DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE document_storage_nodes (
    document_id UUID REFERENCES scientific_documents(id) ON DELETE CASCADE,
    storage_node_id UUID REFERENCES storage_nodes(id) ON DELETE CASCADE, 
    is_verified BOOLEAN NOT NULL, -- DEFAULT false
    content_hash TEXT NOT NULL,
    PRIMARY KEY (document_id, storage_node_id)
);

CREATE TABLE operations (
    id UUID PRIMARY KEY, -- DEFAULT gen_random_uuid()
    kind kind NOT NULL,
    executant TEXT REFERENCES users(user)
)

CREATE TABLE failed_deletions (
    storage_node_id UUID REFERENCES storage_nodes(id) ON DELETE CASCADE
    files UUID[] NOT NULL
)