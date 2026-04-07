--* PASSED ✅👍
INSERT INTO document_sub_topics (
    document_id,
    sub_topic_id
)
VALUES($1, $2):


--* PASSED ✅👍
SELECT name FROM sub_topics;


--* PASSED ✅👍
SELECT name FROM sub_topics WHERE topic_id = $1;


--* PASSED ✅👍
SELECT st.name
FROM sub_topics st
JOIN document_sub_topics dst ON dst.sub_topic_id = st.id
WHERE dst.document_id = $1; -- ID del documento