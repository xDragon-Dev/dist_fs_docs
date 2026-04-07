--* PASSED ✅👍
INSERT INTO document_topics (
    document_id,
    topic_id
) 
VALUES($1, $2);


--* PASSED ✅👍
SELECT name FROM topics;


--* PASSED ✅👍
SELECT t.name
FROM topics t
JOIN document_topics dt ON dt.topic_id = t.id
WHERE dt.document_id = $1 --ID del documento