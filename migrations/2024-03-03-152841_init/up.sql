PRAGMA foreign_keys = ON;

CREATE TABLE boat (
    id INTEGER PRIMARY KEY ASC NOT NULL,
    name TEXT NOT NULL,
    weight_class TEXT CHECK( weight_class IN ('Light', 'Medium', 'Heavy', 'Tubby') ) NOT NULL,
    seat_count INTEGER CHECK( seat_count IN (1, 2, 4, 8) ) NOT NULL,
    has_cox INTEGER CHECK( has_cox IN (0, 1) ) NOT NULL,
    oars_per_seat INTEGER CHECK( oars_per_seat IN (1, 2) ) NOT NULL,

    acquired_at DATE,
    manufactured_at DATE,
    relinquished_at DATE
);

CREATE TABLE use_scenario (
    id INTEGER PRIMARY KEY ASC NOT NULL,
    name TEXT NOT NULL UNIQUE,
    default_time TEXT
);

INSERT INTO use_scenario (id, name, default_time) VALUES
    (1, 'Youth GGRC Practice', NULL),
    (2, 'Youth Somerville Practice', NULL),
    (3, 'Masters AM Practice', '05:30'),
    (4, 'Masters PM Practice', '18:30'),
    (5, 'Learn To Row', NULL),
    (6, 'Sculling Saturday', NULL),
    (7, 'Private Session', NULL),
    (8, 'Regatta', NULL);

CREATE TABLE use_event_batch (
    id INTEGER PRIMARY KEY ASC NOT NULL,
    recorded_at DATETIME NOT NULL,
    use_scenario_id INTEGER NOT NULL,
    FOREIGN KEY (use_scenario_id) REFERENCES use_scenario(id)
);

CREATE TABLE use_event (
    id INTEGER PRIMARY KEY ASC NOT NULL,
    boat_id INTEGER NOT NULL,
    batch_id INTEGER,
    recorded_at DATETIME NOT NULL,
    use_scenario_id INTEGER NOT NULL,
    note TEXT,
    FOREIGN KEY (boat_id) REFERENCES boat(id),
    FOREIGN KEY (batch_id) REFERENCES use_event_batch(id),
    FOREIGN KEY (use_scenario_id) REFERENCES use_scenario(id)
);

CREATE TABLE issue (
    id INTEGER PRIMARY KEY ASC NOT NULL,
    boat_id INTEGER,
    use_event_id INTEGER,
    recorded_at DATETIME NOT NULL,
    note TEXT NOT NULL,
    resolved_at DATETIME,

    FOREIGN KEY (boat_id) REFERENCES boat(id),
    FOREIGN KEY (use_event_id) REFERENCES use_event(id)
);
