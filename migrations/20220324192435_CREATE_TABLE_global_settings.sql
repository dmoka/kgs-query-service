CREATE TABLE global_settings(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  fuel_price decimal(12,2) NOT NULL,
  diesel_price decimal(12,2) NOT NULL
);