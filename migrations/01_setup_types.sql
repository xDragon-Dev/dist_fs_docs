CREATE TYPE role AS ENUM ('User', 'Admin');

CREATE TYPE document_type AS ENUM (
    'OriginalArticle', 
    'Review', 
    'CaseReport', 
    'Letter', 
    'Editorial', 
    'ConferencePaper', 
    'Thesis'
);

CREATE TYPE status AS ENUM (
    'Active', 
    'Offline'
);

CREATE TYPE scope AS ENUM (
    'Local',
    'Global'
)