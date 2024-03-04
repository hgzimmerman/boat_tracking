-- Your SQL goes here

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


CREATE TABLE use_event (
    id INTEGER PRIMARY KEY ASC NOT NULL,
    boat_id INTEGER NOT NULL,
    recorded_at DATETIME NOT NULL, 
    use_scenario TEXT CHECK( use_scenario IN ('AM', 'PM', 'Regatta', 'Other') ) NOT NULL,
    note TEXT,
    FOREIGN KEY (boat_id) REFERENCES boat(id)
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