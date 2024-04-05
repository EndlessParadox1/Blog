CREATE TABLE users (
    username VARCHAR(255) PRIMARY KEY,
    password VARCHAR(255) NOT NULL
);

CREATE TABLE topics (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    summary VARCHAR(255) NOT NULL,
    markdown TEXT NOT NULL,
    html TEXT NOT NULL,
    dateline TIMESTAMP WITH TIME ZONE NOT NULL,
    writer VARCHAR(255) NOT NULL,
    FOREIGN KEY (writer) REFERENCES users (username)
);
