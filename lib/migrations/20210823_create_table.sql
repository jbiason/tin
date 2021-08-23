-- Projects
CREATE TABLE project (
	id INTEGER NOT NULL PRIMARY KEY,
	name TEXT NOT NULL
);
CREATE UNIQUE INDEX project_name ON project (name);

CREATE TABLE tag (
	id INTEGER NOT NULL PRIMARY KEY,
	label TEXT NOT NULL
)
CREATE UNIQUE INDEX tag_label ON tag (label);

CREATE TABLE entry (
	id INTEGER NOT NULL PRIMARY KEY,
	start_ts DATETIME NOT NULL,
	stop_ts DATETIME,
	project_id INTEGER NOT NULL,
	FOREIGN KEY (project_id) REFERENCES project (id)
);

CREATE TABLE entry_tags (
	id INTEGER NOT NULL PRIMARY KEY,
	entry_id INTEGER NOT NULL,
	tag_id INTEGER NOT NULL,
	CONSTRAINT UNIQUE (entry_id, tag_id),
	FOREIGN KEY (entry_id) REFERENCES entry (id),
	FOREIGN KEY (tag_id) REFERENCES tag (id)
)
