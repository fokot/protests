CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    login_code TEXT,
    login_code_created TIMESTAMP,
    created TIMESTAMP NOT NULL DEFAULT now()
);

ALTER TABLE users
CREATE TABLE IF NOT EXISTS region (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id INTEGER,
    FOREIGN KEY (parent_id) REFERENCES region(id)
);

CREATE TABLE IF NOT EXISTS protest (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    protest_date DATE NOT NULL,
    protest_time TIME WITHOUT TIME ZONE NOT NULL,
    location TEXT NOT NULL,
    region_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT now(),
    updated TIMESTAMP NOT NULL DEFAULT now(),
    deleted TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (region_id) REFERENCES region(id)
);

CREATE TABLE IF NOT EXISTS tag (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS protest_tag (
    protest_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (protest_id, tag_id),
    FOREIGN KEY (protest_id) REFERENCES protest(id),
    FOREIGN KEY (tag_id) REFERENCES tag(id)
);

CREATE TABLE IF NOT EXISTS notification (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    region_id INTEGER,
    days_before INTEGER,
    immediatelly BOOLEAN NOT NULL,
    created TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS notification_tag (
    notification_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (notification_id, tag_id),
    FOREIGN KEY (notification_id) REFERENCES notification(id),
    FOREIGN KEY (tag_id) REFERENCES tag(id)
);

INSERT INTO users (email, name, created) VALUES ('test@gmail.com', 'Test User', NOW());
INSERT INTO region (name) VALUES ('World');