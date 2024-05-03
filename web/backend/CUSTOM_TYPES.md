# Custom Types
Here we define a list of SQL custom types that could/should be implemented for each table in the migrations. This markdown has been created after the discovery of custom types in SQL and how to convert them to Rust using Diesel (see [this repo](https://github.com/mvisani/sql_custom_types)).

## Tables
- [ ] `users` column `first_name`, `last_name` and `middle_name` should not be empty strings.
- [ ] `user_emails` column `email` should be a valid email.
- [ ] Luca said we shoudl also have a type for Path
- [ ] Should all small enums be custom types? (For example sample states could be a custom enum type instead of populating the table with our csv file)
- [ ] 