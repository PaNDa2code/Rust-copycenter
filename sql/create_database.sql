DROP DATABASE IF EXISTS CopyCenter;

CREATE DATABASE CopyCenter;

\c copycenter


CREATE TYPE file_type AS ENUM ('pdf', 'word', 'image', 'other');
CREATE TYPE jop_type AS ENUM ('printing', 'copying');
CREATE TYPE sides AS ENUM ('one-side', 'two-sides');
CREATE TYPE printing_quality AS ENUM ('standard', 'high-quality');
CREATE TYPE paper_wight AS ENUM ('70g', '80g');

CREATE TABLE customers (
  customer_id SERIAL PRIMARY KEY,
  customer_name varchar(40),
  customer_phone_number varchar(11)
);

CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_full_name varchar(40),
  user_name varchar(40),
  passhash varchar(40)
);

CREATE TABLE files (
  file_id SERIAL PRIMARY KEY,
  file_type file_type,
  file_checksum_SHA_264 varchar(64),
  file_name varchar(40),
  file_dir varchar(50),
  file_pages_count int
);

CREATE TABLE jobs (
  jop_id SERIAL PRIMARY KEY,
  customer_id int NOT NULL,
  user_id int NOT NULL,
  jop_added_at_time timestamp NOT NULL DEFAULT now(),
  jop_done_at_time timestamp,
  jop_type jop_type DEFAULT 'printing',
  jop_done bool DEFAULT false,
  file_id int,
  pages_per_sheet int,
  paper_wight paper_wight NOT NULL DEFAULT '70g',
  copies_count int NOT NULL,
  paper_count int NOT NULL,
  sides sides NOT NULL DEFAULT 'one-side',
  plank_back_cover bool NOT NULL DEFAULT false,
  printing_quality printing_quality NOT NULL DEFAULT 'standard',

  CONSTRAINT file_info_required CHECK (
    (jop_type = 'printing' AND file_id IS NOT NULL)
    OR jop_type != 'printing'
  )
);


ALTER TABLE jobs ADD FOREIGN KEY (user_id) REFERENCES users (user_id);

ALTER TABLE jobs ADD FOREIGN KEY (file_id) REFERENCES files (file_id);

ALTER TABLE jobs ADD FOREIGN KEY (customer_id) REFERENCES customers (customer_id);
