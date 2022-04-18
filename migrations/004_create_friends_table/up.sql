CREATE TABLE friends (
    id SERIAL PRIMARY KEY NOT NULL,
    user_from_id BIGINT NOT NULL REFERENCES users(id),
    user_to_id BIGINT NOT NULL REFERENCES users(id),
    status VARCHAR NOT NULL,
    update_timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);
