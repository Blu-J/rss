CREATE TABLE users (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    username text NOT NULL
);

CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);

INSERT INTO users (username)
    VALUES ('dragondef');

-- Scraping Sites
CREATE TABLE scraping_sites (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id text NOT NULL,
    every_seconds integer NOT NULL,
    url text NOT NULL,
    articles_sel text NOT NULL,
    title_sel text NOT NULL,
    link_sel text NOT NULL,
    site_title text NOT NULL,
    image_sel text,
    description_sel text,
    comments_sel text,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX IF NOT EXISTS scraping_sites_by_user_idx ON scraping_sites (user_id);

INSERT INTO scraping_sites (user_id, site_title, every_seconds, url, articles_sel, title_sel, link_sel, description_sel, image_sel, comments_sel)
    VALUES (1, "Old Reddit", 300, 'https://old.reddit.com', '.thing:not(.promoted)', 'a.title', 'a.title', NULL, 'a img', 'a.comments');

-- Site Tags
CREATE TABLE site_tags (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    site_id integer NOT NULL,
    tag text NOT NULL,
    UNIQUE (site_id, tag),
    FOREIGN KEY (site_id) REFERENCES scraping_sites (id)
);

CREATE INDEX IF NOT EXISTS site_tags_by_site_idx ON site_tags (site_id);

INSERT INTO site_tags (site_id, tag)
    VALUES (1, 'news');

-- Articles
CREATE TABLE articles (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    site_id text NOT NULL,
    date DATETIME NOT NULL,
    read_on DATETIME,
    title text NOT NULL,
    href text NOT NULL,
    description text,
    image_src text,
    comments_href text,
    UNIQUE (site_id, title),
    FOREIGN KEY (site_id) REFERENCES scraping_sites (id)
);

CREATE INDEX IF NOT EXISTS articles_sites_idx ON articles (site_id, date DESC);

-- Preferences
CREATE TABLE preferences (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id text NOT NULL,
    preference text NOT NULL,
    value text NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS preferences_udx ON preferences (user_id, preference);

