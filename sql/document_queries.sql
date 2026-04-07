
--* PASSED ✅👍
INSERT INTO scientific_documents (
    id,
    posted_by,
    title,
    authors,
    abstract,
    keywords,
    document_type,
    publication_date,
    language
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);


--* PASSED ✅👍
SELECT 
    id, 
    posted_by, 
    title, 
    authors, 
    abstract,
    keywords,
    document_type,
    publication_date, 
    language
FROM scientific_documents
WHERE id = $1;


--! PASSED BUT IGNORED ❌👎
SELECT 
    d.id, 
    d.posted_by, 
    d.title, 
    d.authors, 
    d.abstract,
    d.keywords,
    d.document_type,
    d.publication_date, 
    d.language,
    COALESCE((
        SELECT jsonb_agg(t.name)
        FROM topics t
        JOIN document_topics dt ON dt.topic_id = t.id
        WHERE dt.document_id = d.id
    ), '[]') as topics,
    COALESCE((
        SELECT jsonb_agg(st.name)
        FROM sub_topics st
        JOIN document_sub_topics dst ON dst.sub_topic_id = st.id
        WHERE dst.document_id = d.id
    ), '[]') as sub_topics
FROM scientific_documents d
WHERE d.id = $1;