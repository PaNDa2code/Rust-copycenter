-- Insert data into customers table
\c copycenter 

INSERT INTO
    customers (
        customer_name, customer_phone_number
    )
VALUES ('John Doe', '01234567890'),
    ('Jane Smith', '01234567890');

-- Insert data into users table
INSERT INTO
    users (
        user_full_name, user_name, passhash
    )
VALUES (
        'Alice Wonderland', 'alice', 'hashed_password_1'
    ),
    (
        'Bob Builder', 'bob', 'hashed_password_2'
    );

-- Insert data into files table
INSERT INTO
    files (
        file_type, file_checksum_SHA_256, file_name, file_dir, file_pages_count
    )
VALUES (
        'pdf', 'checksum1', 'document.pdf', '/path/to/files', 99
    ),
    (
        'word', 'checksum2', 'report.docx', '/path/to/files', 131
    );

-- Insert data into jobs table
INSERT INTO
    jobs (
        customer_id, 
        user_id, 
        jop_added_at_time, 
        jop_done_at_time, 
        jop_type, 
        file_id, 
        pages_per_sheet, 
        paper_wight, 
        copies_count, 
        paper_count, 
        sides, 
        plank_back_cover, 
        printing_quality
    )
VALUES (
        1, 1, '2024-01-30 12:00', '2024-01-30 14:30', 'printing', 1, 1, '70g', 20, 10, 'one-side', true, 'standard'
    ),
    (
        2, 2, '2024-01-30 10:00', NULL, 'copying', NULL, 1, '80g', 3, 20, 'two-sides', false, 'high-quality'
    );