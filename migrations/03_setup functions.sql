CREATE OR REPLACE FUNCTION validate_subtopic_hierarchy()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 
        FROM document_topics dt
        JOIN sub_topics st ON st.topic_id = dt.topic_id
        WHERE dt.document_id = NEW.document_id 
        AND st.id = NEW.sub_topic_id
    ) THEN
        RAISE EXCEPTION 'Subtopic % does not belong to any of the already assigned topics to the document %', 
            NEW.sub_topic_id, NEW.document_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- El Trigger
CREATE TRIGGER trg_validate_subtopic
BEFORE INSERT OR UPDATE ON document_sub_topics
FOR EACH ROW EXECUTE FUNCTION validate_subtopic_hierarchy();