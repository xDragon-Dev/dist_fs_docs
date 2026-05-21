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

CREATE TYPE scope AS ENUM (
    'Local',
    'Global'
);

CREATE TYPE kind AS ENUM (
    'Upload',
    'Download'
);